use super::*;

impl Editor {
    pub fn sync_selections(
        &mut self,
        other: Entity<Editor>,
        cx: &mut Context<Self>,
    ) -> gpui::Subscription {
        let other_selections = other.read(cx).selections.disjoint_anchors().to_vec();
        if !other_selections.is_empty() {
            self.selections
                .change_with(&self.display_snapshot(cx), |selections| {
                    selections.select_anchors(other_selections);
                });
        }

        let other_subscription = cx.subscribe(&other, |this, other, other_evt, cx| {
            if let EditorEvent::SelectionsChanged { local: true } = other_evt {
                let other_selections = other.read(cx).selections.disjoint_anchors().to_vec();
                if other_selections.is_empty() {
                    return;
                }
                let snapshot = this.display_snapshot(cx);
                this.selections.change_with(&snapshot, |selections| {
                    selections.select_anchors(other_selections);
                });
            }
        });

        let this_subscription = cx.subscribe_self::<EditorEvent>(move |this, this_evt, cx| {
            if let EditorEvent::SelectionsChanged { local: true } = this_evt {
                let these_selections = this.selections.disjoint_anchors().to_vec();
                if these_selections.is_empty() {
                    return;
                }
                other.update(cx, |other_editor, cx| {
                    let snapshot = other_editor.display_snapshot(cx);
                    other_editor
                        .selections
                        .change_with(&snapshot, |selections| {
                            selections.select_anchors(these_selections);
                        })
                });
            }
        });

        Subscription::join(other_subscription, this_subscription)
    }

    /// Changes selections using the provided mutation function. Changes to `self.selections` occur
    /// immediately, but when run within `transact` or `with_selection_effects_deferred` other
    /// effects of selection change occur at the end of the transaction.
    pub fn change_selections<R>(
        &mut self,
        effects: SelectionEffects,
        window: &mut Window,
        cx: &mut Context<Self>,
        change: impl FnOnce(&mut MutableSelectionsCollection<'_, '_>) -> R,
    ) -> R {
        let snapshot = self.display_snapshot(cx);
        if let Some(state) = &mut self.deferred_selection_effects_state {
            state.effects.scroll = effects.scroll.or(state.effects.scroll);
            state.effects.completions = effects.completions;
            state.effects.nav_history = effects.nav_history.or(state.effects.nav_history);
            let (changed, result) = self.selections.change_with(&snapshot, change);
            state.changed |= changed;
            return result;
        }
        let mut state = DeferredSelectionEffectsState {
            changed: false,
            effects,
            old_cursor_position: self.selections.newest_anchor().head(),
            history_entry: SelectionHistoryEntry {
                selections: self.selections.disjoint_anchors_arc(),
                select_next_state: self.select_next_state.clone(),
                select_prev_state: self.select_prev_state.clone(),
                add_selections_state: self.add_selections_state.clone(),
            },
        };
        let (changed, result) = self.selections.change_with(&snapshot, change);
        state.changed = state.changed || changed;
        if self.defer_selection_effects {
            self.deferred_selection_effects_state = Some(state);
        } else {
            self.apply_selection_effects(state, window, cx);
        }
        result
    }

    /// Defers the effects of selection change, so that the effects of multiple calls to
    /// `change_selections` are applied at the end. This way these intermediate states aren't added
    /// to selection history and the state of popovers based on selection position aren't
    /// erroneously updated.
    pub fn with_selection_effects_deferred<R>(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        update: impl FnOnce(&mut Self, &mut Window, &mut Context<Self>) -> R,
    ) -> R {
        let already_deferred = self.defer_selection_effects;
        self.defer_selection_effects = true;
        let result = update(self, window, cx);
        if !already_deferred {
            self.defer_selection_effects = false;
            if let Some(state) = self.deferred_selection_effects_state.take() {
                self.apply_selection_effects(state, window, cx);
            }
        }
        result
    }

