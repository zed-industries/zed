use super::*;

impl Editor {
    pub fn undo(&mut self, _: &Undo, window: &mut Window, cx: &mut Context<Self>) {
        if self.read_only(cx) {
            return;
        }

        if let Some(transaction_id) = self.buffer.update(cx, |buffer, cx| buffer.undo(cx)) {
            if let Some((selections, _)) =
                self.selection_history.transaction(transaction_id).cloned()
            {
                self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_anchors(selections.to_vec());
                });
            } else {
                log::error!(
                    "No entry in selection_history found for undo. \
                     This may correspond to a bug where undo does not update the selection. \
                     If this is occurring, please add details to \
                     https://github.com/zed-industries/zed/issues/22692"
                );
            }
            self.request_autoscroll(Autoscroll::fit(), cx);
            self.unmark_text(window, cx);
            self.refresh_edit_prediction(true, false, window, cx);
            cx.emit(EditorEvent::Edited { transaction_id });
            cx.emit(EditorEvent::TransactionUndone { transaction_id });
        }
    }

    pub fn redo(&mut self, _: &Redo, window: &mut Window, cx: &mut Context<Self>) {
        if self.read_only(cx) {
            return;
        }

        if let Some(transaction_id) = self.buffer.update(cx, |buffer, cx| buffer.redo(cx)) {
            if let Some((_, Some(selections))) =
                self.selection_history.transaction(transaction_id).cloned()
            {
                self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_anchors(selections.to_vec());
                });
            } else {
                log::error!(
                    "No entry in selection_history found for redo. \
                     This may correspond to a bug where undo does not update the selection. \
                     If this is occurring, please add details to \
                     https://github.com/zed-industries/zed/issues/22692"
                );
            }
            self.request_autoscroll(Autoscroll::fit(), cx);
            self.unmark_text(window, cx);
            self.refresh_edit_prediction(true, false, window, cx);
            cx.emit(EditorEvent::Edited { transaction_id });
        }
    }

    pub fn finalize_last_transaction(&mut self, cx: &mut Context<Self>) {
        self.buffer
            .update(cx, |buffer, cx| buffer.finalize_last_transaction(cx));
    }

    pub fn group_until_transaction(&mut self, tx_id: TransactionId, cx: &mut Context<Self>) {
        self.buffer
            .update(cx, |buffer, cx| buffer.group_until_transaction(tx_id, cx));
    }

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
        self.take_rename(true, window, cx);

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

    pub fn context_menu_first(
        &mut self,
        _: &ContextMenuFirst,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(context_menu) = self.context_menu.borrow_mut().as_mut() {
            context_menu.select_first(self.completion_provider.as_deref(), window, cx);
        }
    }

    pub fn context_menu_prev(
        &mut self,
        _: &ContextMenuPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(context_menu) = self.context_menu.borrow_mut().as_mut() {
            context_menu.select_prev(self.completion_provider.as_deref(), window, cx);
        }
    }

    pub fn context_menu_next(
        &mut self,
        _: &ContextMenuNext,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(context_menu) = self.context_menu.borrow_mut().as_mut() {
            context_menu.select_next(self.completion_provider.as_deref(), window, cx);
        }
    }

    pub fn context_menu_last(
        &mut self,
        _: &ContextMenuLast,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(context_menu) = self.context_menu.borrow_mut().as_mut() {
            context_menu.select_last(self.completion_provider.as_deref(), window, cx);
        }
    }

    pub fn signature_help_prev(
        &mut self,
        _: &SignatureHelpPrevious,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(popover) = self.signature_help_state.popover_mut() {
            if popover.current_signature == 0 {
                popover.current_signature = popover.signatures.len() - 1;
            } else {
                popover.current_signature -= 1;
            }
            cx.notify();
        }
    }

    pub fn signature_help_next(
        &mut self,
        _: &SignatureHelpNext,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(popover) = self.signature_help_state.popover_mut() {
            if popover.current_signature + 1 == popover.signatures.len() {
                popover.current_signature = 0;
            } else {
                popover.current_signature += 1;
            }
            cx.notify();
        }
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

    pub fn delete_to_previous_word_start(
        &mut self,
        action: &DeleteToPreviousWordStart,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }
        self.transact(window, cx, |this, window, cx| {
            this.select_autoclose_pair(window, cx);
            this.change_selections(Default::default(), window, cx, |s| {
                s.move_with(&mut |map, selection| {
                    if selection.is_empty() {
                        let mut cursor = if action.ignore_newlines {
                            movement::previous_word_start(map, selection.head())
                        } else {
                            movement::previous_word_start_or_newline(map, selection.head())
                        };
                        cursor = movement::adjust_greedy_deletion(
                            map,
                            selection.head(),
                            cursor,
                            action.ignore_brackets,
                        );
                        selection.set_head(cursor, SelectionGoal::None);
                    }
                });
            });
            this.insert("", window, cx);
        });
    }

    pub fn delete_to_previous_subword_start(
        &mut self,
        action: &DeleteToPreviousSubwordStart,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }
        self.transact(window, cx, |this, window, cx| {
            this.select_autoclose_pair(window, cx);
            this.change_selections(Default::default(), window, cx, |s| {
                s.move_with(&mut |map, selection| {
                    if selection.is_empty() {
                        let mut cursor = if action.ignore_newlines {
                            movement::previous_subword_start(map, selection.head())
                        } else {
                            movement::previous_subword_start_or_newline(map, selection.head())
                        };
                        cursor = movement::adjust_greedy_deletion(
                            map,
                            selection.head(),
                            cursor,
                            action.ignore_brackets,
                        );
                        selection.set_head(cursor, SelectionGoal::None);
                    }
                });
            });
            this.insert("", window, cx);
        });
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

    pub fn delete_to_next_word_end(
        &mut self,
        action: &DeleteToNextWordEnd,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }
        self.transact(window, cx, |this, window, cx| {
            this.change_selections(Default::default(), window, cx, |s| {
                s.move_with(&mut |map, selection| {
                    if selection.is_empty() {
                        let mut cursor = if action.ignore_newlines {
                            movement::next_word_end(map, selection.head())
                        } else {
                            movement::next_word_end_or_newline(map, selection.head())
                        };
                        cursor = movement::adjust_greedy_deletion(
                            map,
                            selection.head(),
                            cursor,
                            action.ignore_brackets,
                        );
                        selection.set_head(cursor, SelectionGoal::None);
                    }
                });
            });
            this.insert("", window, cx);
        });
    }

    pub fn delete_to_next_subword_end(
        &mut self,
        action: &DeleteToNextSubwordEnd,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }
        self.transact(window, cx, |this, window, cx| {
            this.change_selections(Default::default(), window, cx, |s| {
                s.move_with(&mut |map, selection| {
                    if selection.is_empty() {
                        let mut cursor = if action.ignore_newlines {
                            movement::next_subword_end(map, selection.head())
                        } else {
                            movement::next_subword_end_or_newline(map, selection.head())
                        };
                        cursor = movement::adjust_greedy_deletion(
                            map,
                            selection.head(),
                            cursor,
                            action.ignore_brackets,
                        );
                        selection.set_head(cursor, SelectionGoal::None);
                    }
                });
            });
            this.insert("", window, cx);
        });
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

    pub fn delete_to_beginning_of_line(
        &mut self,
        action: &DeleteToBeginningOfLine,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }
        self.transact(window, cx, |this, window, cx| {
            this.change_selections(Default::default(), window, cx, |s| {
                s.move_with(&mut |_, selection| {
                    selection.reversed = true;
                });
            });

            this.select_to_beginning_of_line(
                &SelectToBeginningOfLine {
                    stop_at_soft_wraps: false,
                    stop_at_indent: action.stop_at_indent,
                },
                window,
                cx,
            );
            this.backspace(&Backspace, window, cx);
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

    pub fn delete_to_end_of_line(
        &mut self,
        _: &DeleteToEndOfLine,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }
        self.transact(window, cx, |this, window, cx| {
            this.select_to_end_of_line(
                &SelectToEndOfLine {
                    stop_at_soft_wraps: false,
                },
                window,
                cx,
            );
            this.delete(&Delete, window, cx);
        });
    }

    pub fn cut_to_end_of_line(
        &mut self,
        action: &CutToEndOfLine,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }
        self.transact(window, cx, |this, window, cx| {
            this.select_to_end_of_line(
                &SelectToEndOfLine {
                    stop_at_soft_wraps: false,
                },
                window,
                cx,
            );
            if !action.stop_at_newlines {
                this.change_selections(Default::default(), window, cx, |s| {
                    s.move_with(&mut |_, sel| {
                        if sel.is_empty() {
                            sel.end = DisplayPoint::new(sel.end.row() + 1_u32, 0);
                        }
                    });
                });
            }
            let item = this.cut_common(false, window, cx);
            cx.write_to_clipboard(item);
        });
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

    pub fn nav_history(&self) -> Option<&ItemNavHistory> {
        self.nav_history.as_ref()
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

    pub(crate) fn navigation_entry(
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

    pub fn select_to_end(&mut self, _: &SelectToEnd, window: &mut Window, cx: &mut Context<Self>) {
        let buffer = self.buffer.read(cx).snapshot(cx);
        let mut selection = self
            .selections
            .first::<MultiBufferOffset>(&self.display_snapshot(cx));
        selection.set_head(buffer.len(), SelectionGoal::None);
        self.change_selections(Default::default(), window, cx, |s| {
            s.select(vec![selection]);
        });
    }

    pub fn select_all(&mut self, _: &SelectAll, window: &mut Window, cx: &mut Context<Self>) {
        self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges(vec![Anchor::Min..Anchor::Max]);
        });
    }

    pub fn select_line(&mut self, _: &SelectLine, window: &mut Window, cx: &mut Context<Self>) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut selections = self.selections.all::<Point>(&display_map);
        let max_point = display_map.buffer_snapshot().max_point();
        for selection in &mut selections {
            let rows = selection.spanned_rows(true, &display_map);
            selection.start = Point::new(rows.start.0, 0);
            selection.end = cmp::min(max_point, Point::new(rows.end.0, 0));
            selection.reversed = false;
        }
        self.change_selections(Default::default(), window, cx, |s| {
            s.select(selections);
        });
    }

    pub fn split_selection_into_lines(
        &mut self,
        action: &SplitSelectionIntoLines,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selections = self
            .selections
            .all::<Point>(&self.display_snapshot(cx))
            .into_iter()
            .map(|selection| selection.start..selection.end)
            .collect::<Vec<_>>();
        self.unfold_ranges(&selections, true, false, cx);

        let mut new_selection_ranges = Vec::new();
        {
            let buffer = self.buffer.read(cx).read(cx);
            for selection in selections {
                for row in selection.start.row..selection.end.row {
                    let line_start = Point::new(row, 0);
                    let line_end = Point::new(row, buffer.line_len(MultiBufferRow(row)));

                    if action.keep_selections {
                        // Keep the selection range for each line
                        let selection_start = if row == selection.start.row {
                            selection.start
                        } else {
                            line_start
                        };
                        new_selection_ranges.push(selection_start..line_end);
                    } else {
                        // Collapse to cursor at end of line
                        new_selection_ranges.push(line_end..line_end);
                    }
                }

                let is_multiline_selection = selection.start.row != selection.end.row;
                // Don't insert last one if it's a multi-line selection ending at the start of a line,
                // so this action feels more ergonomic when paired with other selection operations
                let should_skip_last = is_multiline_selection && selection.end.column == 0;
                if !should_skip_last {
                    if action.keep_selections {
                        if is_multiline_selection {
                            let line_start = Point::new(selection.end.row, 0);
                            new_selection_ranges.push(line_start..selection.end);
                        } else {
                            new_selection_ranges.push(selection.start..selection.end);
                        }
                    } else {
                        new_selection_ranges.push(selection.end..selection.end);
                    }
                }
            }
        }
        self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges(new_selection_ranges);
        });
    }

    pub fn add_selection_above(
        &mut self,
        action: &AddSelectionAbove,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.add_selection(true, action.skip_soft_wrap, window, cx);
    }

    pub fn add_selection_below(
        &mut self,
        action: &AddSelectionBelow,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.add_selection(false, action.skip_soft_wrap, window, cx);
    }

    fn add_selection(
        &mut self,
        above: bool,
        skip_soft_wrap: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let all_selections = self.selections.all::<Point>(&display_map);
        let text_layout_details = self.text_layout_details(window, cx);

        let (mut columnar_selections, new_selections_to_columnarize) = {
            if let Some(state) = self.add_selections_state.as_ref() {
                let columnar_selection_ids: HashSet<_> = state
                    .groups
                    .iter()
                    .flat_map(|group| group.stack.iter())
                    .copied()
                    .collect();

                all_selections
                    .into_iter()
                    .partition(|s| columnar_selection_ids.contains(&s.id))
            } else {
                (Vec::new(), all_selections)
            }
        };

        let mut state = self
            .add_selections_state
            .take()
            .unwrap_or_else(|| AddSelectionsState { groups: Vec::new() });

        for selection in new_selections_to_columnarize {
            let range = selection.display_range(&display_map).sorted();
            let start_x = display_map.x_for_display_point(range.start, &text_layout_details);
            let end_x = display_map.x_for_display_point(range.end, &text_layout_details);
            let positions = start_x.min(end_x)..start_x.max(end_x);
            let mut stack = Vec::new();
            for row in range.start.row().0..=range.end.row().0 {
                if let Some(selection) = self.selections.build_columnar_selection(
                    &display_map,
                    DisplayRow(row),
                    &positions,
                    selection.reversed,
                    &text_layout_details,
                ) {
                    stack.push(selection.id);
                    columnar_selections.push(selection);
                }
            }
            if !stack.is_empty() {
                if above {
                    stack.reverse();
                }
                state.groups.push(AddSelectionsGroup { above, stack });
            }
        }

        let mut final_selections = Vec::new();
        let end_row = if above {
            DisplayRow(0)
        } else {
            display_map.max_point().row()
        };

        // When `skip_soft_wrap` is true, we use UTF-16 columns instead of pixel
        // positions to place new selections, so we need to keep track of the
        // column range of the oldest selection in each group, because
        // intermediate selections may have been clamped to shorter lines.
        let mut goal_columns_by_selection_id = if skip_soft_wrap {
            let mut map = HashMap::default();
            for group in state.groups.iter() {
                if let Some(oldest_id) = group.stack.first() {
                    if let Some(oldest_selection) =
                        columnar_selections.iter().find(|s| s.id == *oldest_id)
                    {
                        let snapshot = display_map.buffer_snapshot();
                        let start_col =
                            snapshot.point_to_point_utf16(oldest_selection.start).column;
                        let end_col = snapshot.point_to_point_utf16(oldest_selection.end).column;
                        let goal_columns = start_col.min(end_col)..start_col.max(end_col);
                        for id in &group.stack {
                            map.insert(*id, goal_columns.clone());
                        }
                    }
                }
            }
            map
        } else {
            HashMap::default()
        };

        let mut last_added_item_per_group = HashMap::default();
        for group in state.groups.iter_mut() {
            if let Some(last_id) = group.stack.last() {
                last_added_item_per_group.insert(*last_id, group);
            }
        }

        for selection in columnar_selections {
            if let Some(group) = last_added_item_per_group.get_mut(&selection.id) {
                if above == group.above {
                    let range = selection.display_range(&display_map).sorted();
                    debug_assert_eq!(range.start.row(), range.end.row());
                    let row = range.start.row();
                    let positions =
                        if let SelectionGoal::HorizontalRange { start, end } = selection.goal {
                            Pixels::from(start)..Pixels::from(end)
                        } else {
                            let start_x =
                                display_map.x_for_display_point(range.start, &text_layout_details);
                            let end_x =
                                display_map.x_for_display_point(range.end, &text_layout_details);
                            start_x.min(end_x)..start_x.max(end_x)
                        };

                    let maybe_new_selection = if skip_soft_wrap {
                        let goal_columns = goal_columns_by_selection_id
                            .remove(&selection.id)
                            .unwrap_or_else(|| {
                                let snapshot = display_map.buffer_snapshot();
                                let start_col =
                                    snapshot.point_to_point_utf16(selection.start).column;
                                let end_col = snapshot.point_to_point_utf16(selection.end).column;
                                start_col.min(end_col)..start_col.max(end_col)
                            });
                        self.selections.find_next_columnar_selection_by_buffer_row(
                            &display_map,
                            row,
                            end_row,
                            above,
                            &goal_columns,
                            selection.reversed,
                            &text_layout_details,
                        )
                    } else {
                        self.selections.find_next_columnar_selection_by_display_row(
                            &display_map,
                            row,
                            end_row,
                            above,
                            &positions,
                            selection.reversed,
                            &text_layout_details,
                        )
                    };

                    if let Some(new_selection) = maybe_new_selection {
                        group.stack.push(new_selection.id);
                        if above {
                            final_selections.push(new_selection);
                            final_selections.push(selection);
                        } else {
                            final_selections.push(selection);
                            final_selections.push(new_selection);
                        }
                    } else {
                        final_selections.push(selection);
                    }
                } else {
                    group.stack.pop();
                }
            } else {
                final_selections.push(selection);
            }
        }

        self.change_selections(Default::default(), window, cx, |s| {
            s.select(final_selections);
        });

        let final_selection_ids: HashSet<_> = self
            .selections
            .all::<Point>(&display_map)
            .iter()
            .map(|s| s.id)
            .collect();
        state.groups.retain_mut(|group| {
            // selections might get merged above so we remove invalid items from stacks
            group.stack.retain(|id| final_selection_ids.contains(id));

            // single selection in stack can be treated as initial state
            group.stack.len() > 1
        });

        if !state.groups.is_empty() {
            self.add_selections_state = Some(state);
        }
    }

    pub fn insert_snippet_at_selections(
        &mut self,
        action: &InsertSnippet,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.try_insert_snippet_at_selections(action, window, cx)
            .log_err();
    }

    fn try_insert_snippet_at_selections(
        &mut self,
        action: &InsertSnippet,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let insertion_ranges = self
            .selections
            .all::<MultiBufferOffset>(&self.display_snapshot(cx))
            .into_iter()
            .map(|selection| selection.range())
            .collect_vec();

        let snippet = if let Some(snippet_body) = &action.snippet {
            if action.language.is_none() && action.name.is_none() {
                Snippet::parse(snippet_body)?
            } else {
                bail!("`snippet` is mutually exclusive with `language` and `name`")
            }
        } else if let Some(name) = &action.name {
            let project = self.project().context("no project")?;
            let snippet_store = project.read(cx).snippets().read(cx);
            let snippet = snippet_store
                .snippets_for(action.language.clone(), cx)
                .into_iter()
                .find(|snippet| snippet.name == *name)
                .context("snippet not found")?;
            Snippet::parse(&snippet.body)?
        } else {
            // todo(andrew): open modal to select snippet
            bail!("`name` or `snippet` is required")
        };

        self.insert_snippet(&insertion_ranges, snippet, window, cx)
    }

    fn select_match_ranges(
        &mut self,
        range: Range<MultiBufferOffset>,
        reversed: bool,
        replace_newest: bool,
        auto_scroll: Option<Autoscroll>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        self.unfold_ranges(
            std::slice::from_ref(&range),
            false,
            auto_scroll.is_some(),
            cx,
        );
        let effects = if let Some(scroll) = auto_scroll {
            SelectionEffects::scroll(scroll)
        } else {
            SelectionEffects::no_scroll()
        };
        self.change_selections(effects, window, cx, |s| {
            if replace_newest {
                s.delete(s.newest_anchor().id);
            }
            if reversed {
                s.insert_range(range.end..range.start);
            } else {
                s.insert_range(range);
            }
        });
    }

    pub fn select_next_match_internal(
        &mut self,
        display_map: &DisplaySnapshot,
        replace_newest: bool,
        autoscroll: Option<Autoscroll>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let buffer = display_map.buffer_snapshot();
        let mut selections = self.selections.all::<MultiBufferOffset>(&display_map);
        if let Some(mut select_next_state) = self.select_next_state.take() {
            let query = &select_next_state.query;
            if !select_next_state.done {
                let first_selection = selections.iter().min_by_key(|s| s.id).unwrap();
                let last_selection = selections.iter().max_by_key(|s| s.id).unwrap();
                let mut next_selected_range = None;

                let bytes_after_last_selection =
                    buffer.bytes_in_range(last_selection.end..buffer.len());
                let bytes_before_first_selection =
                    buffer.bytes_in_range(MultiBufferOffset(0)..first_selection.start);
                let query_matches = query
                    .stream_find_iter(bytes_after_last_selection)
                    .map(|result| (last_selection.end, result))
                    .chain(
                        query
                            .stream_find_iter(bytes_before_first_selection)
                            .map(|result| (MultiBufferOffset(0), result)),
                    );

                for (start_offset, query_match) in query_matches {
                    let query_match = query_match.unwrap(); // can only fail due to I/O
                    let offset_range =
                        start_offset + query_match.start()..start_offset + query_match.end();

                    if !select_next_state.wordwise
                        || (!buffer.is_inside_word(offset_range.start, None)
                            && !buffer.is_inside_word(offset_range.end, None))
                    {
                        let idx = selections
                            .partition_point(|selection| selection.end <= offset_range.start);
                        let overlaps = selections
                            .get(idx)
                            .map_or(false, |selection| selection.start < offset_range.end);

                        if !overlaps {
                            next_selected_range = Some(offset_range);
                            break;
                        }
                    }
                }

                if let Some(next_selected_range) = next_selected_range {
                    self.select_match_ranges(
                        next_selected_range,
                        last_selection.reversed,
                        replace_newest,
                        autoscroll,
                        window,
                        cx,
                    );
                } else {
                    select_next_state.done = true;
                }
            }

            self.select_next_state = Some(select_next_state);
        } else {
            let mut only_carets = true;
            let mut same_text_selected = true;
            let mut selected_text = None;

            let mut selections_iter = selections.iter().peekable();
            while let Some(selection) = selections_iter.next() {
                if selection.start != selection.end {
                    only_carets = false;
                }

                if same_text_selected {
                    if selected_text.is_none() {
                        selected_text =
                            Some(buffer.text_for_range(selection.range()).collect::<String>());
                    }

                    if let Some(next_selection) = selections_iter.peek() {
                        if next_selection.len() == selection.len() {
                            let next_selected_text = buffer
                                .text_for_range(next_selection.range())
                                .collect::<String>();
                            if Some(next_selected_text) != selected_text {
                                same_text_selected = false;
                                selected_text = None;
                            }
                        } else {
                            same_text_selected = false;
                            selected_text = None;
                        }
                    }
                }
            }

            if only_carets {
                for selection in &mut selections {
                    let (word_range, _) = buffer.surrounding_word(selection.start, None);
                    selection.start = word_range.start;
                    selection.end = word_range.end;
                    selection.goal = SelectionGoal::None;
                    selection.reversed = false;
                    self.select_match_ranges(
                        selection.start..selection.end,
                        selection.reversed,
                        replace_newest,
                        autoscroll,
                        window,
                        cx,
                    );
                }

                if selections.len() == 1 {
                    let selection = selections
                        .last()
                        .expect("ensured that there's only one selection");
                    let query = buffer
                        .text_for_range(selection.start..selection.end)
                        .collect::<String>();
                    let is_empty = query.is_empty();
                    let select_state = SelectNextState {
                        query: self.build_query(&[query], cx)?,
                        wordwise: true,
                        done: is_empty,
                    };
                    self.select_next_state = Some(select_state);
                } else {
                    self.select_next_state = None;
                }
            } else if let Some(selected_text) = selected_text {
                self.select_next_state = Some(SelectNextState {
                    query: self.build_query(&[selected_text], cx)?,
                    wordwise: false,
                    done: false,
                });
                self.select_next_match_internal(
                    display_map,
                    replace_newest,
                    autoscroll,
                    window,
                    cx,
                )?;
            }
        }
        Ok(())
    }

    pub fn select_all_matches(
        &mut self,
        _action: &SelectAllMatches,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        self.select_next_match_internal(&display_map, false, None, window, cx)?;
        let Some(select_next_state) = self.select_next_state.as_mut().filter(|state| !state.done)
        else {
            return Ok(());
        };

        let mut new_selections = Vec::new();
        let initial_selection = self.selections.oldest::<MultiBufferOffset>(&display_map);
        let reversed = initial_selection.reversed;
        let buffer = display_map.buffer_snapshot();
        let query_matches = select_next_state
            .query
            .stream_find_iter(buffer.bytes_in_range(MultiBufferOffset(0)..buffer.len()));

        for query_match in query_matches.into_iter() {
            let query_match = query_match.context("query match for select all action")?; // can only fail due to I/O
            let offset_range = if reversed {
                MultiBufferOffset(query_match.end())..MultiBufferOffset(query_match.start())
            } else {
                MultiBufferOffset(query_match.start())..MultiBufferOffset(query_match.end())
            };

            let is_partial_word_match = select_next_state.wordwise
                && (buffer.is_inside_word(offset_range.start, None)
                    || buffer.is_inside_word(offset_range.end, None));

            let is_initial_selection = MultiBufferOffset(query_match.start())
                == initial_selection.start
                && MultiBufferOffset(query_match.end()) == initial_selection.end;

            if !is_partial_word_match && !is_initial_selection {
                new_selections.push(offset_range);
            }
        }

        // Ensure that the initial range is the last selection, as
        // `MutableSelectionsCollection::select_ranges` makes the last selection
        // the newest selection, which the editor then relies on as the primary
        // cursor for scroll targeting. Without this, the last match would then
        // be automatically focused when the user started editing the selected
        // matches.
        let initial_directed_range = if reversed {
            initial_selection.end..initial_selection.start
        } else {
            initial_selection.start..initial_selection.end
        };
        new_selections.push(initial_directed_range);

        select_next_state.done = true;
        self.unfold_ranges(&new_selections, false, false, cx);
        self.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
            selections.select_ranges(new_selections)
        });

        Ok(())
    }

    pub fn select_next(
        &mut self,
        action: &SelectNext,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        self.select_next_match_internal(
            &display_map,
            action.replace_newest,
            Some(Autoscroll::newest()),
            window,
            cx,
        )
    }

    pub fn select_previous(
        &mut self,
        action: &SelectPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = display_map.buffer_snapshot();
        let mut selections = self.selections.all::<MultiBufferOffset>(&display_map);
        if let Some(mut select_prev_state) = self.select_prev_state.take() {
            let query = &select_prev_state.query;
            if !select_prev_state.done {
                let first_selection = selections.iter().min_by_key(|s| s.id).unwrap();
                let last_selection = selections.iter().max_by_key(|s| s.id).unwrap();
                let mut next_selected_range = None;
                // When we're iterating matches backwards, the oldest match will actually be the furthest one in the buffer.
                let bytes_before_last_selection =
                    buffer.reversed_bytes_in_range(MultiBufferOffset(0)..last_selection.start);
                let bytes_after_first_selection =
                    buffer.reversed_bytes_in_range(first_selection.end..buffer.len());
                let query_matches = query
                    .stream_find_iter(bytes_before_last_selection)
                    .map(|result| (last_selection.start, result))
                    .chain(
                        query
                            .stream_find_iter(bytes_after_first_selection)
                            .map(|result| (buffer.len(), result)),
                    );
                for (end_offset, query_match) in query_matches {
                    let query_match = query_match.unwrap(); // can only fail due to I/O
                    let offset_range =
                        end_offset - query_match.end()..end_offset - query_match.start();

                    if !select_prev_state.wordwise
                        || (!buffer.is_inside_word(offset_range.start, None)
                            && !buffer.is_inside_word(offset_range.end, None))
                    {
                        next_selected_range = Some(offset_range);
                        break;
                    }
                }

                if let Some(next_selected_range) = next_selected_range {
                    self.select_match_ranges(
                        next_selected_range,
                        last_selection.reversed,
                        action.replace_newest,
                        Some(Autoscroll::newest()),
                        window,
                        cx,
                    );
                } else {
                    select_prev_state.done = true;
                }
            }

            self.select_prev_state = Some(select_prev_state);
        } else {
            let mut only_carets = true;
            let mut same_text_selected = true;
            let mut selected_text = None;

            let mut selections_iter = selections.iter().peekable();
            while let Some(selection) = selections_iter.next() {
                if selection.start != selection.end {
                    only_carets = false;
                }

                if same_text_selected {
                    if selected_text.is_none() {
                        selected_text =
                            Some(buffer.text_for_range(selection.range()).collect::<String>());
                    }

                    if let Some(next_selection) = selections_iter.peek() {
                        if next_selection.len() == selection.len() {
                            let next_selected_text = buffer
                                .text_for_range(next_selection.range())
                                .collect::<String>();
                            if Some(next_selected_text) != selected_text {
                                same_text_selected = false;
                                selected_text = None;
                            }
                        } else {
                            same_text_selected = false;
                            selected_text = None;
                        }
                    }
                }
            }

            if only_carets {
                for selection in &mut selections {
                    let (word_range, _) = buffer.surrounding_word(selection.start, None);
                    selection.start = word_range.start;
                    selection.end = word_range.end;
                    selection.goal = SelectionGoal::None;
                    selection.reversed = false;
                    self.select_match_ranges(
                        selection.start..selection.end,
                        selection.reversed,
                        action.replace_newest,
                        Some(Autoscroll::newest()),
                        window,
                        cx,
                    );
                }
                if selections.len() == 1 {
                    let selection = selections
                        .last()
                        .expect("ensured that there's only one selection");
                    let query = buffer
                        .text_for_range(selection.start..selection.end)
                        .collect::<String>();
                    let is_empty = query.is_empty();
                    let select_state = SelectNextState {
                        query: self.build_query(&[query.chars().rev().collect::<String>()], cx)?,
                        wordwise: true,
                        done: is_empty,
                    };
                    self.select_prev_state = Some(select_state);
                } else {
                    self.select_prev_state = None;
                }
            } else if let Some(selected_text) = selected_text {
                self.select_prev_state = Some(SelectNextState {
                    query: self
                        .build_query(&[selected_text.chars().rev().collect::<String>()], cx)?,
                    wordwise: false,
                    done: false,
                });
                self.select_previous(action, window, cx)?;
            }
        }
        Ok(())
    }

    /// Builds an `AhoCorasick` automaton from the provided patterns, while
    /// setting the case sensitivity based on the global
    /// `SelectNextCaseSensitive` setting, if set, otherwise based on the
    /// editor's settings.
    fn build_query<I, P>(&self, patterns: I, cx: &Context<Self>) -> Result<AhoCorasick, BuildError>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<[u8]>,
    {
        let case_sensitive = self
            .select_next_is_case_sensitive
            .unwrap_or_else(|| EditorSettings::get_global(cx).search.case_sensitive);

        let mut builder = AhoCorasickBuilder::new();
        builder.ascii_case_insensitive(!case_sensitive);
        builder.build(patterns)
    }

    pub fn find_next_match(
        &mut self,
        _: &FindNextMatch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let selections = self.selections.disjoint_anchors_arc();
        match selections.first() {
            Some(first) if selections.len() >= 2 => {
                self.change_selections(Default::default(), window, cx, |s| {
                    s.select_ranges([first.range()]);
                });
            }
            _ => self.select_next(
                &SelectNext {
                    replace_newest: true,
                },
                window,
                cx,
            )?,
        }
        Ok(())
    }

    pub fn find_previous_match(
        &mut self,
        _: &FindPreviousMatch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let selections = self.selections.disjoint_anchors_arc();
        match selections.last() {
            Some(last) if selections.len() >= 2 => {
                self.change_selections(Default::default(), window, cx, |s| {
                    s.select_ranges([last.range()]);
                });
            }
            _ => self.select_previous(
                &SelectPrevious {
                    replace_newest: true,
                },
                window,
                cx,
            )?,
        }
        Ok(())
    }

    pub fn toggle_block_comments(
        &mut self,
        _: &ToggleBlockComments,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }
        self.transact(window, cx, |this, _window, cx| {
            let mut selections = this
                .selections
                .all::<MultiBufferPoint>(&this.display_snapshot(cx));
            let mut edits = Vec::new();
            let snapshot = this.buffer.read(cx).read(cx);
            let empty_str: Arc<str> = Arc::default();
            let mut markers_inserted = Vec::new();

            for selection in &mut selections {
                let start_point = selection.start;
                let end_point = selection.end;

                let Some(language) =
                    snapshot.language_scope_at(Point::new(start_point.row, start_point.column))
                else {
                    continue;
                };

                let Some(BlockCommentConfig {
                    start: comment_start,
                    end: comment_end,
                    ..
                }) = language.block_comment()
                else {
                    continue;
                };

                let prefix_needle = comment_start.trim_end().as_bytes();
                let suffix_needle = comment_end.trim_start().as_bytes();

                // Collect full lines spanning the selection as the search region
                let region_start = Point::new(start_point.row, 0);
                let region_end = Point::new(
                    end_point.row,
                    snapshot.line_len(MultiBufferRow(end_point.row)),
                );
                let region_bytes: Vec<u8> = snapshot
                    .bytes_in_range(region_start..region_end)
                    .flatten()
                    .copied()
                    .collect();

                let region_start_offset = snapshot.point_to_offset(region_start);
                let start_byte = snapshot.point_to_offset(start_point) - region_start_offset;
                let end_byte = snapshot.point_to_offset(end_point) - region_start_offset;

                let mut is_commented = false;
                let mut prefix_range = start_point..start_point;
                let mut suffix_range = end_point..end_point;

                // Find rightmost /* at or before the selection end
                if let Some(prefix_pos) = region_bytes[..end_byte.min(region_bytes.len())]
                    .windows(prefix_needle.len())
                    .rposition(|w| w == prefix_needle)
                {
                    let after_prefix = prefix_pos + prefix_needle.len();

                    // Find the first */ after that /*
                    if let Some(suffix_pos) = region_bytes[after_prefix..]
                        .windows(suffix_needle.len())
                        .position(|w| w == suffix_needle)
                        .map(|p| p + after_prefix)
                    {
                        let suffix_end = suffix_pos + suffix_needle.len();

                        // Case 1: /* ... */ surrounds the selection
                        let markers_surround = prefix_pos <= start_byte
                            && suffix_end >= end_byte
                            && start_byte < suffix_end;

                        // Case 2: selection contains /* ... */ (only whitespace padding)
                        let selection_contains = start_byte <= prefix_pos
                            && suffix_end <= end_byte
                            && region_bytes[start_byte..prefix_pos]
                                .iter()
                                .all(|&b| b.is_ascii_whitespace())
                            && region_bytes[suffix_end..end_byte]
                                .iter()
                                .all(|&b| b.is_ascii_whitespace());

                        if markers_surround || selection_contains {
                            is_commented = true;
                            let prefix_pt =
                                snapshot.offset_to_point(region_start_offset + prefix_pos);
                            let suffix_pt =
                                snapshot.offset_to_point(region_start_offset + suffix_pos);
                            prefix_range = prefix_pt
                                ..Point::new(
                                    prefix_pt.row,
                                    prefix_pt.column + prefix_needle.len() as u32,
                                );
                            suffix_range = suffix_pt
                                ..Point::new(
                                    suffix_pt.row,
                                    suffix_pt.column + suffix_needle.len() as u32,
                                );
                        }
                    }
                }

                if is_commented {
                    // Also remove the space after /* and before */
                    if snapshot
                        .bytes_in_range(prefix_range.end..snapshot.max_point())
                        .flatten()
                        .next()
                        == Some(&b' ')
                    {
                        prefix_range.end.column += 1;
                    }
                    if suffix_range.start.column > 0 {
                        let before =
                            Point::new(suffix_range.start.row, suffix_range.start.column - 1);
                        if snapshot
                            .bytes_in_range(before..suffix_range.start)
                            .flatten()
                            .next()
                            == Some(&b' ')
                        {
                            suffix_range.start.column -= 1;
                        }
                    }

                    edits.push((prefix_range, empty_str.clone()));
                    edits.push((suffix_range, empty_str.clone()));
                } else {
                    let prefix: Arc<str> = if comment_start.ends_with(' ') {
                        comment_start.clone()
                    } else {
                        format!("{} ", comment_start).into()
                    };
                    let suffix: Arc<str> = if comment_end.starts_with(' ') {
                        comment_end.clone()
                    } else {
                        format!(" {}", comment_end).into()
                    };

                    edits.push((start_point..start_point, prefix.clone()));
                    edits.push((end_point..end_point, suffix.clone()));
                    markers_inserted.push((
                        selection.id,
                        prefix.len(),
                        suffix.len(),
                        selection.is_empty(),
                        end_point.row,
                    ));
                }
            }

            drop(snapshot);
            this.buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
            });

            let mut selections = this
                .selections
                .all::<MultiBufferPoint>(&this.display_snapshot(cx));
            for selection in &mut selections {
                if let Some((_, prefix_len, suffix_len, was_empty, suffix_row)) = markers_inserted
                    .iter()
                    .find(|(id, _, _, _, _)| *id == selection.id)
                {
                    if *was_empty {
                        selection.start.column = selection
                            .start
                            .column
                            .saturating_sub((*prefix_len + *suffix_len) as u32);
                    } else {
                        selection.start.column =
                            selection.start.column.saturating_sub(*prefix_len as u32);
                        if selection.end.row == *suffix_row {
                            selection.end.column += *suffix_len as u32;
                        }
                    }
                }
            }
            this.change_selections(Default::default(), _window, cx, |s| s.select(selections));
        });
    }

    pub fn toggle_comments(
        &mut self,
        action: &ToggleComments,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }
        let text_layout_details = &self.text_layout_details(window, cx);
        self.transact(window, cx, |this, window, cx| {
            let mut selections = this
                .selections
                .all::<MultiBufferPoint>(&this.display_snapshot(cx));
            let mut edits = Vec::new();
            let mut selection_edit_ranges = Vec::new();
            let mut last_toggled_row = None;
            let snapshot = this.buffer.read(cx).read(cx);
            let empty_str: Arc<str> = Arc::default();
            let mut suffixes_inserted = Vec::new();
            let ignore_indent = action.ignore_indent;

            fn comment_prefix_range(
                snapshot: &MultiBufferSnapshot,
                row: MultiBufferRow,
                comment_prefix: &str,
                comment_prefix_whitespace: &str,
                ignore_indent: bool,
            ) -> Range<Point> {
                let indent_size = if ignore_indent {
                    0
                } else {
                    snapshot.indent_size_for_line(row).len
                };

                let start = Point::new(row.0, indent_size);

                let mut line_bytes = snapshot
                    .bytes_in_range(start..snapshot.max_point())
                    .flatten()
                    .copied();

                // If this line currently begins with the line comment prefix, then record
                // the range containing the prefix.
                if line_bytes
                    .by_ref()
                    .take(comment_prefix.len())
                    .eq(comment_prefix.bytes())
                {
                    // Include any whitespace that matches the comment prefix.
                    let matching_whitespace_len = line_bytes
                        .zip(comment_prefix_whitespace.bytes())
                        .take_while(|(a, b)| a == b)
                        .count() as u32;
                    let end = Point::new(
                        start.row,
                        start.column + comment_prefix.len() as u32 + matching_whitespace_len,
                    );
                    start..end
                } else {
                    start..start
                }
            }

            fn comment_suffix_range(
                snapshot: &MultiBufferSnapshot,
                row: MultiBufferRow,
                comment_suffix: &str,
                comment_suffix_has_leading_space: bool,
            ) -> Range<Point> {
                let end = Point::new(row.0, snapshot.line_len(row));
                let suffix_start_column = end.column.saturating_sub(comment_suffix.len() as u32);

                let mut line_end_bytes = snapshot
                    .bytes_in_range(Point::new(end.row, suffix_start_column.saturating_sub(1))..end)
                    .flatten()
                    .copied();

                let leading_space_len = if suffix_start_column > 0
                    && line_end_bytes.next() == Some(b' ')
                    && comment_suffix_has_leading_space
                {
                    1
                } else {
                    0
                };

                // If this line currently begins with the line comment prefix, then record
                // the range containing the prefix.
                if line_end_bytes.by_ref().eq(comment_suffix.bytes()) {
                    let start = Point::new(end.row, suffix_start_column - leading_space_len);
                    start..end
                } else {
                    end..end
                }
            }

            // TODO: Handle selections that cross excerpts
            for selection in &mut selections {
                let start_column = snapshot
                    .indent_size_for_line(MultiBufferRow(selection.start.row))
                    .len;
                let language = if let Some(language) =
                    snapshot.language_scope_at(Point::new(selection.start.row, start_column))
                {
                    language
                } else {
                    continue;
                };

                selection_edit_ranges.clear();

                // If multiple selections contain a given row, avoid processing that
                // row more than once.
                let mut start_row = MultiBufferRow(selection.start.row);
                if last_toggled_row == Some(start_row) {
                    start_row = start_row.next_row();
                }
                let end_row =
                    if selection.end.row > selection.start.row && selection.end.column == 0 {
                        MultiBufferRow(selection.end.row - 1)
                    } else {
                        MultiBufferRow(selection.end.row)
                    };
                last_toggled_row = Some(end_row);

                if start_row > end_row {
                    continue;
                }

                // If the language has line comments, toggle those.
                let mut full_comment_prefixes = language.line_comment_prefixes().to_vec();

                // If ignore_indent is set, trim spaces from the right side of all full_comment_prefixes
                if ignore_indent {
                    full_comment_prefixes = full_comment_prefixes
                        .into_iter()
                        .map(|s| Arc::from(s.trim_end()))
                        .collect();
                }

                if !full_comment_prefixes.is_empty() {
                    let first_prefix = full_comment_prefixes
                        .first()
                        .expect("prefixes is non-empty");
                    let prefix_trimmed_lengths = full_comment_prefixes
                        .iter()
                        .map(|p| p.trim_end_matches(' ').len())
                        .collect::<SmallVec<[usize; 4]>>();

                    let mut all_selection_lines_are_comments = true;

                    for row in start_row.0..=end_row.0 {
                        let row = MultiBufferRow(row);
                        if start_row < end_row && snapshot.is_line_blank(row) {
                            continue;
                        }

                        let prefix_range = full_comment_prefixes
                            .iter()
                            .zip(prefix_trimmed_lengths.iter().copied())
                            .map(|(prefix, trimmed_prefix_len)| {
                                comment_prefix_range(
                                    snapshot.deref(),
                                    row,
                                    &prefix[..trimmed_prefix_len],
                                    &prefix[trimmed_prefix_len..],
                                    ignore_indent,
                                )
                            })
                            .max_by_key(|range| range.end.column - range.start.column)
                            .expect("prefixes is non-empty");

                        if prefix_range.is_empty() {
                            all_selection_lines_are_comments = false;
                        }

                        selection_edit_ranges.push(prefix_range);
                    }

                    if all_selection_lines_are_comments {
                        edits.extend(
                            selection_edit_ranges
                                .iter()
                                .cloned()
                                .map(|range| (range, empty_str.clone())),
                        );
                    } else {
                        let min_column = selection_edit_ranges
                            .iter()
                            .map(|range| range.start.column)
                            .min()
                            .unwrap_or(0);
                        edits.extend(selection_edit_ranges.iter().map(|range| {
                            let position = Point::new(range.start.row, min_column);
                            (position..position, first_prefix.clone())
                        }));
                    }
                } else if let Some(BlockCommentConfig {
                    start: full_comment_prefix,
                    end: comment_suffix,
                    ..
                }) = language.block_comment()
                {
                    let comment_prefix = full_comment_prefix.trim_end_matches(' ');
                    let comment_prefix_whitespace = &full_comment_prefix[comment_prefix.len()..];
                    let prefix_range = comment_prefix_range(
                        snapshot.deref(),
                        start_row,
                        comment_prefix,
                        comment_prefix_whitespace,
                        ignore_indent,
                    );
                    let suffix_range = comment_suffix_range(
                        snapshot.deref(),
                        end_row,
                        comment_suffix.trim_start_matches(' '),
                        comment_suffix.starts_with(' '),
                    );

                    if prefix_range.is_empty() || suffix_range.is_empty() {
                        edits.push((
                            prefix_range.start..prefix_range.start,
                            full_comment_prefix.clone(),
                        ));
                        edits.push((suffix_range.end..suffix_range.end, comment_suffix.clone()));
                        suffixes_inserted.push((end_row, comment_suffix.len()));
                    } else {
                        edits.push((prefix_range, empty_str.clone()));
                        edits.push((suffix_range, empty_str.clone()));
                    }
                } else {
                    continue;
                }
            }

            drop(snapshot);
            this.buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
            });

            // Adjust selections so that they end before any comment suffixes that
            // were inserted.
            let mut suffixes_inserted = suffixes_inserted.into_iter().peekable();
            let mut selections = this.selections.all::<Point>(&this.display_snapshot(cx));
            let snapshot = this.buffer.read(cx).read(cx);
            for selection in &mut selections {
                while let Some((row, suffix_len)) = suffixes_inserted.peek().copied() {
                    match row.cmp(&MultiBufferRow(selection.end.row)) {
                        Ordering::Less => {
                            suffixes_inserted.next();
                            continue;
                        }
                        Ordering::Greater => break,
                        Ordering::Equal => {
                            if selection.end.column == snapshot.line_len(row) {
                                if selection.is_empty() {
                                    selection.start.column -= suffix_len as u32;
                                }
                                selection.end.column -= suffix_len as u32;
                            }
                            break;
                        }
                    }
                }
            }

            drop(snapshot);
            this.change_selections(Default::default(), window, cx, |s| s.select(selections));

            let selections = this.selections.all::<Point>(&this.display_snapshot(cx));
            let selections_on_single_row = selections.windows(2).all(|selections| {
                selections[0].start.row == selections[1].start.row
                    && selections[0].end.row == selections[1].end.row
                    && selections[0].start.row == selections[0].end.row
            });
            let selections_selecting = selections
                .iter()
                .any(|selection| selection.start != selection.end);
            let advance_downwards = action.advance_downwards
                && selections_on_single_row
                && !selections_selecting
                && !matches!(this.mode, EditorMode::SingleLine);

            if advance_downwards {
                let snapshot = this.buffer.read(cx).snapshot(cx);

                this.change_selections(Default::default(), window, cx, |s| {
                    s.move_cursors_with(&mut |display_snapshot, display_point, _| {
                        let mut point = display_point.to_point(display_snapshot);
                        point.row += 1;
                        point = snapshot.clip_point(point, Bias::Left);
                        let display_point = point.to_display_point(display_snapshot);
                        let goal = SelectionGoal::HorizontalPosition(
                            display_snapshot
                                .x_for_display_point(display_point, text_layout_details)
                                .into(),
                        );
                        (display_point, goal)
                    })
                });
            }
        });
    }

    pub fn select_enclosing_symbol(
        &mut self,
        _: &SelectEnclosingSymbol,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let buffer = self.buffer.read(cx).snapshot(cx);
        let old_selections = self
            .selections
            .all::<MultiBufferOffset>(&self.display_snapshot(cx))
            .into_boxed_slice();

        fn update_selection(
            selection: &Selection<MultiBufferOffset>,
            buffer_snap: &MultiBufferSnapshot,
        ) -> Option<Selection<MultiBufferOffset>> {
            let cursor = selection.head();
            let (_buffer_id, symbols) = buffer_snap.symbols_containing(cursor, None)?;
            for symbol in symbols.iter().rev() {
                let start = symbol.range.start.to_offset(buffer_snap);
                let end = symbol.range.end.to_offset(buffer_snap);
                let new_range = start..end;
                if start < selection.start || end > selection.end {
                    return Some(Selection {
                        id: selection.id,
                        start: new_range.start,
                        end: new_range.end,
                        goal: SelectionGoal::None,
                        reversed: selection.reversed,
                    });
                }
            }
            None
        }

        let mut selected_larger_symbol = false;
        let new_selections = old_selections
            .iter()
            .map(|selection| match update_selection(selection, &buffer) {
                Some(new_selection) => {
                    if new_selection.range() != selection.range() {
                        selected_larger_symbol = true;
                    }
                    new_selection
                }
                None => selection.clone(),
            })
            .collect::<Vec<_>>();

        if selected_larger_symbol {
            self.change_selections(Default::default(), window, cx, |s| {
                s.select(new_selections);
            });
        }
    }

    pub fn select_larger_syntax_node(
        &mut self,
        _: &SelectLargerSyntaxNode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(visible_row_count) = self.visible_row_count() else {
            return;
        };
        let old_selections: Box<[_]> = self
            .selections
            .all::<MultiBufferOffset>(&self.display_snapshot(cx))
            .into();
        if old_selections.is_empty() {
            return;
        }

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);

        let mut selected_larger_node = false;
        let mut new_selections = old_selections
            .iter()
            .map(|selection| {
                let old_range = selection.start..selection.end;

                if let Some((node, _)) = buffer.syntax_ancestor(old_range.clone()) {
                    // manually select word at selection
                    if ["string_content", "inline"].contains(&node.kind()) {
                        let (word_range, _) = buffer.surrounding_word(old_range.start, None);
                        // ignore if word is already selected
                        if !word_range.is_empty() && old_range != word_range {
                            let (last_word_range, _) = buffer.surrounding_word(old_range.end, None);
                            // only select word if start and end point belongs to same word
                            if word_range == last_word_range {
                                selected_larger_node = true;
                                return Selection {
                                    id: selection.id,
                                    start: word_range.start,
                                    end: word_range.end,
                                    goal: SelectionGoal::None,
                                    reversed: selection.reversed,
                                };
                            }
                        }
                    }
                }

                let mut new_range = old_range.clone();
                while let Some((node, range)) = buffer.syntax_ancestor(new_range.clone()) {
                    new_range = range;
                    if !node.is_named() {
                        continue;
                    }
                    if !display_map.intersects_fold(new_range.start)
                        && !display_map.intersects_fold(new_range.end)
                    {
                        break;
                    }
                }

                selected_larger_node |= new_range != old_range;
                Selection {
                    id: selection.id,
                    start: new_range.start,
                    end: new_range.end,
                    goal: SelectionGoal::None,
                    reversed: selection.reversed,
                }
            })
            .collect::<Vec<_>>();

        if !selected_larger_node {
            return; // don't put this call in the history
        }

        // scroll based on transformation done to the last selection created by the user
        let (last_old, last_new) = old_selections
            .last()
            .zip(new_selections.last().cloned())
            .expect("old_selections isn't empty");

        let is_selection_reversed = if new_selections.len() == 1 {
            let should_be_reversed = last_old.start != last_new.start;
            new_selections.last_mut().expect("checked above").reversed = should_be_reversed;
            should_be_reversed
        } else {
            last_new.reversed
        };

        if selected_larger_node {
            self.select_syntax_node_history.disable_clearing = true;
            self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select(new_selections.clone());
            });
            self.select_syntax_node_history.disable_clearing = false;
        }

        let start_row = last_new.start.to_display_point(&display_map).row().0;
        let end_row = last_new.end.to_display_point(&display_map).row().0;
        let selection_height = end_row - start_row + 1;
        let scroll_margin_rows = self.vertical_scroll_margin() as u32;

        let fits_on_the_screen = visible_row_count >= selection_height + scroll_margin_rows * 2;
        let scroll_behavior = if fits_on_the_screen {
            self.request_autoscroll(Autoscroll::fit(), cx);
            SelectSyntaxNodeScrollBehavior::FitSelection
        } else if is_selection_reversed {
            self.scroll_cursor_top(&ScrollCursorTop, window, cx);
            SelectSyntaxNodeScrollBehavior::CursorTop
        } else {
            self.scroll_cursor_bottom(&ScrollCursorBottom, window, cx);
            SelectSyntaxNodeScrollBehavior::CursorBottom
        };

        let old_selections: Box<[Selection<Anchor>]> = old_selections
            .iter()
            .map(|s| s.map(|offset| buffer.anchor_before(offset)))
            .collect();
        self.select_syntax_node_history.push((
            old_selections,
            scroll_behavior,
            is_selection_reversed,
        ));
    }

    pub fn select_smaller_syntax_node(
        &mut self,
        _: &SelectSmallerSyntaxNode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some((mut selections, scroll_behavior, is_selection_reversed)) =
            self.select_syntax_node_history.pop()
        {
            if let Some(selection) = selections.last_mut() {
                selection.reversed = is_selection_reversed;
            }

            let snapshot = self.buffer.read(cx).snapshot(cx);
            let selections: Vec<Selection<MultiBufferOffset>> = selections
                .iter()
                .map(|s| s.map(|anchor| anchor.to_offset(&snapshot)))
                .collect();

            self.select_syntax_node_history.disable_clearing = true;
            self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select(selections);
            });
            self.select_syntax_node_history.disable_clearing = false;

            match scroll_behavior {
                SelectSyntaxNodeScrollBehavior::CursorTop => {
                    self.scroll_cursor_top(&ScrollCursorTop, window, cx);
                }
                SelectSyntaxNodeScrollBehavior::FitSelection => {
                    self.request_autoscroll(Autoscroll::fit(), cx);
                }
                SelectSyntaxNodeScrollBehavior::CursorBottom => {
                    self.scroll_cursor_bottom(&ScrollCursorBottom, window, cx);
                }
            }
        }
    }

    pub fn unwrap_syntax_node(
        &mut self,
        _: &UnwrapSyntaxNode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }

        let buffer = self.buffer.read(cx).snapshot(cx);
        let selections = self
            .selections
            .all::<MultiBufferOffset>(&self.display_snapshot(cx))
            .into_iter()
            // subtracting the offset requires sorting
            .sorted_by_key(|i| i.start);

        let full_edits = selections
            .into_iter()
            .filter_map(|selection| {
                let child = if selection.is_empty()
                    && let Some((_, ancestor_range)) =
                        buffer.syntax_ancestor(selection.start..selection.end)
                {
                    ancestor_range
                } else {
                    selection.range()
                };

                let mut parent = child.clone();
                while let Some((_, ancestor_range)) = buffer.syntax_ancestor(parent.clone()) {
                    parent = ancestor_range;
                    if parent.start < child.start || parent.end > child.end {
                        break;
                    }
                }

                if parent == child {
                    return None;
                }
                let text = buffer.text_for_range(child).collect::<String>();
                Some((selection.id, parent, text))
            })
            .collect::<Vec<_>>();
        if full_edits.is_empty() {
            return;
        }

        self.transact(window, cx, |this, window, cx| {
            this.buffer.update(cx, |buffer, cx| {
                buffer.edit(
                    full_edits
                        .iter()
                        .map(|(_, p, t)| (p.clone(), t.clone()))
                        .collect::<Vec<_>>(),
                    None,
                    cx,
                );
            });
            this.change_selections(Default::default(), window, cx, |s| {
                let mut offset = 0;
                let mut selections = vec![];
                for (id, parent, text) in full_edits {
                    let start = parent.start - offset;
                    offset += (parent.end - parent.start) - text.len();
                    selections.push(Selection {
                        id,
                        start,
                        end: start + text.len(),
                        reversed: false,
                        goal: Default::default(),
                    });
                }
                s.select(selections);
            });
        });
    }

    pub fn select_next_syntax_node(
        &mut self,
        _: &SelectNextSyntaxNode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let old_selections = self.selections.all_anchors(&self.display_snapshot(cx));
        if old_selections.is_empty() {
            return;
        }

        let buffer = self.buffer.read(cx).snapshot(cx);
        let mut selected_sibling = false;

        let new_selections = old_selections
            .iter()
            .map(|selection| {
                let old_range =
                    selection.start.to_offset(&buffer)..selection.end.to_offset(&buffer);
                if let Some(results) = buffer.map_excerpt_ranges(
                    old_range,
                    |buf, _excerpt_range, input_buffer_range| {
                        let Some(node) = buf.syntax_next_sibling(input_buffer_range) else {
                            return Vec::new();
                        };
                        vec![(
                            BufferOffset(node.byte_range().start)
                                ..BufferOffset(node.byte_range().end),
                            (),
                        )]
                    },
                ) && let [(new_range, _)] = results.as_slice()
                {
                    selected_sibling = true;
                    let new_range =
                        buffer.anchor_after(new_range.start)..buffer.anchor_before(new_range.end);
                    Selection {
                        id: selection.id,
                        start: new_range.start,
                        end: new_range.end,
                        goal: SelectionGoal::None,
                        reversed: selection.reversed,
                    }
                } else {
                    selection.clone()
                }
            })
            .collect::<Vec<_>>();

        if selected_sibling {
            self.change_selections(
                SelectionEffects::scroll(Autoscroll::fit()),
                window,
                cx,
                |s| {
                    s.select(new_selections);
                },
            );
        }
    }

    pub fn select_prev_syntax_node(
        &mut self,
        _: &SelectPreviousSyntaxNode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let old_selections: Arc<[_]> = self.selections.all_anchors(&self.display_snapshot(cx));

        let multibuffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let mut selected_sibling = false;

        let new_selections = old_selections
            .iter()
            .map(|selection| {
                let old_range = selection.start.to_offset(&multibuffer_snapshot)
                    ..selection.end.to_offset(&multibuffer_snapshot);
                if let Some(results) = multibuffer_snapshot.map_excerpt_ranges(
                    old_range,
                    |buf, _excerpt_range, input_buffer_range| {
                        let Some(node) = buf.syntax_prev_sibling(input_buffer_range) else {
                            return Vec::new();
                        };
                        vec![(
                            BufferOffset(node.byte_range().start)
                                ..BufferOffset(node.byte_range().end),
                            (),
                        )]
                    },
                ) && let [(new_range, _)] = results.as_slice()
                {
                    selected_sibling = true;
                    let new_range = multibuffer_snapshot.anchor_after(new_range.start)
                        ..multibuffer_snapshot.anchor_before(new_range.end);
                    Selection {
                        id: selection.id,
                        start: new_range.start,
                        end: new_range.end,
                        goal: SelectionGoal::None,
                        reversed: selection.reversed,
                    }
                } else {
                    selection.clone()
                }
            })
            .collect::<Vec<_>>();

        if selected_sibling {
            self.change_selections(
                SelectionEffects::scroll(Autoscroll::fit()),
                window,
                cx,
                |s| {
                    s.select(new_selections);
                },
            );
        }
    }

    pub fn move_to_start_of_larger_syntax_node(
        &mut self,
        _: &MoveToStartOfLargerSyntaxNode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_cursors_to_syntax_nodes(window, cx, false);
    }

    pub fn move_to_end_of_larger_syntax_node(
        &mut self,
        _: &MoveToEndOfLargerSyntaxNode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_cursors_to_syntax_nodes(window, cx, true);
    }

    fn find_syntax_node_boundary(
        &self,
        selection_pos: MultiBufferOffset,
        move_to_end: bool,
        display_map: &DisplaySnapshot,
        buffer: &MultiBufferSnapshot,
    ) -> MultiBufferOffset {
        let old_range = selection_pos..selection_pos;
        let mut new_pos = selection_pos;
        let mut search_range = old_range;
        while let Some((node, range)) = buffer.syntax_ancestor(search_range.clone()) {
            search_range = range.clone();
            if !node.is_named()
                || display_map.intersects_fold(range.start)
                || display_map.intersects_fold(range.end)
                // If cursor is already at the end of the syntax node, continue searching
                || (move_to_end && range.end == selection_pos)
                // If cursor is already at the start of the syntax node, continue searching
                || (!move_to_end && range.start == selection_pos)
            {
                continue;
            }

            // If we found a string_content node, find the largest parent that is still string_content
            // Enables us to skip to the end of strings without taking multiple steps inside the string
            let (_, final_range) = if node.kind() == "string_content" {
                let mut current_node = node;
                let mut current_range = range;
                while let Some((parent, parent_range)) =
                    buffer.syntax_ancestor(current_range.clone())
                {
                    if parent.kind() == "string_content" {
                        current_node = parent;
                        current_range = parent_range;
                    } else {
                        break;
                    }
                }

                (current_node, current_range)
            } else {
                (node, range)
            };

            new_pos = if move_to_end {
                final_range.end
            } else {
                final_range.start
            };

            break;
        }

        new_pos
    }

    fn move_cursors_to_syntax_nodes(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        move_to_end: bool,
    ) -> bool {
        let old_selections: Box<[_]> = self
            .selections
            .all::<MultiBufferOffset>(&self.display_snapshot(cx))
            .into();
        if old_selections.is_empty() {
            return false;
        }

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);

        let mut any_cursor_moved = false;
        let new_selections = old_selections
            .iter()
            .map(|selection| {
                if !selection.is_empty() {
                    return selection.clone();
                }

                let selection_pos = selection.head();
                let new_pos = self.find_syntax_node_boundary(
                    selection_pos,
                    move_to_end,
                    &display_map,
                    &buffer,
                );

                any_cursor_moved |= new_pos != selection_pos;

                Selection {
                    id: selection.id,
                    start: new_pos,
                    end: new_pos,
                    goal: SelectionGoal::None,
                    reversed: false,
                }
            })
            .collect::<Vec<_>>();

        self.change_selections(Default::default(), window, cx, |s| {
            s.select(new_selections);
        });
        self.request_autoscroll(Autoscroll::newest(), cx);

        any_cursor_moved
    }

    pub fn select_to_start_of_larger_syntax_node(
        &mut self,
        _: &SelectToStartOfLargerSyntaxNode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_to_syntax_nodes(window, cx, false);
    }

    pub fn select_to_end_of_larger_syntax_node(
        &mut self,
        _: &SelectToEndOfLargerSyntaxNode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_to_syntax_nodes(window, cx, true);
    }

    fn select_to_syntax_nodes(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        move_to_end: bool,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);
        let old_selections = self.selections.all::<MultiBufferOffset>(&display_map);

        let new_selections = old_selections
            .iter()
            .map(|selection| {
                let new_pos = self.find_syntax_node_boundary(
                    selection.head(),
                    move_to_end,
                    &display_map,
                    &buffer,
                );

                let mut new_selection = selection.clone();
                new_selection.set_head(new_pos, SelectionGoal::None);
                new_selection
            })
            .collect::<Vec<_>>();

        self.change_selections(Default::default(), window, cx, |s| {
            s.select(new_selections);
        });
    }

    pub fn move_to_enclosing_bracket(
        &mut self,
        _: &MoveToEnclosingBracket,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.change_selections(Default::default(), window, cx, |s| {
            s.move_offsets_with(&mut |snapshot, selection| {
                let Some(enclosing_bracket_ranges) =
                    snapshot.enclosing_bracket_ranges(selection.start..selection.end)
                else {
                    return;
                };

                let mut best_length = usize::MAX;
                let mut best_inside = false;
                let mut best_in_bracket_range = false;
                let mut best_destination = None;
                for (open, close) in enclosing_bracket_ranges {
                    let close = close.to_inclusive();
                    let length = *close.end() - open.start;
                    let inside = selection.start >= open.end && selection.end <= *close.start();
                    let in_bracket_range = open.to_inclusive().contains(&selection.head())
                        || close.contains(&selection.head());

                    // If best is next to a bracket and current isn't, skip
                    if !in_bracket_range && best_in_bracket_range {
                        continue;
                    }

                    // Prefer smaller lengths unless best is inside and current isn't
                    if length > best_length && (best_inside || !inside) {
                        continue;
                    }

                    best_length = length;
                    best_inside = inside;
                    best_in_bracket_range = in_bracket_range;
                    best_destination = Some(
                        if close.contains(&selection.start) && close.contains(&selection.end) {
                            if inside { open.end } else { open.start }
                        } else if inside {
                            *close.start()
                        } else {
                            *close.end()
                        },
                    );
                }

                if let Some(destination) = best_destination {
                    selection.collapse_to(destination, SelectionGoal::None);
                }
            })
        });
    }

    pub fn undo_selection(
        &mut self,
        _: &UndoSelection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(entry) = self.selection_history.undo_stack.pop_back() {
            self.selection_history.mode = SelectionHistoryMode::Undoing;
            self.with_selection_effects_deferred(window, cx, |this, window, cx| {
                this.end_selection(window, cx);
                this.change_selections(
                    SelectionEffects::scroll(Autoscroll::newest()),
                    window,
                    cx,
                    |s| s.select_anchors(entry.selections.to_vec()),
                );
            });
            self.selection_history.mode = SelectionHistoryMode::Normal;

            self.select_next_state = entry.select_next_state;
            self.select_prev_state = entry.select_prev_state;
            self.add_selections_state = entry.add_selections_state;
        }
    }

    pub fn redo_selection(
        &mut self,
        _: &RedoSelection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(entry) = self.selection_history.redo_stack.pop_back() {
            self.selection_history.mode = SelectionHistoryMode::Redoing;
            self.with_selection_effects_deferred(window, cx, |this, window, cx| {
                this.end_selection(window, cx);
                this.change_selections(
                    SelectionEffects::scroll(Autoscroll::newest()),
                    window,
                    cx,
                    |s| s.select_anchors(entry.selections.to_vec()),
                );
            });
            self.selection_history.mode = SelectionHistoryMode::Normal;

            self.select_next_state = entry.select_next_state;
            self.select_prev_state = entry.select_prev_state;
            self.add_selections_state = entry.add_selections_state;
        }
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

    pub fn expand_excerpts_for_direction(
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

    pub(crate) fn expand_excerpt(
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

    pub fn go_to_document_highlight_before_or_after_position(
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

    pub(crate) fn navigate_to_hover_links(
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
                let (target_buffer, target_ranges) = locations.into_iter().next().unwrap();

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

                let _ = editor
                    .update(acx, |editor, ecx| {
                        editor.change_selections(
                            SelectionEffects::scroll(Autoscroll::newest()),
                            window,
                            ecx,
                            |s| s.select_ranges(selection),
                        );
                    })
                    .ok();
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
                let (target_buffer, target_ranges) = locations.into_iter().next().unwrap();
                let target_range = target_ranges.first().unwrap().clone();

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

    /// Opens a multibuffer with the given project locations in it.
    pub fn open_locations_in_multibuffer(
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

    pub fn rename(
        &mut self,
        _: &Rename,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        use language::ToOffset as _;

        if self.read_only(cx) {
            return None;
        }
        let provider = self.semantics_provider.clone()?;
        let selection = self.selections.newest_anchor().clone();
        let (cursor_buffer, cursor_buffer_position) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(selection.head(), cx)?;
        let (tail_buffer, cursor_buffer_position_end) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(selection.tail(), cx)?;
        if tail_buffer != cursor_buffer {
            return None;
        }

        let snapshot = cursor_buffer.read(cx).snapshot();
        let cursor_buffer_offset = cursor_buffer_position.to_offset(&snapshot);
        let cursor_buffer_offset_end = cursor_buffer_position_end.to_offset(&snapshot);
        let prepare_rename = provider.range_for_rename(&cursor_buffer, cursor_buffer_position, cx);
        drop(snapshot);

        Some(cx.spawn_in(window, async move |this, cx| {
            let rename_range = prepare_rename.await?;
            if let Some(rename_range) = rename_range {
                this.update_in(cx, |this, window, cx| {
                    let snapshot = cursor_buffer.read(cx).snapshot();
                    let rename_buffer_range = rename_range.to_offset(&snapshot);
                    let cursor_offset_in_rename_range =
                        cursor_buffer_offset.saturating_sub(rename_buffer_range.start);
                    let cursor_offset_in_rename_range_end =
                        cursor_buffer_offset_end.saturating_sub(rename_buffer_range.start);

                    this.take_rename(false, window, cx);
                    let buffer = this.buffer.read(cx).read(cx);
                    let cursor_offset = selection.head().to_offset(&buffer);
                    let rename_start =
                        cursor_offset.saturating_sub_usize(cursor_offset_in_rename_range);
                    let rename_end = rename_start + rename_buffer_range.len();
                    let range = buffer.anchor_before(rename_start)..buffer.anchor_after(rename_end);
                    let mut old_highlight_id = None;
                    let old_name: Arc<str> = buffer
                        .chunks(
                            rename_start..rename_end,
                            LanguageAwareStyling {
                                tree_sitter: true,
                                diagnostics: true,
                            },
                        )
                        .map(|chunk| {
                            if old_highlight_id.is_none() {
                                old_highlight_id = chunk.syntax_highlight_id;
                            }
                            chunk.text
                        })
                        .collect::<String>()
                        .into();

                    drop(buffer);

                    // Position the selection in the rename editor so that it matches the current selection.
                    this.show_local_selections = false;
                    let rename_editor = cx.new(|cx| {
                        let mut editor = Editor::single_line(window, cx);
                        editor.buffer.update(cx, |buffer, cx| {
                            buffer.edit(
                                [(MultiBufferOffset(0)..MultiBufferOffset(0), old_name.clone())],
                                None,
                                cx,
                            )
                        });
                        let cursor_offset_in_rename_range =
                            MultiBufferOffset(cursor_offset_in_rename_range);
                        let cursor_offset_in_rename_range_end =
                            MultiBufferOffset(cursor_offset_in_rename_range_end);
                        let rename_selection_range = match cursor_offset_in_rename_range
                            .cmp(&cursor_offset_in_rename_range_end)
                        {
                            Ordering::Equal => {
                                editor.select_all(&SelectAll, window, cx);
                                return editor;
                            }
                            Ordering::Less => {
                                cursor_offset_in_rename_range..cursor_offset_in_rename_range_end
                            }
                            Ordering::Greater => {
                                cursor_offset_in_rename_range_end..cursor_offset_in_rename_range
                            }
                        };
                        if rename_selection_range.end.0 > old_name.len() {
                            editor.select_all(&SelectAll, window, cx);
                        } else {
                            editor.change_selections(Default::default(), window, cx, |s| {
                                s.select_ranges([rename_selection_range]);
                            });
                        }
                        editor
                    });
                    cx.subscribe(&rename_editor, |_, _, e: &EditorEvent, cx| {
                        if e == &EditorEvent::Focused {
                            cx.emit(EditorEvent::FocusedIn)
                        }
                    })
                    .detach();

                    let write_highlights =
                        this.clear_background_highlights(HighlightKey::DocumentHighlightWrite, cx);
                    let read_highlights =
                        this.clear_background_highlights(HighlightKey::DocumentHighlightRead, cx);
                    let ranges = write_highlights
                        .iter()
                        .flat_map(|(_, ranges)| ranges.iter())
                        .chain(read_highlights.iter().flat_map(|(_, ranges)| ranges.iter()))
                        .cloned()
                        .collect();

                    this.highlight_text(
                        HighlightKey::Rename,
                        ranges,
                        HighlightStyle {
                            fade_out: Some(0.6),
                            ..Default::default()
                        },
                        cx,
                    );
                    let rename_focus_handle = rename_editor.focus_handle(cx);
                    window.focus(&rename_focus_handle, cx);
                    let block_id = this.insert_blocks(
                        [BlockProperties {
                            style: BlockStyle::Flex,
                            placement: BlockPlacement::Below(range.start),
                            height: Some(1),
                            render: Arc::new({
                                let rename_editor = rename_editor.clone();
                                move |cx: &mut BlockContext| {
                                    let mut text_style = cx.editor_style.text.clone();
                                    if let Some(highlight_style) = old_highlight_id
                                        .and_then(|h| cx.editor_style.syntax.get(h).cloned())
                                    {
                                        text_style = text_style.highlight(highlight_style);
                                    }
                                    div()
                                        .block_mouse_except_scroll()
                                        .pl(cx.anchor_x)
                                        .child(EditorElement::new(
                                            &rename_editor,
                                            EditorStyle {
                                                background: cx.theme().system().transparent,
                                                local_player: cx.editor_style.local_player,
                                                text: text_style,
                                                scrollbar_width: cx.editor_style.scrollbar_width,
                                                syntax: cx.editor_style.syntax.clone(),
                                                status: cx.editor_style.status.clone(),
                                                inlay_hints_style: HighlightStyle {
                                                    font_weight: Some(FontWeight::BOLD),
                                                    ..make_inlay_hints_style(cx.app)
                                                },
                                                edit_prediction_styles: make_suggestion_styles(
                                                    cx.app,
                                                ),
                                                ..EditorStyle::default()
                                            },
                                        ))
                                        .into_any_element()
                                }
                            }),
                            priority: 0,
                        }],
                        Some(Autoscroll::fit()),
                        cx,
                    )[0];
                    this.pending_rename = Some(RenameState {
                        range,
                        old_name,
                        editor: rename_editor,
                        block_id,
                    });
                })?;
            }

            Ok(())
        }))
    }

    pub fn confirm_rename(
        &mut self,
        _: &ConfirmRename,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.read_only(cx) {
            return None;
        }
        let rename = self.take_rename(false, window, cx)?;
        let workspace = self.workspace()?.downgrade();
        let (buffer, start) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(rename.range.start, cx)?;
        let (end_buffer, _) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(rename.range.end, cx)?;
        if buffer != end_buffer {
            return None;
        }

        let old_name = rename.old_name;
        let new_name = rename.editor.read(cx).text(cx);

        let rename = self.semantics_provider.as_ref()?.perform_rename(
            &buffer,
            start,
            new_name.clone(),
            cx,
        )?;

        Some(cx.spawn_in(window, async move |editor, cx| {
            let project_transaction = rename.await?;
            Self::open_project_transaction(
                &editor,
                workspace,
                project_transaction,
                format!("Rename: {} → {}", old_name, new_name),
                cx,
            )
            .await?;

            editor.update(cx, |editor, cx| {
                editor.refresh_document_highlights(cx);
            })?;
            Ok(())
        }))
    }

    pub(super) fn take_rename(
        &mut self,
        moving_cursor: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<RenameState> {
        let rename = self.pending_rename.take()?;
        if rename.editor.focus_handle(cx).is_focused(window) {
            window.focus(&self.focus_handle, cx);
        }

        self.remove_blocks(
            [rename.block_id].into_iter().collect(),
            Some(Autoscroll::fit()),
            cx,
        );
        self.clear_highlights(HighlightKey::Rename, cx);
        self.show_local_selections = true;

        if moving_cursor {
            let cursor_in_rename_editor = rename.editor.update(cx, |editor, cx| {
                editor
                    .selections
                    .newest::<MultiBufferOffset>(&editor.display_snapshot(cx))
                    .head()
            });

            // Update the selection to match the position of the selection inside
            // the rename editor.
            let snapshot = self.buffer.read(cx).read(cx);
            let rename_range = rename.range.to_offset(&snapshot);
            let cursor_in_editor = snapshot
                .clip_offset(rename_range.start + cursor_in_rename_editor, Bias::Left)
                .min(rename_range.end);
            drop(snapshot);

            self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_ranges(vec![cursor_in_editor..cursor_in_editor])
            });
        } else {
            self.refresh_document_highlights(cx);
        }

        Some(rename)
    }

    pub fn pending_rename(&self) -> Option<&RenameState> {
        self.pending_rename.as_ref()
    }

    pub(super) fn can_format_selections(&self, cx: &App) -> bool {
        if !self.mode.is_full() {
            return false;
        }

        let Some(project) = &self.project else {
            return false;
        };

        let project = project.read(cx);
        let multi_buffer = self.buffer.read(cx);
        let snapshot = multi_buffer.snapshot(cx);

        self.selections
            .disjoint_anchor_ranges()
            .filter(|range| range.start != range.end)
            .flat_map(|range| [range.start, range.end])
            .filter_map(|anchor| snapshot.anchor_to_buffer_anchor(anchor))
            .filter_map(|(_, buffer_snapshot)| multi_buffer.buffer(buffer_snapshot.remote_id()))
            .any(|buffer| project.supports_range_formatting(&buffer, cx))
    }

    pub(super) fn format(
        &mut self,
        _: &Format,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.read_only(cx) {
            return None;
        }

        let project = match &self.project {
            Some(project) => project.clone(),
            None => return None,
        };

        Some(self.perform_format(
            project,
            FormatTrigger::Manual,
            FormatTarget::Buffers(self.buffer.read(cx).all_buffers()),
            window,
            cx,
        ))
    }

    pub(super) fn format_selections(
        &mut self,
        _: &FormatSelections,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.read_only(cx) {
            return None;
        }

        let project = match &self.project {
            Some(project) => project.clone(),
            None => return None,
        };

        let ranges = self
            .selections
            .all_adjusted(&self.display_snapshot(cx))
            .into_iter()
            .map(|selection| selection.range())
            .collect_vec();

        Some(self.perform_format(
            project,
            FormatTrigger::Manual,
            FormatTarget::Ranges(ranges),
            window,
            cx,
        ))
    }

    pub(super) fn perform_format(
        &mut self,
        project: Entity<Project>,
        trigger: FormatTrigger,
        target: FormatTarget,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let buffer = self.buffer.clone();
        let (buffers, target) = match target {
            FormatTarget::Buffers(buffers) => (buffers, LspFormatTarget::Buffers),
            FormatTarget::Ranges(selection_ranges) => {
                let multi_buffer = buffer.read(cx);
                let snapshot = multi_buffer.read(cx);
                let mut buffers = HashSet::default();
                let mut buffer_id_to_ranges: BTreeMap<BufferId, Vec<Range<text::Anchor>>> =
                    BTreeMap::new();
                for selection_range in selection_ranges {
                    for (buffer_snapshot, buffer_range, _) in
                        snapshot.range_to_buffer_ranges(selection_range.start..selection_range.end)
                    {
                        let buffer_id = buffer_snapshot.remote_id();
                        let start = buffer_snapshot.anchor_before(buffer_range.start);
                        let end = buffer_snapshot.anchor_after(buffer_range.end);
                        buffers.insert(multi_buffer.buffer(buffer_id).unwrap());
                        buffer_id_to_ranges
                            .entry(buffer_id)
                            .and_modify(|buffer_ranges| buffer_ranges.push(start..end))
                            .or_insert_with(|| vec![start..end]);
                    }
                }
                (buffers, LspFormatTarget::Ranges(buffer_id_to_ranges))
            }
        };

        let transaction_id_prev = buffer.read(cx).last_transaction_id(cx);
        let selections_prev = transaction_id_prev
            .and_then(|transaction_id_prev| {
                // default to selections as they were after the last edit, if we have them,
                // instead of how they are now.
                // This will make it so that editing, moving somewhere else, formatting, then undoing the format
                // will take you back to where you made the last edit, instead of staying where you scrolled
                self.selection_history
                    .transaction(transaction_id_prev)
                    .map(|t| t.0.clone())
            })
            .unwrap_or_else(|| self.selections.disjoint_anchors_arc());

        let mut timeout = cx.background_executor().timer(FORMAT_TIMEOUT).fuse();
        let format = project.update(cx, |project, cx| {
            project.format(buffers, target, true, trigger, cx)
        });

        cx.spawn_in(window, async move |editor, cx| {
            let transaction = futures::select_biased! {
                transaction = format.log_err().fuse() => transaction,
                () = timeout => {
                    log::warn!("timed out waiting for formatting");
                    None
                }
            };

            buffer.update(cx, |buffer, cx| {
                if let Some(transaction) = transaction
                    && !buffer.is_singleton()
                {
                    buffer.push_transaction(&transaction.0, cx);
                }
                cx.notify();
            });

            if let Some(transaction_id_now) =
                buffer.read_with(cx, |b, cx| b.last_transaction_id(cx))
            {
                let has_new_transaction = transaction_id_prev != Some(transaction_id_now);
                if has_new_transaction {
                    editor
                        .update(cx, |editor, _| {
                            editor
                                .selection_history
                                .insert_transaction(transaction_id_now, selections_prev);
                        })
                        .ok();
                }
            }

            Ok(())
        })
    }

    pub(super) fn organize_imports(
        &mut self,
        _: &OrganizeImports,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.read_only(cx) {
            return None;
        }
        let project = match &self.project {
            Some(project) => project.clone(),
            None => return None,
        };
        Some(self.perform_code_action_kind(
            project,
            CodeActionKind::SOURCE_ORGANIZE_IMPORTS,
            window,
            cx,
        ))
    }

    pub(super) fn perform_code_action_kind(
        &mut self,
        project: Entity<Project>,
        kind: CodeActionKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let buffer = self.buffer.clone();
        let buffers = buffer.read(cx).all_buffers();
        let mut timeout = cx.background_executor().timer(CODE_ACTION_TIMEOUT).fuse();
        let apply_action = project.update(cx, |project, cx| {
            project.apply_code_action_kind(buffers, kind, true, cx)
        });
        cx.spawn_in(window, async move |_, cx| {
            let transaction = futures::select_biased! {
                () = timeout => {
                    log::warn!("timed out waiting for executing code action");
                    None
                }
                transaction = apply_action.log_err().fuse() => transaction,
            };
            buffer.update(cx, |buffer, cx| {
                // check if we need this
                if let Some(transaction) = transaction
                    && !buffer.is_singleton()
                {
                    buffer.push_transaction(&transaction.0, cx);
                }
                cx.notify();
            });
            Ok(())
        })
    }

    pub fn restart_language_server(
        &mut self,
        _: &RestartLanguageServer,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(project) = self.project.clone() {
            self.buffer.update(cx, |multi_buffer, cx| {
                project.update(cx, |project, cx| {
                    project.restart_language_servers_for_buffers(
                        multi_buffer.all_buffers().into_iter().collect(),
                        HashSet::default(),
                        cx,
                    );
                });
            })
        }
    }

    pub fn stop_language_server(
        &mut self,
        _: &StopLanguageServer,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(project) = self.project.clone() {
            self.buffer.update(cx, |multi_buffer, cx| {
                project.update(cx, |project, cx| {
                    project.stop_language_servers_for_buffers(
                        multi_buffer.all_buffers().into_iter().collect(),
                        HashSet::default(),
                        cx,
                    );
                });
            });
        }
    }

    pub(super) fn cancel_language_server_work(
        workspace: &mut Workspace,
        _: &actions::CancelLanguageServerWork,
        _: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let project = workspace.project();
        let buffers = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
            .map_or(HashSet::default(), |editor| {
                editor.read(cx).buffer.read(cx).all_buffers()
            });
        project.update(cx, |project, cx| {
            project.cancel_language_server_work_for_buffers(buffers, cx);
        });
    }

    pub(super) fn show_character_palette(
        &mut self,
        _: &ShowCharacterPalette,
        window: &mut Window,
        _: &mut Context<Self>,
    ) {
        window.show_character_palette();
    }
}
