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

    pub(super) fn begin_columnar_selection(
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
}