    pub fn has_non_empty_selection(&self, snapshot: &DisplaySnapshot) -> bool {
        self.selections
            .all_adjusted(snapshot)
            .iter()
            .any(|selection| !selection.is_empty())
    }

    pub fn is_range_selected(&mut self, range: &Range<Anchor>, cx: &mut Context<Self>) -> bool {
        if self
            .selections
            .pending_anchor()
            .is_some_and(|pending_selection| {
                let snapshot = self.buffer().read(cx).snapshot(cx);
                pending_selection.range().includes(range, &snapshot)
            })
        {
            return true;
        }

        self.selections
            .disjoint_in_range::<MultiBufferOffset>(range.clone(), &self.display_snapshot(cx))
            .into_iter()
            .any(|selection| {
                // This is needed to cover a corner case, if we just check for an existing
                // selection in the fold range, having a cursor at the start of the fold
                // marks it as selected. Non-empty selections don't cause this.
                let length = selection.end - selection.start;
                length > 0
            })
    }

    pub fn has_pending_nonempty_selection(&self) -> bool {
        let pending_nonempty_selection = match self.selections.pending_anchor() {
            Some(Selection { start, end, .. }) => start != end,
            None => false,
        };

        pending_nonempty_selection
            || (self.columnar_selection_state.is_some()
                && self.selections.disjoint_anchors().len() > 1)
    }

    pub fn has_pending_selection(&self) -> bool {
        self.selections.pending_anchor().is_some() || self.columnar_selection_state.is_some()
    }

    pub fn set_selections_from_remote(
        &mut self,
        selections: Vec<Selection<Anchor>>,
        pending_selection: Option<Selection<Anchor>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let old_cursor_position = self.selections.newest_anchor().head();
        self.selections
            .change_with(&self.display_snapshot(cx), |s| {
                s.select_anchors(selections);
                if let Some(pending_selection) = pending_selection {
                    s.set_pending(pending_selection, SelectMode::Character);
                } else {
                    s.clear_pending();
                }
            });
        self.selections_did_change(
            false,
            &old_cursor_position,
            SelectionEffects::default(),
            window,
            cx,
        );
    }

    pub fn set_mark(&mut self, _: &actions::SetMark, window: &mut Window, cx: &mut Context<Self>) {
        if self.selection_mark_mode {
            self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.move_with(&mut |_, sel| {
                    sel.collapse_to(sel.head(), SelectionGoal::None);
                });
            })
        }
        self.selection_mark_mode = true;
        cx.notify();
    }

    pub fn swap_selection_ends(
        &mut self,
        _: &actions::SwapSelectionEnds,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.move_with(&mut |_, sel| {
                if sel.start != sel.end {
                    sel.reversed = !sel.reversed
                }
            });
        });
        self.request_autoscroll(Autoscroll::newest(), cx);
        cx.notify();
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
                let first_selection = selections
                    .iter()
                    .min_by_key(|s| s.id)
                    .context("missing selection for select previous action")?;
                let last_selection = selections
                    .iter()
                    .max_by_key(|s| s.id)
                    .context("missing selection for select previous action")?;
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
                    let query_match =
                        query_match.context("query match for select previous action")?;
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

        self.select_syntax_node_history.disable_clearing = true;
        self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select(new_selections.clone());
        });
        self.select_syntax_node_history.disable_clearing = false;

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

    pub(super) fn select(
        &mut self,
        phase: SelectPhase,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.hide_context_menu(window, cx);

        match phase {
            SelectPhase::Begin {
                position,
                add,
                click_count,
            } => self.begin_selection(position, add, click_count, window, cx),
            SelectPhase::BeginColumnar {
                position,
                goal_column,
                reset,
                mode,
            } => self.begin_columnar_selection(position, goal_column, reset, mode, window, cx),
            SelectPhase::Extend {
                position,
                click_count,
            } => self.extend_selection(position, click_count, window, cx),
            SelectPhase::Update {
                position,
                goal_column,
                scroll_delta,
            } => self.update_selection(position, goal_column, scroll_delta, window, cx),
            SelectPhase::End => self.end_selection(window, cx),
        }
    }

    pub(super) fn extend_selection(
        &mut self,
        position: DisplayPoint,
        click_count: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let tail = self
            .selections
            .newest::<MultiBufferOffset>(&display_map)
            .tail();
        let click_count = click_count.max(match self.selections.select_mode() {
            SelectMode::Character => 1,
            SelectMode::Word(_) => 2,
            SelectMode::Line(_) => 3,
            SelectMode::All => 4,
        });
        self.begin_selection(position, false, click_count, window, cx);

        let tail_anchor = display_map.buffer_snapshot().anchor_before(tail);

        let current_selection = match self.selections.select_mode() {
            SelectMode::Character | SelectMode::All => tail_anchor..tail_anchor,
            SelectMode::Word(range) | SelectMode::Line(range) => range.clone(),
        };

        let Some((mut pending_selection, mut pending_mode)) = self.pending_selection_and_mode()
        else {
            log::error!("extend_selection dispatched with no pending selection");
            return;
        };

        if pending_selection
            .start
            .cmp(&current_selection.start, display_map.buffer_snapshot())
            == Ordering::Greater
        {
            pending_selection.start = current_selection.start;
        }
        if pending_selection
            .end
            .cmp(&current_selection.end, display_map.buffer_snapshot())
            == Ordering::Less
        {
            pending_selection.end = current_selection.end;
            pending_selection.reversed = true;
        }

        match &mut pending_mode {
            SelectMode::Word(range) | SelectMode::Line(range) => *range = current_selection,
            _ => {}
        }

        let effects = if EditorSettings::get_global(cx).autoscroll_on_clicks {
            SelectionEffects::scroll(Autoscroll::fit())
        } else {
            SelectionEffects::no_scroll()
        };

        self.change_selections(effects, window, cx, |s| {
            s.set_pending(pending_selection.clone(), pending_mode);
            s.set_is_extending(true);
        });
    }

    pub(super) fn begin_selection(
        &mut self,
        position: DisplayPoint,
        add: bool,
        click_count: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.focus_handle.is_focused(window) {
            self.last_focused_descendant = None;
            window.focus(&self.focus_handle, cx);
        }

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = display_map.buffer_snapshot();
        let position = display_map.clip_point(position, Bias::Left);

        let start;
        let end;
        let mode;
        let mut auto_scroll;
        match click_count {
            1 => {
                start = buffer.anchor_before(position.to_point(&display_map));
                end = start;
                mode = SelectMode::Character;
                auto_scroll = true;
            }
            2 => {
                let position = display_map
                    .clip_point(position, Bias::Left)
                    .to_offset(&display_map, Bias::Left);
                let (range, _) = buffer.surrounding_word(position, None);
                start = buffer.anchor_before(range.start);
                end = buffer.anchor_before(range.end);
                mode = SelectMode::Word(start..end);
                auto_scroll = true;
            }
            3 => {
                let position = display_map
                    .clip_point(position, Bias::Left)
                    .to_point(&display_map);
                let line_start = display_map.prev_line_boundary(position).0;
                let next_line_start = buffer.clip_point(
                    display_map.next_line_boundary(position).0 + Point::new(1, 0),
                    Bias::Left,
                );
                start = buffer.anchor_before(line_start);
                end = buffer.anchor_before(next_line_start);
                mode = SelectMode::Line(start..end);
                auto_scroll = true;
            }
            _ => {
                start = buffer.anchor_before(MultiBufferOffset(0));
                end = buffer.anchor_before(buffer.len());
                mode = SelectMode::All;
                auto_scroll = false;
            }
        }
        auto_scroll &= EditorSettings::get_global(cx).autoscroll_on_clicks;

        let point_to_delete: Option<usize> = {
            let selected_points: Vec<Selection<Point>> =
                self.selections.disjoint_in_range(start..end, &display_map);

            if !add || click_count > 1 {
                None
            } else if !selected_points.is_empty() {
                Some(selected_points[0].id)
            } else {
                let clicked_point_already_selected =
                    self.selections.disjoint_anchors().iter().find(|selection| {
                        selection.start.to_point(buffer) == start.to_point(buffer)
                            || selection.end.to_point(buffer) == end.to_point(buffer)
                    });

                clicked_point_already_selected.map(|selection| selection.id)
            }
        };

        let selections_count = self.selections.count();
        let effects = if auto_scroll {
            SelectionEffects::default()
        } else {
            SelectionEffects::no_scroll()
        };

        self.change_selections(effects, window, cx, |s| {
            if let Some(point_to_delete) = point_to_delete {
                s.delete(point_to_delete);

                if selections_count == 1 {
                    s.set_pending_anchor_range(start..end, mode);
                }
            } else {
                if !add {
                    s.clear_disjoint();
                }

                s.set_pending_anchor_range(start..end, mode);
            }
        });
    }

    pub(super) fn update_selection(
        &mut self,
        position: DisplayPoint,
        goal_column: u32,
        scroll_delta: gpui::Point<f32>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        if self.columnar_selection_state.is_some() {
            self.select_columns(position, goal_column, &display_map, window, cx);
        } else if let Some((mut pending, mode)) = self.pending_selection_and_mode() {
            let buffer = display_map.buffer_snapshot();
            let head;
            let tail;
            match &mode {
                SelectMode::Character => {
                    head = position.to_point(&display_map);
                    tail = pending.tail().to_point(buffer);
                }
                SelectMode::Word(original_range) => {
                    let offset = display_map
                        .clip_point(position, Bias::Left)
                        .to_offset(&display_map, Bias::Left);
                    let original_range = original_range.to_offset(buffer);

                    let head_offset = if buffer.is_inside_word(offset, None)
                        || original_range.contains(&offset)
                    {
                        let (word_range, _) = buffer.surrounding_word(offset, None);
                        if word_range.start < original_range.start {
                            word_range.start
                        } else {
                            word_range.end
                        }
                    } else {
                        offset
                    };

                    head = head_offset.to_point(buffer);
                    if head_offset <= original_range.start {
                        tail = original_range.end.to_point(buffer);
                    } else {
                        tail = original_range.start.to_point(buffer);
                    }
                }
                SelectMode::Line(original_range) => {
                    let original_range = original_range.to_point(display_map.buffer_snapshot());

                    let position = display_map
                        .clip_point(position, Bias::Left)
                        .to_point(&display_map);
                    let line_start = display_map.prev_line_boundary(position).0;
                    let next_line_start = buffer.clip_point(
                        display_map.next_line_boundary(position).0 + Point::new(1, 0),
                        Bias::Left,
                    );

                    if line_start < original_range.start {
                        head = line_start
                    } else {
                        head = next_line_start
                    }

                    if head <= original_range.start {
                        tail = original_range.end;
                    } else {
                        tail = original_range.start;
                    }
                }
                SelectMode::All => {
                    return;
                }
            };

            if head < tail {
                pending.start = buffer.anchor_before(head);
                pending.end = buffer.anchor_before(tail);
                pending.reversed = true;
            } else {
                pending.start = buffer.anchor_before(tail);
                pending.end = buffer.anchor_before(head);
                pending.reversed = false;
            }

            self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.set_pending(pending.clone(), mode);
            });
        } else {
            log::error!("update_selection dispatched with no pending selection");
            return;
        }

        self.apply_scroll_delta(scroll_delta, window, cx);
        cx.notify();
    }

    pub(super) fn end_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.columnar_selection_state.take();
        if let Some(pending_mode) = self.selections.pending_mode() {
            let selections = self
                .selections
                .all::<MultiBufferOffset>(&self.display_snapshot(cx));
            self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select(selections);
                s.clear_pending();
                if s.is_extending() {
                    s.set_is_extending(false);
                } else {
                    s.set_select_mode(pending_mode);
                }
            });
        }
    }

    fn begin_columnar_selection(
        &mut self,
        position: DisplayPoint,
        goal_column: u32,
        reset: bool,
        mode: ColumnarMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.focus_handle.is_focused(window) {
            self.last_focused_descendant = None;
            window.focus(&self.focus_handle, cx);
        }

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        if reset {
            let pointer_position = display_map
                .buffer_snapshot()
                .anchor_before(position.to_point(&display_map));

            self.change_selections(
                SelectionEffects::scroll(Autoscroll::newest()),
                window,
                cx,
                |s| {
                    s.clear_disjoint();
                    s.set_pending_anchor_range(
                        pointer_position..pointer_position,
                        SelectMode::Character,
                    );
                },
            );
        };

        let tail = self.selections.newest::<Point>(&display_map).tail();
        let selection_anchor = display_map.buffer_snapshot().anchor_before(tail);
        self.columnar_selection_state = match mode {
            ColumnarMode::FromMouse => Some(ColumnarSelectionState::FromMouse {
                selection_tail: selection_anchor,
                display_point: if reset {
                    if position.column() != goal_column {
                        Some(DisplayPoint::new(position.row(), goal_column))
                    } else {
                        None
                    }
                } else {
                    None
                },
            }),
            ColumnarMode::FromSelection => Some(ColumnarSelectionState::FromSelection {
                selection_tail: selection_anchor,
            }),
        };

        if !reset {
            self.select_columns(position, goal_column, &display_map, window, cx);
        }
    }

    fn selections_did_change(
        &mut self,
        local: bool,
        old_cursor_position: &Anchor,
        effects: SelectionEffects,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.last_selection_from_search = effects.from_search;
        window.invalidate_character_coordinates();

        // Copy selections to primary selection buffer
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        if local {
            let selections = self
                .selections
                .all::<MultiBufferOffset>(&self.display_snapshot(cx));
            let buffer_handle = self.buffer.read(cx).read(cx);

            let mut text = String::new();
            for (index, selection) in selections.iter().enumerate() {
                let text_for_selection = buffer_handle
                    .text_for_range(selection.start..selection.end)
                    .collect::<String>();

                text.push_str(&text_for_selection);
                if index != selections.len() - 1 {
                    text.push('\n');
                }
            }

            if !text.is_empty() {
                cx.write_to_primary(ClipboardItem::new_string(text));
            }
        }

        let selection_anchors = self.selections.disjoint_anchors_arc();

        if self.focus_handle.is_focused(window) && self.leader_id.is_none() {
            self.buffer.update(cx, |buffer, cx| {
                buffer.set_active_selections(
                    &selection_anchors,
                    self.selections.line_mode(),
                    self.cursor_shape,
                    cx,
                )
            });
        }
        let display_map = self
            .display_map
            .update(cx, |display_map, cx| display_map.snapshot(cx));
        let buffer = display_map.buffer_snapshot();
        if self.selections.count() == 1 {
            self.add_selections_state = None;
        }
        self.select_next_state = None;
        self.select_prev_state = None;
        self.select_syntax_node_history.try_clear();
        self.invalidate_autoclose_regions(&selection_anchors, buffer);
        self.snippet_stack.invalidate(&selection_anchors, buffer);
        self.take_rename(false, window, cx);

        let newest_selection = self.selections.newest_anchor();
        let new_cursor_position = newest_selection.head();
        let selection_start = newest_selection.start;

        if effects.nav_history.is_none() || effects.nav_history == Some(true) {
            self.push_to_nav_history(
                *old_cursor_position,
                Some(new_cursor_position.to_point(buffer)),
                false,
                effects.nav_history == Some(true),
                cx,
            );
        }

        if local {
            if let Some((anchor, _)) = buffer.anchor_to_buffer_anchor(new_cursor_position) {
                self.register_buffer(anchor.buffer_id, cx);
            }

            let mut context_menu = self.context_menu.borrow_mut();
            let completion_menu = match context_menu.as_ref() {
                Some(CodeContextMenu::Completions(menu)) => Some(menu),
                Some(CodeContextMenu::CodeActions(_)) => {
                    *context_menu = None;
                    None
                }
                None => None,
            };
            let completion_position = completion_menu.map(|menu| menu.initial_position);
            drop(context_menu);

            if effects.completions
                && let Some(completion_position) = completion_position
            {
                let start_offset = selection_start.to_offset(buffer);
                let position_matches = start_offset == completion_position.to_offset(buffer);
                let continue_showing = if let Some((snap, ..)) =
                    buffer.point_to_buffer_offset(completion_position)
                    && !snap.capability.editable()
                {
                    false
                } else if position_matches {
                    if self.snippet_stack.is_empty() {
                        buffer.char_kind_before(start_offset, Some(CharScopeContext::Completion))
                            == Some(CharKind::Word)
                    } else {
                        // Snippet choices can be shown even when the cursor is in whitespace.
                        // Dismissing the menu with actions like backspace is handled by
                        // invalidation regions.
                        true
                    }
                } else {
                    false
                };

                if continue_showing {
                    self.open_or_update_completions_menu(None, None, false, window, cx);
                } else {
                    self.hide_context_menu(window, cx);
                }
            }

            hide_hover(self, cx);

            self.refresh_code_actions_for_selection(window, cx);
            self.refresh_document_highlights(cx);
            refresh_linked_ranges(self, window, cx);

            self.refresh_selected_text_highlights(&display_map, false, window, cx);
            self.refresh_matching_bracket_highlights(&display_map, cx);
            self.refresh_outline_symbols_at_cursor(cx);
            self.update_visible_edit_prediction(window, cx);
            self.hide_blame_popover(true, cx);
            if self.git_blame_inline_enabled {
                self.start_inline_blame_timer(window, cx);
            }
        }

        self.blink_manager.update(cx, BlinkManager::pause_blinking);

        if local && !self.suppress_selection_callback {
            if let Some(callback) = self.on_local_selections_changed.as_ref() {
                let cursor_position = self.selections.newest::<Point>(&display_map).head();
                callback(cursor_position, window, cx);
            }
        }

        cx.emit(EditorEvent::SelectionsChanged { local });

        let selections = &self.selections.disjoint_anchors_arc();
        if local && let Some(buffer_snapshot) = buffer.as_singleton() {
            let inmemory_selections = selections
                .iter()
                .map(|s| {
                    let start = s.range().start.text_anchor_in(buffer_snapshot);
                    let end = s.range().end.text_anchor_in(buffer_snapshot);
                    (start..end).to_point(buffer_snapshot)
                })
                .collect();
            self.update_restoration_data(cx, |data| {
                data.selections = inmemory_selections;
            });

            if WorkspaceSettings::get(None, cx).restore_on_startup
                != RestoreOnStartupBehavior::EmptyTab
                && let Some(workspace_id) = self.workspace_serialization_id(cx)
            {
                let snapshot = self.buffer().read(cx).snapshot(cx);
                let selections = selections.clone();
                let background_executor = cx.background_executor().clone();
                let editor_id = cx.entity().entity_id().as_u64() as ItemId;
                let db = EditorDb::global(cx);
                self.serialize_selections = cx.background_spawn(async move {
                    background_executor.timer(SERIALIZATION_THROTTLE_TIME).await;
                    let db_selections = selections
                        .iter()
                        .map(|selection| {
                            (
                                selection.start.to_offset(&snapshot).0,
                                selection.end.to_offset(&snapshot).0,
                            )
                        })
                        .collect();

                    db.save_editor_selections(editor_id, workspace_id, db_selections)
                        .await
                        .with_context(|| {
                            format!(
                                "persisting editor selections for editor {editor_id}, \
                                workspace {workspace_id:?}"
                            )
                        })
                        .log_err();
                });
            }
        }

        cx.notify();
    }

    fn apply_selection_effects(
        &mut self,
        state: DeferredSelectionEffectsState,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if state.changed {
            self.selection_history.push(state.history_entry);

            if let Some(autoscroll) = state.effects.scroll {
                self.request_autoscroll(autoscroll, cx);
            }

            let old_cursor_position = &state.old_cursor_position;

            self.selections_did_change(true, old_cursor_position, state.effects, window, cx);

            if self.should_open_signature_help_automatically(old_cursor_position, cx) {
                self.show_signature_help_auto(window, cx);
            }
        }
    }

    fn select_columns(
        &mut self,
        head: DisplayPoint,
        goal_column: u32,
        display_map: &DisplaySnapshot,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(columnar_state) = self.columnar_selection_state.as_ref() else {
            return;
        };

        let tail = match columnar_state {
            ColumnarSelectionState::FromMouse {
                selection_tail,
                display_point,
            } => display_point.unwrap_or_else(|| selection_tail.to_display_point(display_map)),
            ColumnarSelectionState::FromSelection { selection_tail } => {
                selection_tail.to_display_point(display_map)
            }
        };

        let start_row = cmp::min(tail.row(), head.row());
        let end_row = cmp::max(tail.row(), head.row());
        let start_column = cmp::min(tail.column(), goal_column);
        let end_column = cmp::max(tail.column(), goal_column);
        let reversed = start_column < tail.column();

        let selection_ranges = (start_row.0..=end_row.0)
            .map(DisplayRow)
            .filter_map(|row| {
                if (matches!(columnar_state, ColumnarSelectionState::FromMouse { .. })
                    || start_column <= display_map.line_len(row))
                    && !display_map.is_block_line(row)
                {
                    let start = display_map
                        .clip_point(DisplayPoint::new(row, start_column), Bias::Left)
                        .to_point(display_map);
                    let end = display_map
                        .clip_point(DisplayPoint::new(row, end_column), Bias::Right)
                        .to_point(display_map);
                    if reversed {
                        Some(end..start)
                    } else {
                        Some(start..end)
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if selection_ranges.is_empty() {
            return;
        }

        let ranges = match columnar_state {
            ColumnarSelectionState::FromMouse { .. } => {
                let mut non_empty_ranges = selection_ranges
                    .iter()
                    .filter(|selection_range| selection_range.start != selection_range.end)
                    .peekable();
                if non_empty_ranges.peek().is_some() {
                    non_empty_ranges.cloned().collect()
                } else {
                    selection_ranges
                }
            }
            _ => selection_ranges,
        };

        self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges(ranges);
        });
        cx.notify();
    }

    fn pending_selection_and_mode(&self) -> Option<(Selection<Anchor>, SelectMode)> {
        Some((
            self.selections.pending_anchor()?.clone(),
            self.selections.pending_mode()?,
        ))
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

    fn select_next_match_internal(
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
                let first_selection = selections
                    .iter()
                    .min_by_key(|s| s.id)
                    .context("missing selection for select next action")?;
                let last_selection = selections
                    .iter()
                    .max_by_key(|s| s.id)
                    .context("missing selection for select next action")?;
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
                    let query_match = query_match.context("query match for select next action")?;
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
    ) {
        let old_selections: Box<[_]> = self
            .selections
            .all::<MultiBufferOffset>(&self.display_snapshot(cx))
            .into();
        if old_selections.is_empty() {
            return;
        }

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = self.buffer.read(cx).snapshot(cx);

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
}
