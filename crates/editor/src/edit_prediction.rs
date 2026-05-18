use super::*;

pub fn make_suggestion_styles(cx: &App) -> EditPredictionStyles {
    EditPredictionStyles {
        insertion: HighlightStyle {
            color: Some(cx.theme().status().predictive),
            ..HighlightStyle::default()
        },
        whitespace: HighlightStyle {
            background_color: Some(cx.theme().status().created_background),
            ..HighlightStyle::default()
        },
    }
}

pub(super) enum EditDisplayMode {
    TabAccept,
    DiffPopover,
    Inline,
}

pub(super) enum EditPrediction {
    Edit {
        // TODO could be a language::Anchor?
        edits: Vec<(Range<Anchor>, Arc<str>)>,
        /// Predicted cursor position as (anchor, offset_from_anchor).
        /// The anchor is in multibuffer coordinates; after applying edits,
        /// resolve the anchor and add the offset to get the final cursor position.
        cursor_position: Option<(Anchor, usize)>,
        edit_preview: Option<EditPreview>,
        display_mode: EditDisplayMode,
        snapshot: BufferSnapshot,
    },
    /// Move to a specific location in the active editor
    MoveWithin {
        target: Anchor,
        snapshot: BufferSnapshot,
    },
    /// Move to a specific location in a different editor (not the active one)
    MoveOutside {
        target: language::Anchor,
        snapshot: BufferSnapshot,
    },
}

pub(super) struct EditPredictionState {
    pub(super) inlay_ids: Vec<InlayId>,
    pub(super) completion: EditPrediction,
    pub(super) completion_id: Option<SharedString>,
    pub(super) invalidation_range: Option<Range<Anchor>>,
}

pub(super) enum EditPredictionSettings {
    Disabled,
    Enabled {
        show_in_menu: bool,
        preview_requires_modifier: bool,
    },
}

pub(super) enum MenuEditPredictionsPolicy {
    #[cfg(test)]
    Never,
    ByProvider,
}

pub(super) enum EditPredictionPreview {
    /// Modifier is not pressed
    Inactive { released_too_fast: bool },
    /// Modifier pressed
    Active {
        since: Instant,
        previous_scroll_position: Option<SharedScrollAnchor>,
    },
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) enum EditPredictionKeybindSurface {
    Inline,
    CursorPopoverCompact,
    CursorPopoverExpanded,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(super) enum EditPredictionKeybindAction {
    Accept,
    Preview,
}

pub(super) struct EditPredictionKeybindDisplay {
    #[cfg(test)]
    pub(super) accept_keystroke: Option<gpui::KeybindingKeystroke>,
    #[cfg(test)]
    pub(super) preview_keystroke: Option<gpui::KeybindingKeystroke>,
    pub(super) displayed_keystroke: Option<gpui::KeybindingKeystroke>,
    pub(super) action: EditPredictionKeybindAction,
    pub(super) missing_accept_keystroke: bool,
    pub(super) show_hold_label: bool,
}

impl EditPredictionPreview {
    pub(super) fn released_too_fast(&self) -> bool {
        match self {
            EditPredictionPreview::Inactive { released_too_fast } => *released_too_fast,
            EditPredictionPreview::Active { .. } => false,
        }
    }

    pub(super) fn set_previous_scroll_position(
        &mut self,
        scroll_position: Option<SharedScrollAnchor>,
    ) {
        if let EditPredictionPreview::Active {
            previous_scroll_position,
            ..
        } = self
        {
            *previous_scroll_position = scroll_position;
        }
    }
}

pub(super) struct RegisteredEditPredictionDelegate {
    pub(super) provider: Arc<dyn EditPredictionDelegateHandle>,
    _subscription: Subscription,
}

pub(super) fn edit_prediction_edit_text(
    current_snapshot: &BufferSnapshot,
    edits: &[(Range<Anchor>, impl AsRef<str>)],
    edit_preview: &EditPreview,
    include_deletions: bool,
    multibuffer_snapshot: &MultiBufferSnapshot,
    cx: &App,
) -> HighlightedText {
    let edits = edits
        .iter()
        .filter_map(|(anchor, text)| {
            Some((
                multibuffer_snapshot
                    .anchor_range_to_buffer_anchor_range(anchor.clone())?
                    .1,
                text,
            ))
        })
        .collect::<Vec<_>>();

    edit_preview.highlight_edits(current_snapshot, &edits, include_deletions, cx)
}

impl Editor {
    pub fn set_edit_prediction_provider<T>(
        &mut self,
        provider: Option<Entity<T>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) where
        T: EditPredictionDelegate,
    {
        self.edit_prediction_provider = provider.map(|provider| RegisteredEditPredictionDelegate {
            _subscription: cx.observe_in(&provider, window, |this, _, window, cx| {
                if this.focus_handle.is_focused(window) {
                    this.update_visible_edit_prediction(window, cx);
                }
            }),
            provider: Arc::new(provider),
        });
        self.update_edit_prediction_settings(cx);
        self.refresh_edit_prediction(false, false, window, cx);
    }

    pub fn set_edit_predictions_hidden_for_vim_mode(
        &mut self,
        hidden: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if hidden != self.edit_predictions_hidden_for_vim_mode {
            self.edit_predictions_hidden_for_vim_mode = hidden;
            if hidden {
                self.update_visible_edit_prediction(window, cx);
            } else {
                self.refresh_edit_prediction(true, false, window, cx);
            }
        }
    }

    pub fn toggle_edit_predictions(
        &mut self,
        _: &ToggleEditPrediction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.show_edit_predictions_override.is_some() {
            self.set_show_edit_predictions(None, window, cx);
        } else {
            let show_edit_predictions = !self.edit_predictions_enabled();
            self.set_show_edit_predictions(Some(show_edit_predictions), window, cx);
        }
    }

    pub fn set_show_edit_predictions(
        &mut self,
        show_edit_predictions: Option<bool>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_edit_predictions_override = show_edit_predictions;
        self.update_edit_prediction_settings(cx);

        if let Some(false) = show_edit_predictions {
            self.discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);
        } else {
            self.refresh_edit_prediction(false, true, window, cx);
        }
    }

    pub fn refresh_edit_prediction(
        &mut self,
        debounce: bool,
        user_requested: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        if self.leader_id.is_some() {
            self.discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);
            return None;
        }

        let cursor = self.selections.newest_anchor().head();
        let (buffer, cursor_buffer_position) =
            self.buffer.read(cx).text_anchor_for_position(cursor, cx)?;

        if DisableAiSettings::is_ai_disabled_for_buffer(Some(&buffer), cx) {
            return None;
        }

        if !self.edit_predictions_enabled_in_buffer(&buffer, cursor_buffer_position, cx) {
            self.discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);
            return None;
        }

        self.update_visible_edit_prediction(window, cx);

        if !user_requested
            && (!self.should_show_edit_predictions()
                || !self.is_focused(window)
                || buffer.read(cx).is_empty())
        {
            self.discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);
            return None;
        }

        self.edit_prediction_provider()?
            .refresh(buffer, cursor_buffer_position, debounce, cx);
        Some(())
    }

    pub fn edit_predictions_enabled(&self) -> bool {
        match self.edit_prediction_settings {
            EditPredictionSettings::Disabled => false,
            EditPredictionSettings::Enabled { .. } => true,
        }
    }

    pub fn update_edit_prediction_settings(&mut self, cx: &mut Context<Self>) {
        if self.edit_prediction_provider.is_none() {
            self.edit_prediction_settings = EditPredictionSettings::Disabled;
            self.discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);
            return;
        }

        let selection = self.selections.newest_anchor();
        let cursor = selection.head();

        if let Some((buffer, cursor_buffer_position)) =
            self.buffer.read(cx).text_anchor_for_position(cursor, cx)
        {
            if DisableAiSettings::is_ai_disabled_for_buffer(Some(&buffer), cx) {
                self.edit_prediction_settings = EditPredictionSettings::Disabled;
                self.discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);
                return;
            }
            self.edit_prediction_settings =
                self.edit_prediction_settings_at_position(&buffer, cursor_buffer_position, cx);
        }
    }

    pub fn edit_prediction_preview_is_active(&self) -> bool {
        matches!(
            self.edit_prediction_preview,
            EditPredictionPreview::Active { .. }
        )
    }

    pub fn edit_predictions_enabled_at_cursor(&self, cx: &App) -> bool {
        let cursor = self.selections.newest_anchor().head();
        if let Some((buffer, cursor_position)) =
            self.buffer.read(cx).text_anchor_for_position(cursor, cx)
        {
            self.edit_predictions_enabled_in_buffer(&buffer, cursor_position, cx)
        } else {
            false
        }
    }

    pub fn show_edit_prediction(
        &mut self,
        _: &ShowEditPrediction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_active_edit_prediction() {
            self.refresh_edit_prediction(false, true, window, cx);
            return;
        }

        self.update_visible_edit_prediction(window, cx);
    }

    pub fn accept_partial_edit_prediction(
        &mut self,
        granularity: EditPredictionGranularity,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }
        if self.show_edit_predictions_in_menu() {
            self.hide_context_menu(window, cx);
        }

        let Some(active_edit_prediction) = self.active_edit_prediction.as_ref() else {
            return;
        };

        if !matches!(granularity, EditPredictionGranularity::Full) && self.selections.count() != 1 {
            return;
        }

        match &active_edit_prediction.completion {
            EditPrediction::MoveWithin { target, .. } => {
                let target = *target;

                if matches!(granularity, EditPredictionGranularity::Full) {
                    if let Some(position_map) = &self.last_position_map {
                        let target_row = target.to_display_point(&position_map.snapshot).row();
                        let is_visible = position_map.visible_row_range.contains(&target_row);

                        if is_visible || !self.edit_prediction_requires_modifier() {
                            self.unfold_ranges(&[target..target], true, false, cx);
                            self.change_selections(
                                SelectionEffects::scroll(Autoscroll::newest()),
                                window,
                                cx,
                                |selections| {
                                    selections.select_anchor_ranges([target..target]);
                                },
                            );
                            self.clear_row_highlights::<EditPredictionPreview>();
                            self.edit_prediction_preview
                                .set_previous_scroll_position(None);
                        } else {
                            // Highlight and request scroll
                            self.edit_prediction_preview
                                .set_previous_scroll_position(Some(
                                    position_map.snapshot.scroll_anchor,
                                ));
                            self.highlight_rows::<EditPredictionPreview>(
                                target..target,
                                cx.theme().colors().editor_highlighted_line_background,
                                RowHighlightOptions {
                                    autoscroll: true,
                                    ..Default::default()
                                },
                                cx,
                            );
                            self.request_autoscroll(Autoscroll::fit(), cx);
                        }
                    }
                } else {
                    self.change_selections(
                        SelectionEffects::scroll(Autoscroll::newest()),
                        window,
                        cx,
                        |selections| {
                            selections.select_anchor_ranges([target..target]);
                        },
                    );
                }
            }
            EditPrediction::MoveOutside { snapshot, target } => {
                if let Some(workspace) = self.workspace() {
                    Self::open_editor_at_anchor(snapshot, *target, &workspace, window, cx)
                        .detach_and_log_err(cx);
                }
            }
            EditPrediction::Edit {
                edits,
                cursor_position,
                ..
            } => {
                self.report_edit_prediction_event(
                    active_edit_prediction.completion_id.clone(),
                    true,
                    cx,
                );

                match granularity {
                    EditPredictionGranularity::Full => {
                        let transaction_id_prev = self.buffer.read(cx).last_transaction_id(cx);

                        // Compute fallback cursor position BEFORE applying the edit,
                        // so the anchor tracks through the edit correctly
                        let fallback_cursor_target = {
                            let snapshot = self.buffer.read(cx).snapshot(cx);
                            let Some((last_edit_range, _)) = edits.last() else {
                                return;
                            };
                            last_edit_range.end.bias_right(&snapshot)
                        };

                        self.buffer.update(cx, |buffer, cx| {
                            buffer.edit(edits.iter().cloned(), None, cx)
                        });

                        if let Some(provider) = self.edit_prediction_provider() {
                            provider.accept(cx);
                        }

                        // Resolve cursor position after the edit is applied
                        let cursor_target = if let Some((anchor, offset)) = cursor_position {
                            // The anchor tracks through the edit, then we add the offset
                            let snapshot = self.buffer.read(cx).snapshot(cx);
                            let base_offset = anchor.to_offset(&snapshot).0;
                            let target_offset =
                                MultiBufferOffset((base_offset + offset).min(snapshot.len().0));
                            snapshot.anchor_after(target_offset)
                        } else {
                            fallback_cursor_target
                        };

                        self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                            s.select_anchor_ranges([cursor_target..cursor_target]);
                        });

                        let selections = self.selections.disjoint_anchors_arc();
                        if let Some(transaction_id_now) =
                            self.buffer.read(cx).last_transaction_id(cx)
                        {
                            if transaction_id_prev != Some(transaction_id_now) {
                                self.selection_history
                                    .insert_transaction(transaction_id_now, selections);
                            }
                        }

                        self.update_visible_edit_prediction(window, cx);
                        if self.active_edit_prediction.is_none() {
                            self.refresh_edit_prediction(true, true, window, cx);
                        }
                        cx.notify();
                    }
                    _ => {
                        let snapshot = self.buffer.read(cx).snapshot(cx);
                        let cursor_offset = self
                            .selections
                            .newest::<MultiBufferOffset>(&self.display_snapshot(cx))
                            .head();

                        let insertion = edits.iter().find_map(|(range, text)| {
                            let range = range.to_offset(&snapshot);
                            if range.is_empty() && range.start == cursor_offset {
                                Some(text)
                            } else {
                                None
                            }
                        });

                        if let Some(text) = insertion {
                            let text_to_insert = match granularity {
                                EditPredictionGranularity::Word => {
                                    let mut partial = text
                                        .chars()
                                        .by_ref()
                                        .take_while(|c| c.is_alphabetic())
                                        .collect::<String>();
                                    if partial.is_empty() {
                                        partial = text
                                            .chars()
                                            .by_ref()
                                            .take_while(|c| c.is_whitespace() || !c.is_alphabetic())
                                            .collect::<String>();
                                    }
                                    partial
                                }
                                EditPredictionGranularity::Line => {
                                    if let Some(line) = text.split_inclusive('\n').next() {
                                        line.to_string()
                                    } else {
                                        text.to_string()
                                    }
                                }
                                EditPredictionGranularity::Full => unreachable!(),
                            };

                            cx.emit(EditorEvent::InputHandled {
                                utf16_range_to_replace: None,
                                text: text_to_insert.clone().into(),
                            });

                            self.replace_selections(&text_to_insert, None, window, cx, false);
                            self.refresh_edit_prediction(true, true, window, cx);
                            cx.notify();
                        } else {
                            self.accept_partial_edit_prediction(
                                EditPredictionGranularity::Full,
                                window,
                                cx,
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn accept_next_word_edit_prediction(
        &mut self,
        _: &AcceptNextWordEditPrediction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.accept_partial_edit_prediction(EditPredictionGranularity::Word, window, cx);
    }

    pub fn accept_next_line_edit_prediction(
        &mut self,
        _: &AcceptNextLineEditPrediction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.accept_partial_edit_prediction(EditPredictionGranularity::Line, window, cx);
    }

    pub fn accept_edit_prediction(
        &mut self,
        _: &AcceptEditPrediction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.accept_partial_edit_prediction(EditPredictionGranularity::Full, window, cx);
    }

    pub fn has_active_edit_prediction(&self) -> bool {
        self.active_edit_prediction.is_some()
    }

    /// Returns true when we're displaying the edit prediction popover below the cursor
    /// like we are not previewing and the LSP autocomplete menu is visible
    /// or we are in `when_holding_modifier` mode.
    pub fn edit_prediction_visible_in_cursor_popover(&self, has_completion: bool) -> bool {
        if self.edit_prediction_preview_is_active()
            || !self.show_edit_predictions_in_menu()
            || !self.edit_predictions_enabled()
        {
            return false;
        }

        if self.has_visible_completions_menu() {
            return true;
        }

        has_completion && self.edit_prediction_requires_modifier()
    }

    pub fn edit_prediction_provider(&self) -> Option<Arc<dyn EditPredictionDelegateHandle>> {
        Some(self.edit_prediction_provider.as_ref()?.provider.clone())
    }

    pub(super) fn preview_edit_prediction_keystroke(
        &self,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<gpui::KeybindingKeystroke> {
        let key_context = self.key_context_internal(true, window, cx);
        let bindings = window.bindings_for_action_in_context(&AcceptEditPrediction, key_context);
        bindings
            .into_iter()
            .rev()
            .find_map(|binding| match binding.keystrokes() {
                [keystroke, ..] if keystroke.modifiers().modified() => Some(keystroke.clone()),
                _ => None,
            })
    }

    pub(super) fn edit_prediction_keybind_display(
        &self,
        surface: EditPredictionKeybindSurface,
        window: &mut Window,
        cx: &mut App,
    ) -> EditPredictionKeybindDisplay {
        let accept_keystroke =
            self.accept_edit_prediction_keystroke(EditPredictionGranularity::Full, window, cx);
        let preview_keystroke = self.preview_edit_prediction_keystroke(window, cx);

        let action = match surface {
            EditPredictionKeybindSurface::Inline
            | EditPredictionKeybindSurface::CursorPopoverCompact => {
                if self.edit_prediction_requires_modifier() {
                    EditPredictionKeybindAction::Preview
                } else {
                    EditPredictionKeybindAction::Accept
                }
            }
            EditPredictionKeybindSurface::CursorPopoverExpanded => self
                .active_edit_prediction
                .as_ref()
                .filter(|completion| {
                    self.edit_prediction_cursor_popover_prefers_preview(completion, cx)
                })
                .map_or(EditPredictionKeybindAction::Accept, |_| {
                    EditPredictionKeybindAction::Preview
                }),
        };
        #[cfg(test)]
        let preview_copy = preview_keystroke.clone();
        #[cfg(test)]
        let accept_copy = accept_keystroke.clone();

        let displayed_keystroke = match surface {
            EditPredictionKeybindSurface::Inline => match action {
                EditPredictionKeybindAction::Accept => accept_keystroke,
                EditPredictionKeybindAction::Preview => preview_keystroke,
            },
            EditPredictionKeybindSurface::CursorPopoverCompact
            | EditPredictionKeybindSurface::CursorPopoverExpanded => match action {
                EditPredictionKeybindAction::Accept => accept_keystroke,
                EditPredictionKeybindAction::Preview => {
                    preview_keystroke.or_else(|| accept_keystroke.clone())
                }
            },
        };

        let missing_accept_keystroke = displayed_keystroke.is_none();

        EditPredictionKeybindDisplay {
            #[cfg(test)]
            accept_keystroke: accept_copy,
            #[cfg(test)]
            preview_keystroke: preview_copy,
            displayed_keystroke,
            action,
            missing_accept_keystroke,
            show_hold_label: matches!(surface, EditPredictionKeybindSurface::CursorPopoverCompact)
                && self.edit_prediction_preview.released_too_fast(),
        }
    }

    pub(super) fn show_edit_predictions_in_menu(&self) -> bool {
        match self.edit_prediction_settings {
            EditPredictionSettings::Disabled => false,
            EditPredictionSettings::Enabled { show_in_menu, .. } => show_in_menu,
        }
    }

    pub(super) fn edit_prediction_requires_modifier(&self) -> bool {
        match self.edit_prediction_settings {
            EditPredictionSettings::Disabled => false,
            EditPredictionSettings::Enabled {
                preview_requires_modifier,
                ..
            } => preview_requires_modifier,
        }
    }

    pub(super) fn discard_edit_prediction(
        &mut self,
        reason: EditPredictionDiscardReason,
        cx: &mut Context<Self>,
    ) -> bool {
        if reason == EditPredictionDiscardReason::Rejected {
            let completion_id = self
                .active_edit_prediction
                .as_ref()
                .and_then(|active_completion| active_completion.completion_id.clone());

            self.report_edit_prediction_event(completion_id, false, cx);
        }

        if let Some(provider) = self.edit_prediction_provider() {
            provider.discard(reason, cx);
        }

        self.take_active_edit_prediction(reason == EditPredictionDiscardReason::Ignored, cx)
    }

    pub(super) fn take_active_edit_prediction(
        &mut self,
        preserve_stale_in_menu: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(active_edit_prediction) = self.active_edit_prediction.take() else {
            if !preserve_stale_in_menu {
                self.stale_edit_prediction_in_menu = None;
            }
            return false;
        };

        self.splice_inlays(&active_edit_prediction.inlay_ids, Default::default(), cx);
        self.clear_highlights(HighlightKey::EditPredictionHighlight, cx);
        self.stale_edit_prediction_in_menu =
            preserve_stale_in_menu.then_some(active_edit_prediction);
        true
    }

    pub(super) fn update_edit_prediction_preview(
        &mut self,
        modifiers: &Modifiers,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let modifiers_held = self.edit_prediction_preview_modifiers_held(modifiers, window, cx);

        if modifiers_held {
            if matches!(
                self.edit_prediction_preview,
                EditPredictionPreview::Inactive { .. }
            ) {
                self.edit_prediction_preview = EditPredictionPreview::Active {
                    previous_scroll_position: None,
                    since: Instant::now(),
                };

                self.update_visible_edit_prediction(window, cx);
                cx.notify();
            }
        } else if let EditPredictionPreview::Active {
            previous_scroll_position,
            since,
        } = self.edit_prediction_preview
        {
            if let (Some(previous_scroll_position), Some(position_map)) =
                (previous_scroll_position, self.last_position_map.as_ref())
            {
                self.set_scroll_position(
                    previous_scroll_position
                        .scroll_position(&position_map.snapshot.display_snapshot),
                    window,
                    cx,
                );
            }

            self.edit_prediction_preview = EditPredictionPreview::Inactive {
                released_too_fast: since.elapsed() < Duration::from_millis(200),
            };
            self.clear_row_highlights::<EditPredictionPreview>();
            self.update_visible_edit_prediction(window, cx);
            cx.notify();
        }
    }

    pub(super) fn update_visible_edit_prediction(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        if self.ime_transaction.is_some() {
            self.discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);
            return None;
        }

        let selection = self.selections.newest_anchor();
        let multibuffer = self.buffer.read(cx).snapshot(cx);
        let cursor = selection.head();
        let (cursor_text_anchor, _) = multibuffer.anchor_to_buffer_anchor(cursor)?;
        let buffer = self.buffer.read(cx).buffer(cursor_text_anchor.buffer_id)?;

        // Check project-level disable_ai setting for the current buffer
        if DisableAiSettings::is_ai_disabled_for_buffer(Some(&buffer), cx) {
            return None;
        }
        let offset_selection = selection.map(|endpoint| endpoint.to_offset(&multibuffer));

        let show_in_menu = self.show_edit_predictions_in_menu();
        let completions_menu_has_precedence = !show_in_menu
            && (self.context_menu.borrow().is_some()
                || (!self.completion_tasks.is_empty() && !self.has_active_edit_prediction()));

        if completions_menu_has_precedence
            || !offset_selection.is_empty()
            || self
                .active_edit_prediction
                .as_ref()
                .is_some_and(|completion| {
                    let Some(invalidation_range) = completion.invalidation_range.as_ref() else {
                        return false;
                    };
                    let invalidation_range = invalidation_range.to_offset(&multibuffer);
                    let invalidation_range = invalidation_range.start..=invalidation_range.end;
                    !invalidation_range.contains(&offset_selection.head())
                })
        {
            self.discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);
            return None;
        }

        self.take_active_edit_prediction(true, cx);
        let Some(provider) = self.edit_prediction_provider() else {
            self.edit_prediction_settings = EditPredictionSettings::Disabled;
            return None;
        };

        self.edit_prediction_settings =
            self.edit_prediction_settings_at_position(&buffer, cursor_text_anchor, cx);

        self.in_leading_whitespace = multibuffer.is_line_whitespace_upto(cursor);

        if self.in_leading_whitespace {
            let cursor_point = cursor.to_point(&multibuffer);
            let mut suggested_indent = None;
            multibuffer.suggested_indents_callback(
                cursor_point.row..cursor_point.row + 1,
                &mut |_, indent| {
                    suggested_indent = Some(indent);
                    ControlFlow::Break(())
                },
                cx,
            );

            if let Some(indent) = suggested_indent
                && indent.len == cursor_point.column
            {
                self.in_leading_whitespace = false;
            }
        }

        let edit_prediction = provider.suggest(&buffer, cursor_text_anchor, cx)?;

        let (completion_id, edits, predicted_cursor_position, edit_preview) = match edit_prediction
        {
            edit_prediction_types::EditPrediction::Local {
                id,
                edits,
                cursor_position,
                edit_preview,
            } => (id, edits, cursor_position, edit_preview),
            edit_prediction_types::EditPrediction::Jump {
                id,
                snapshot,
                target,
            } => {
                if let Some(provider) = &self.edit_prediction_provider {
                    provider.provider.did_show(SuggestionDisplayType::Jump, cx);
                }
                self.stale_edit_prediction_in_menu = None;
                self.active_edit_prediction = Some(EditPredictionState {
                    inlay_ids: vec![],
                    completion: EditPrediction::MoveOutside { snapshot, target },
                    completion_id: id,
                    invalidation_range: None,
                });
                cx.notify();
                return Some(());
            }
        };

        let edits = edits
            .into_iter()
            .flat_map(|(range, new_text)| {
                Some((
                    multibuffer.buffer_anchor_range_to_anchor_range(range)?,
                    new_text,
                ))
            })
            .collect::<Vec<_>>();
        if edits.is_empty() {
            return None;
        }

        let cursor_position = predicted_cursor_position.and_then(|predicted| {
            let anchor = multibuffer.anchor_in_excerpt(predicted.anchor)?;
            Some((anchor, predicted.offset))
        });

        let Some((first_edit_range, _)) = edits.first() else {
            return None;
        };
        let Some((last_edit_range, _)) = edits.last() else {
            return None;
        };

        let first_edit_start = first_edit_range.start;
        let first_edit_start_point = first_edit_start.to_point(&multibuffer);
        let edit_start_row = first_edit_start_point.row.saturating_sub(2);

        let last_edit_end = last_edit_range.end;
        let last_edit_end_point = last_edit_end.to_point(&multibuffer);
        let edit_end_row = cmp::min(multibuffer.max_point().row, last_edit_end_point.row + 2);

        let cursor_row = cursor.to_point(&multibuffer).row;

        let snapshot = multibuffer
            .buffer_for_id(cursor_text_anchor.buffer_id)
            .cloned()?;

        let mut inlay_ids = Vec::new();
        let invalidation_row_range;
        let move_invalidation_row_range = if cursor_row < edit_start_row {
            Some(cursor_row..edit_end_row)
        } else if cursor_row > edit_end_row {
            Some(edit_start_row..cursor_row)
        } else {
            None
        };
        let supports_jump = self
            .edit_prediction_provider
            .as_ref()
            .map(|provider| provider.provider.supports_jump_to_edit())
            .unwrap_or(true);

        let is_move = supports_jump
            && (move_invalidation_row_range.is_some() || self.edit_predictions_hidden_for_vim_mode);
        let completion = if is_move {
            if let Some(provider) = &self.edit_prediction_provider {
                provider.provider.did_show(SuggestionDisplayType::Jump, cx);
            }
            invalidation_row_range =
                move_invalidation_row_range.unwrap_or(edit_start_row..edit_end_row);

            let (_, snapshot) = multibuffer.anchor_to_buffer_anchor(first_edit_start)?;

            EditPrediction::MoveWithin {
                target: first_edit_start,
                snapshot: snapshot.clone(),
            }
        } else {
            let show_completions_in_menu = self.has_visible_completions_menu();
            let show_completions_in_buffer = !self.edit_prediction_visible_in_cursor_popover(true)
                && !self.edit_predictions_hidden_for_vim_mode;

            let display_mode = if all_edits_insertions_or_deletions(&edits, &multibuffer) {
                if provider.show_tab_accept_marker() {
                    EditDisplayMode::TabAccept
                } else {
                    EditDisplayMode::Inline
                }
            } else {
                EditDisplayMode::DiffPopover
            };

            let report_shown = match display_mode {
                EditDisplayMode::DiffPopover | EditDisplayMode::Inline => {
                    show_completions_in_buffer || show_completions_in_menu
                }
                EditDisplayMode::TabAccept => {
                    show_completions_in_menu || self.edit_prediction_preview_is_active()
                }
            };

            if report_shown && let Some(provider) = &self.edit_prediction_provider {
                let suggestion_display_type = match display_mode {
                    EditDisplayMode::DiffPopover => SuggestionDisplayType::DiffPopover,
                    EditDisplayMode::Inline | EditDisplayMode::TabAccept => {
                        SuggestionDisplayType::GhostText
                    }
                };
                provider.provider.did_show(suggestion_display_type, cx);
            }

            if show_completions_in_buffer {
                if edits
                    .iter()
                    .all(|(range, _)| range.to_offset(&multibuffer).is_empty())
                {
                    let mut inlays = Vec::new();
                    for (range, new_text) in &edits {
                        let inlay = Inlay::edit_prediction(
                            post_inc(&mut self.next_inlay_id),
                            range.start,
                            new_text.as_ref(),
                        );
                        inlay_ids.push(inlay.id);
                        inlays.push(inlay);
                    }

                    self.splice_inlays(&[], inlays, cx);
                } else {
                    let background_color = cx.theme().status().deleted_background;
                    self.highlight_text(
                        HighlightKey::EditPredictionHighlight,
                        edits.iter().map(|(range, _)| range.clone()).collect(),
                        HighlightStyle {
                            background_color: Some(background_color),
                            ..Default::default()
                        },
                        cx,
                    );
                }
            }

            invalidation_row_range = edit_start_row..edit_end_row;

            EditPrediction::Edit {
                edits,
                cursor_position,
                edit_preview,
                display_mode,
                snapshot,
            }
        };

        let invalidation_range = multibuffer
            .anchor_before(Point::new(invalidation_row_range.start, 0))
            ..multibuffer.anchor_after(Point::new(
                invalidation_row_range.end,
                multibuffer.line_len(MultiBufferRow(invalidation_row_range.end)),
            ));

        self.stale_edit_prediction_in_menu = None;
        self.active_edit_prediction = Some(EditPredictionState {
            inlay_ids,
            completion,
            completion_id,
            invalidation_range: Some(invalidation_range),
        });

        cx.notify();

        Some(())
    }

    pub(super) fn render_edit_prediction_popover(
        &mut self,
        text_bounds: &Bounds<Pixels>,
        content_origin: gpui::Point<Pixels>,
        right_margin: Pixels,
        editor_snapshot: &EditorSnapshot,
        visible_row_range: Range<DisplayRow>,
        scroll_top: ScrollOffset,
        scroll_bottom: ScrollOffset,
        line_layouts: &[LineWithInvisibles],
        line_height: Pixels,
        scroll_position: gpui::Point<ScrollOffset>,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        newest_selection_head: Option<DisplayPoint>,
        editor_width: Pixels,
        style: &EditorStyle,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<(AnyElement, gpui::Point<Pixels>)> {
        if self.mode().is_minimap() {
            return None;
        }
        let active_edit_prediction = self.active_edit_prediction.as_ref()?;

        if self.edit_prediction_visible_in_cursor_popover(true) {
            return None;
        }

        match &active_edit_prediction.completion {
            EditPrediction::MoveWithin { target, .. } => {
                let target_display_point = target.to_display_point(editor_snapshot);

                if self.edit_prediction_requires_modifier() {
                    if !self.edit_prediction_preview_is_active() {
                        return None;
                    }

                    self.render_edit_prediction_modifier_jump_popover(
                        text_bounds,
                        content_origin,
                        visible_row_range,
                        line_layouts,
                        line_height,
                        scroll_pixel_position,
                        newest_selection_head,
                        target_display_point,
                        window,
                        cx,
                    )
                } else {
                    self.render_edit_prediction_eager_jump_popover(
                        text_bounds,
                        content_origin,
                        editor_snapshot,
                        visible_row_range,
                        scroll_top,
                        scroll_bottom,
                        line_height,
                        scroll_pixel_position,
                        target_display_point,
                        editor_width,
                        window,
                        cx,
                    )
                }
            }
            EditPrediction::Edit {
                display_mode: EditDisplayMode::Inline,
                ..
            } => None,
            EditPrediction::Edit {
                display_mode: EditDisplayMode::TabAccept,
                edits,
                ..
            } => {
                let range = &edits.first()?.0;
                let target_display_point = range.end.to_display_point(editor_snapshot);

                self.render_edit_prediction_end_of_line_popover(
                    "Accept",
                    editor_snapshot,
                    visible_row_range,
                    target_display_point,
                    line_height,
                    scroll_pixel_position,
                    content_origin,
                    editor_width,
                    window,
                    cx,
                )
            }
            EditPrediction::Edit {
                edits,
                edit_preview,
                display_mode: EditDisplayMode::DiffPopover,
                snapshot,
                ..
            } => self.render_edit_prediction_diff_popover(
                text_bounds,
                content_origin,
                right_margin,
                editor_snapshot,
                visible_row_range,
                line_layouts,
                line_height,
                scroll_position,
                scroll_pixel_position,
                newest_selection_head,
                editor_width,
                style,
                edits,
                edit_preview,
                snapshot,
                window,
                cx,
            ),
            EditPrediction::MoveOutside { snapshot, .. } => {
                let mut element = self
                    .render_edit_prediction_jump_outside_popover(snapshot, window, cx)
                    .into_any();

                let size = element.layout_as_root(AvailableSpace::min_size(), window, cx);
                let origin_x = text_bounds.size.width - size.width - px(30.);
                let origin = text_bounds.origin + gpui::Point::new(origin_x, px(16.));
                element.prepaint_at(origin, window, cx);

                Some((element, origin))
            }
        }
    }

    pub(super) fn edit_prediction_cursor_popover_height(&self) -> Pixels {
        px(30.)
    }

    pub(super) fn render_edit_prediction_cursor_popover(
        &self,
        min_width: Pixels,
        max_width: Pixels,
        cursor_point: Point,
        style: &EditorStyle,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Option<AnyElement> {
        let provider = self.edit_prediction_provider.as_ref()?;
        let icons = Self::get_prediction_provider_icons(&self.edit_prediction_provider, cx);

        let is_refreshing = provider.provider.is_refreshing(cx);

        fn pending_completion_container(icon: IconName) -> Div {
            h_flex().h_full().flex_1().gap_2().child(Icon::new(icon))
        }

        let completion = match &self.active_edit_prediction {
            Some(prediction) => {
                if !self.has_visible_completions_menu() {
                    const RADIUS: Pixels = px(6.);
                    const BORDER_WIDTH: Pixels = px(1.);
                    let keybind_display = self.edit_prediction_keybind_display(
                        EditPredictionKeybindSurface::CursorPopoverCompact,
                        window,
                        cx,
                    );

                    return Some(
                        h_flex()
                            .elevation_2(cx)
                            .border(BORDER_WIDTH)
                            .border_color(cx.theme().colors().border)
                            .when(keybind_display.missing_accept_keystroke, |el| {
                                el.border_color(cx.theme().status().error)
                            })
                            .rounded(RADIUS)
                            .rounded_tl(px(0.))
                            .overflow_hidden()
                            .child(div().px_1p5().child(match &prediction.completion {
                                EditPrediction::MoveWithin { target, snapshot } => {
                                    use text::ToPoint as _;
                                    if target.text_anchor_in(&snapshot).to_point(snapshot).row
                                        > cursor_point.row
                                    {
                                        Icon::new(icons.down)
                                    } else {
                                        Icon::new(icons.up)
                                    }
                                }
                                EditPrediction::MoveOutside { .. } => {
                                    // TODO [zeta2] custom icon for external jump?
                                    Icon::new(icons.base)
                                }
                                EditPrediction::Edit { .. } => Icon::new(icons.base),
                            }))
                            .child(
                                h_flex()
                                    .gap_1()
                                    .py_1()
                                    .px_2()
                                    .rounded_r(RADIUS - BORDER_WIDTH)
                                    .border_l_1()
                                    .border_color(cx.theme().colors().border)
                                    .bg(Self::edit_prediction_line_popover_bg_color(cx))
                                    .when(keybind_display.show_hold_label, |el| {
                                        el.child(
                                            Label::new("Hold")
                                                .size(LabelSize::Small)
                                                .when(
                                                    keybind_display.missing_accept_keystroke,
                                                    |el| el.strikethrough(),
                                                )
                                                .line_height_style(LineHeightStyle::UiLabel),
                                        )
                                    })
                                    .id("edit_prediction_cursor_popover_keybind")
                                    .when(keybind_display.missing_accept_keystroke, |el| {
                                        let status_colors = cx.theme().status();

                                        el.bg(status_colors.error_background)
                                            .border_color(status_colors.error.opacity(0.6))
                                            .child(Icon::new(IconName::Info).color(Color::Error))
                                            .cursor_default()
                                            .hoverable_tooltip(move |_window, cx| {
                                                cx.new(|_| MissingEditPredictionKeybindingTooltip)
                                                    .into()
                                            })
                                    })
                                    .when_some(
                                        keybind_display.displayed_keystroke.as_ref(),
                                        |el, compact_keystroke| {
                                            el.child(self.render_edit_prediction_popover_keystroke(
                                                compact_keystroke,
                                                Color::Default,
                                                cx,
                                            ))
                                        },
                                    ),
                            )
                            .into_any(),
                    );
                }

                self.render_edit_prediction_cursor_popover_preview(
                    prediction,
                    cursor_point,
                    style,
                    cx,
                )?
            }

            None if is_refreshing => match &self.stale_edit_prediction_in_menu {
                Some(stale_completion) => self.render_edit_prediction_cursor_popover_preview(
                    stale_completion,
                    cursor_point,
                    style,
                    cx,
                )?,

                None => pending_completion_container(icons.base)
                    .child(Label::new("...").size(LabelSize::Small)),
            },

            None => pending_completion_container(icons.base)
                .child(Label::new("...").size(LabelSize::Small)),
        };

        let completion = if is_refreshing || self.active_edit_prediction.is_none() {
            completion
                .with_animation(
                    "loading-completion",
                    Animation::new(Duration::from_secs(2))
                        .repeat()
                        .with_easing(pulsating_between(0.4, 0.8)),
                    |label, delta| label.opacity(delta),
                )
                .into_any_element()
        } else {
            completion.into_any_element()
        };

        let has_completion = self.active_edit_prediction.is_some();
        let keybind_display = self.edit_prediction_keybind_display(
            EditPredictionKeybindSurface::CursorPopoverExpanded,
            window,
            cx,
        );

        Some(
            h_flex()
                .min_w(min_width)
                .max_w(max_width)
                .flex_1()
                .elevation_2(cx)
                .border_color(cx.theme().colors().border)
                .child(
                    div()
                        .flex_1()
                        .py_1()
                        .px_2()
                        .overflow_hidden()
                        .child(completion),
                )
                .when_some(
                    keybind_display.displayed_keystroke.as_ref(),
                    |el, keystroke| {
                        let key_color = if !has_completion {
                            Color::Muted
                        } else {
                            Color::Default
                        };

                        if keybind_display.action == EditPredictionKeybindAction::Preview {
                            el.child(
                                h_flex()
                                    .h_full()
                                    .border_l_1()
                                    .rounded_r_lg()
                                    .border_color(cx.theme().colors().border)
                                    .bg(Self::edit_prediction_line_popover_bg_color(cx))
                                    .gap_1()
                                    .py_1()
                                    .px_2()
                                    .child(self.render_edit_prediction_popover_keystroke(
                                        keystroke, key_color, cx,
                                    ))
                                    .child(Label::new("Preview").into_any_element())
                                    .opacity(if has_completion { 1.0 } else { 0.4 }),
                            )
                        } else {
                            el.child(
                                h_flex()
                                    .h_full()
                                    .border_l_1()
                                    .rounded_r_lg()
                                    .border_color(cx.theme().colors().border)
                                    .bg(Self::edit_prediction_line_popover_bg_color(cx))
                                    .gap_1()
                                    .py_1()
                                    .px_2()
                                    .child(self.render_edit_prediction_popover_keystroke(
                                        keystroke, key_color, cx,
                                    ))
                                    .opacity(if has_completion { 1.0 } else { 0.4 }),
                            )
                        }
                    },
                )
                .into_any(),
        )
    }

    fn accept_edit_prediction_keystroke(
        &self,
        granularity: EditPredictionGranularity,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<gpui::KeybindingKeystroke> {
        let key_context = self.key_context_internal(true, window, cx);

        let bindings =
            match granularity {
                EditPredictionGranularity::Word => window
                    .bindings_for_action_in_context(&AcceptNextWordEditPrediction, key_context),
                EditPredictionGranularity::Line => window
                    .bindings_for_action_in_context(&AcceptNextLineEditPrediction, key_context),
                EditPredictionGranularity::Full => {
                    window.bindings_for_action_in_context(&AcceptEditPrediction, key_context)
                }
            };

        bindings
            .into_iter()
            .rev()
            .find_map(|binding| match binding.keystrokes() {
                [keystroke, ..] => Some(keystroke.clone()),
                _ => None,
            })
    }

    fn edit_prediction_preview_modifiers_held(
        &self,
        modifiers: &Modifiers,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let can_supersede_active_menu =
            self.context_menu.borrow().as_ref().is_none_or(|menu| {
                !menu.visible() || matches!(menu, CodeContextMenu::Completions(_))
            });

        if !can_supersede_active_menu {
            return false;
        }

        let key_context = self.key_context_internal(true, window, cx);
        let actions: [&dyn Action; 3] = [
            &AcceptEditPrediction,
            &AcceptNextWordEditPrediction,
            &AcceptNextLineEditPrediction,
        ];

        actions.into_iter().any(|action| {
            window
                .bindings_for_action_in_context(action, key_context.clone())
                .into_iter()
                .rev()
                .any(|binding| {
                    binding.keystrokes().first().is_some_and(|keystroke| {
                        keystroke.modifiers().modified() && keystroke.modifiers() == modifiers
                    })
                })
        })
    }

    fn edit_prediction_cursor_popover_prefers_preview(
        &self,
        completion: &EditPredictionState,
        cx: &App,
    ) -> bool {
        let multibuffer_snapshot = self.buffer.read(cx).snapshot(cx);

        match &completion.completion {
            EditPrediction::Edit {
                edits, snapshot, ..
            } => {
                let mut start_row: Option<u32> = None;
                let mut end_row: Option<u32> = None;

                for (range, text) in edits {
                    let Some((_, range)) =
                        multibuffer_snapshot.anchor_range_to_buffer_anchor_range(range.clone())
                    else {
                        continue;
                    };
                    let edit_start_row = range.start.to_point(snapshot).row;
                    let old_end_row = range.end.to_point(snapshot).row;
                    let inserted_newline_count = text
                        .as_ref()
                        .chars()
                        .filter(|character| *character == '\n')
                        .count() as u32;
                    let deleted_newline_count = old_end_row - edit_start_row;
                    let preview_end_row = edit_start_row + inserted_newline_count;

                    start_row =
                        Some(start_row.map_or(edit_start_row, |row| row.min(edit_start_row)));
                    end_row = Some(end_row.map_or(preview_end_row, |row| row.max(preview_end_row)));

                    if deleted_newline_count > 1 {
                        end_row = Some(end_row.map_or(old_end_row, |row| row.max(old_end_row)));
                    }
                }

                start_row
                    .zip(end_row)
                    .is_some_and(|(start_row, end_row)| end_row > start_row)
            }
            EditPrediction::MoveWithin { .. } | EditPrediction::MoveOutside { .. } => false,
        }
    }

    fn edit_predictions_disabled_in_scope(
        &self,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        cx: &App,
    ) -> bool {
        let snapshot = buffer.read(cx).snapshot();
        let settings = snapshot.settings_at(buffer_position, cx);

        let Some(scope) = snapshot.language_scope_at(buffer_position) else {
            return false;
        };

        scope.override_name().is_some_and(|scope_name| {
            settings
                .edit_predictions_disabled_in
                .iter()
                .any(|s| s == scope_name)
        })
    }

    fn edit_prediction_settings_at_position(
        &self,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        cx: &App,
    ) -> EditPredictionSettings {
        if !self.mode.is_full()
            || !self.show_edit_predictions_override.unwrap_or(true)
            || self.edit_predictions_disabled_in_scope(buffer, buffer_position, cx)
        {
            return EditPredictionSettings::Disabled;
        }

        if !LanguageSettings::for_buffer(&buffer.read(cx), cx).show_edit_predictions {
            return EditPredictionSettings::Disabled;
        };

        let by_provider = matches!(
            self.menu_edit_predictions_policy,
            MenuEditPredictionsPolicy::ByProvider
        );

        let show_in_menu = by_provider
            && self
                .edit_prediction_provider
                .as_ref()
                .is_some_and(|provider| provider.provider.show_predictions_in_menu());

        let file = buffer.read(cx).file();
        let preview_requires_modifier =
            all_language_settings(file, cx).edit_predictions_mode() == EditPredictionsMode::Subtle;

        EditPredictionSettings::Enabled {
            show_in_menu,
            preview_requires_modifier,
        }
    }

    fn should_show_edit_predictions(&self) -> bool {
        self.snippet_stack.is_empty() && self.edit_predictions_enabled()
    }

    fn edit_predictions_enabled_in_buffer(
        &self,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        cx: &App,
    ) -> bool {
        maybe!({
            if self.read_only(cx) || self.leader_id.is_some() {
                return Some(false);
            }
            let provider = self.edit_prediction_provider()?;
            if !provider.is_enabled(buffer, buffer_position, cx) {
                return Some(false);
            }
            let buffer = buffer.read(cx);
            let Some(file) = buffer.file() else {
                return Some(true);
            };
            let settings = all_language_settings(Some(file), cx);
            Some(settings.edit_predictions_enabled_for_file(file, cx))
        })
        .unwrap_or(false)
    }

    fn report_edit_prediction_event(&self, id: Option<SharedString>, accepted: bool, cx: &App) {
        let Some(provider) = self.edit_prediction_provider() else {
            return;
        };

        let buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let Some((position, _)) =
            buffer_snapshot.anchor_to_buffer_anchor(self.selections.newest_anchor().head())
        else {
            return;
        };
        let Some(buffer) = self.buffer.read(cx).buffer(position.buffer_id) else {
            return;
        };

        let extension = buffer
            .read(cx)
            .file()
            .and_then(|file| Some(file.path().extension()?.to_string()));

        let event_type = match accepted {
            true => "Edit Prediction Accepted",
            false => "Edit Prediction Discarded",
        };
        telemetry::event!(
            event_type,
            provider = provider.name(),
            prediction_id = id,
            suggestion_accepted = accepted,
            file_extension = extension,
        );
    }

    fn open_editor_at_anchor(
        snapshot: &language::BufferSnapshot,
        target: language::Anchor,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        workspace.update(cx, |workspace, cx| {
            let path = snapshot.file().map(|file| file.full_path(cx));
            let Some(path) =
                path.and_then(|path| workspace.project().read(cx).find_project_path(path, cx))
            else {
                return Task::ready(Err(anyhow::anyhow!("Project path not found")));
            };
            let target = text::ToPoint::to_point(&target, snapshot);
            let item = workspace.open_path(path, None, true, window, cx);
            window.spawn(cx, async move |cx| {
                let Some(editor) = item.await?.downcast::<Editor>() else {
                    return Ok(());
                };
                editor
                    .update_in(cx, |editor, window, cx| {
                        editor.go_to_singleton_buffer_point(target, window, cx);
                    })
                    .ok();
                anyhow::Ok(())
            })
        })
    }

    const EDIT_PREDICTION_POPOVER_PADDING_X: Pixels = px(24.);

    const EDIT_PREDICTION_POPOVER_PADDING_Y: Pixels = px(2.);

    fn render_edit_prediction_modifier_jump_popover(
        &mut self,
        text_bounds: &Bounds<Pixels>,
        content_origin: gpui::Point<Pixels>,
        visible_row_range: Range<DisplayRow>,
        line_layouts: &[LineWithInvisibles],
        line_height: Pixels,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        newest_selection_head: Option<DisplayPoint>,
        target_display_point: DisplayPoint,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<(AnyElement, gpui::Point<Pixels>)> {
        let scrolled_content_origin =
            content_origin - gpui::Point::new(scroll_pixel_position.x.into(), Pixels::ZERO);

        const SCROLL_PADDING_Y: Pixels = px(12.);

        if target_display_point.row() < visible_row_range.start {
            return self.render_edit_prediction_scroll_popover(
                &|_| SCROLL_PADDING_Y,
                IconName::ArrowUp,
                visible_row_range,
                line_layouts,
                newest_selection_head,
                scrolled_content_origin,
                window,
                cx,
            );
        } else if target_display_point.row() >= visible_row_range.end {
            return self.render_edit_prediction_scroll_popover(
                &|size| text_bounds.size.height - size.height - SCROLL_PADDING_Y,
                IconName::ArrowDown,
                visible_row_range,
                line_layouts,
                newest_selection_head,
                scrolled_content_origin,
                window,
                cx,
            );
        }

        const POLE_WIDTH: Pixels = px(2.);

        let line_layout =
            line_layouts.get(target_display_point.row().minus(visible_row_range.start) as usize)?;
        let target_column = target_display_point.column() as usize;

        let target_x = line_layout.x_for_index(target_column);
        let target_y = (target_display_point.row().as_f64() * f64::from(line_height))
            - scroll_pixel_position.y;

        let flag_on_right = target_x < text_bounds.size.width / 2.;

        let mut border_color = Self::edit_prediction_callout_popover_border_color(cx);
        border_color.l += 0.001;

        let mut element = v_flex()
            .items_end()
            .when(flag_on_right, |el| el.items_start())
            .child(if flag_on_right {
                self.render_edit_prediction_line_popover("Jump", None, window, cx)
                    .rounded_bl(px(0.))
                    .rounded_tl(px(0.))
                    .border_l_2()
                    .border_color(border_color)
            } else {
                self.render_edit_prediction_line_popover("Jump", None, window, cx)
                    .rounded_br(px(0.))
                    .rounded_tr(px(0.))
                    .border_r_2()
                    .border_color(border_color)
            })
            .child(div().w(POLE_WIDTH).bg(border_color).h(line_height))
            .into_any();

        let size = element.layout_as_root(AvailableSpace::min_size(), window, cx);

        let mut origin = scrolled_content_origin + point(target_x, target_y.into())
            - point(
                if flag_on_right {
                    POLE_WIDTH
                } else {
                    size.width - POLE_WIDTH
                },
                size.height - line_height,
            );

        origin.x = origin.x.max(content_origin.x);

        element.prepaint_at(origin, window, cx);

        Some((element, origin))
    }

    fn render_edit_prediction_scroll_popover(
        &mut self,
        to_y: &dyn Fn(Size<Pixels>) -> Pixels,
        scroll_icon: IconName,
        visible_row_range: Range<DisplayRow>,
        line_layouts: &[LineWithInvisibles],
        newest_selection_head: Option<DisplayPoint>,
        scrolled_content_origin: gpui::Point<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<(AnyElement, gpui::Point<Pixels>)> {
        let mut element = self
            .render_edit_prediction_line_popover("Scroll", Some(scroll_icon), window, cx)
            .into_any();

        let size = element.layout_as_root(AvailableSpace::min_size(), window, cx);

        let cursor = newest_selection_head?;
        let cursor_row_layout =
            line_layouts.get(cursor.row().minus(visible_row_range.start) as usize)?;
        let cursor_column = cursor.column() as usize;

        let cursor_character_x = cursor_row_layout.x_for_index(cursor_column);

        let origin = scrolled_content_origin + point(cursor_character_x, to_y(size));

        element.prepaint_at(origin, window, cx);
        Some((element, origin))
    }

    fn render_edit_prediction_eager_jump_popover(
        &mut self,
        text_bounds: &Bounds<Pixels>,
        content_origin: gpui::Point<Pixels>,
        editor_snapshot: &EditorSnapshot,
        visible_row_range: Range<DisplayRow>,
        scroll_top: ScrollOffset,
        scroll_bottom: ScrollOffset,
        line_height: Pixels,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        target_display_point: DisplayPoint,
        editor_width: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<(AnyElement, gpui::Point<Pixels>)> {
        if target_display_point.row().as_f64() < scroll_top {
            let mut element = self
                .render_edit_prediction_line_popover(
                    "Jump to Edit",
                    Some(IconName::ArrowUp),
                    window,
                    cx,
                )
                .into_any();

            let size = element.layout_as_root(AvailableSpace::min_size(), window, cx);
            let offset = point(
                (text_bounds.size.width - size.width) / 2.,
                Self::EDIT_PREDICTION_POPOVER_PADDING_Y,
            );

            let origin = text_bounds.origin + offset;
            element.prepaint_at(origin, window, cx);
            Some((element, origin))
        } else if (target_display_point.row().as_f64() + 1.) > scroll_bottom {
            let mut element = self
                .render_edit_prediction_line_popover(
                    "Jump to Edit",
                    Some(IconName::ArrowDown),
                    window,
                    cx,
                )
                .into_any();

            let size = element.layout_as_root(AvailableSpace::min_size(), window, cx);
            let offset = point(
                (text_bounds.size.width - size.width) / 2.,
                text_bounds.size.height - size.height - Self::EDIT_PREDICTION_POPOVER_PADDING_Y,
            );

            let origin = text_bounds.origin + offset;
            element.prepaint_at(origin, window, cx);
            Some((element, origin))
        } else {
            self.render_edit_prediction_end_of_line_popover(
                "Jump to Edit",
                editor_snapshot,
                visible_row_range,
                target_display_point,
                line_height,
                scroll_pixel_position,
                content_origin,
                editor_width,
                window,
                cx,
            )
        }
    }

    fn render_edit_prediction_end_of_line_popover(
        self: &mut Editor,
        label: &'static str,
        editor_snapshot: &EditorSnapshot,
        visible_row_range: Range<DisplayRow>,
        target_display_point: DisplayPoint,
        line_height: Pixels,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        content_origin: gpui::Point<Pixels>,
        editor_width: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<(AnyElement, gpui::Point<Pixels>)> {
        let target_line_end = DisplayPoint::new(
            target_display_point.row(),
            editor_snapshot.line_len(target_display_point.row()),
        );

        let mut element = self
            .render_edit_prediction_line_popover(label, None, window, cx)
            .into_any();

        let size = element.layout_as_root(AvailableSpace::min_size(), window, cx);

        let line_origin =
            self.display_to_pixel_point(target_line_end, editor_snapshot, window, cx)?;

        let start_point = content_origin - point(scroll_pixel_position.x.into(), Pixels::ZERO);
        let mut origin = start_point
            + line_origin
            + point(Self::EDIT_PREDICTION_POPOVER_PADDING_X, Pixels::ZERO);
        origin.x = origin.x.max(content_origin.x);

        let max_x = content_origin.x + editor_width - size.width;

        if origin.x > max_x {
            let offset = line_height + Self::EDIT_PREDICTION_POPOVER_PADDING_Y;

            let icon = if visible_row_range.contains(&(target_display_point.row() + 2)) {
                origin.y += offset;
                IconName::ArrowUp
            } else {
                origin.y -= offset;
                IconName::ArrowDown
            };

            element = self
                .render_edit_prediction_line_popover(label, Some(icon), window, cx)
                .into_any();

            let size = element.layout_as_root(AvailableSpace::min_size(), window, cx);

            origin.x = content_origin.x + editor_width - size.width - px(2.);
        }

        element.prepaint_at(origin, window, cx);
        Some((element, origin))
    }

    fn render_edit_prediction_diff_popover(
        self: &Editor,
        text_bounds: &Bounds<Pixels>,
        content_origin: gpui::Point<Pixels>,
        right_margin: Pixels,
        editor_snapshot: &EditorSnapshot,
        visible_row_range: Range<DisplayRow>,
        line_layouts: &[LineWithInvisibles],
        line_height: Pixels,
        scroll_position: gpui::Point<ScrollOffset>,
        scroll_pixel_position: gpui::Point<ScrollPixelOffset>,
        newest_selection_head: Option<DisplayPoint>,
        editor_width: Pixels,
        style: &EditorStyle,
        edits: &Vec<(Range<Anchor>, Arc<str>)>,
        edit_preview: &Option<language::EditPreview>,
        snapshot: &language::BufferSnapshot,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<(AnyElement, gpui::Point<Pixels>)> {
        let Some((first_edit_range, _)) = edits.first() else {
            return None;
        };
        let Some((last_edit_range, _)) = edits.last() else {
            return None;
        };

        let edit_start = first_edit_range.start.to_display_point(editor_snapshot);
        let edit_end = last_edit_range.end.to_display_point(editor_snapshot);

        let is_visible = visible_row_range.contains(&edit_start.row())
            || visible_row_range.contains(&edit_end.row());
        if !is_visible {
            return None;
        }

        let highlighted_edits = if let Some(edit_preview) = edit_preview.as_ref() {
            edit_prediction_edit_text(
                snapshot,
                edits,
                edit_preview,
                false,
                editor_snapshot.buffer_snapshot(),
                cx,
            )
        } else {
            // Fallback for providers without edit_preview
            edit_prediction_fallback_text(edits, cx)
        };

        let styled_text = highlighted_edits.to_styled_text(&style.text);
        let line_count = highlighted_edits.text.lines().count();

        const BORDER_WIDTH: Pixels = px(1.);

        let keybind = self.render_edit_prediction_keybind(window, cx);
        let has_keybind = keybind.is_some();

        let mut element = h_flex()
            .items_start()
            .child(
                h_flex()
                    .bg(cx.theme().colors().editor_background)
                    .border(BORDER_WIDTH)
                    .shadow_xs()
                    .border_color(cx.theme().colors().border)
                    .rounded_l_lg()
                    .when(line_count > 1, |el| el.rounded_br_lg())
                    .pr_1()
                    .child(styled_text),
            )
            .child(
                h_flex()
                    .h(line_height + BORDER_WIDTH * 2.)
                    .px_1p5()
                    .gap_1()
                    // Workaround: For some reason, there's a gap if we don't do this
                    .ml(-BORDER_WIDTH)
                    .shadow(vec![gpui::BoxShadow {
                        color: gpui::black().opacity(0.05),
                        offset: point(px(1.), px(1.)),
                        blur_radius: px(2.),
                        spread_radius: px(0.),
                    }])
                    .bg(Editor::edit_prediction_line_popover_bg_color(cx))
                    .border(BORDER_WIDTH)
                    .border_color(cx.theme().colors().border)
                    .rounded_r_lg()
                    .id("edit_prediction_diff_popover_keybind")
                    .when(!has_keybind, |el| {
                        let status_colors = cx.theme().status();

                        el.bg(status_colors.error_background)
                            .border_color(status_colors.error.opacity(0.6))
                            .child(Icon::new(IconName::Info).color(Color::Error))
                            .cursor_default()
                            .hoverable_tooltip(move |_window, cx| {
                                cx.new(|_| MissingEditPredictionKeybindingTooltip).into()
                            })
                    })
                    .children(keybind),
            )
            .into_any();

        let longest_row =
            editor_snapshot.longest_row_in_range(edit_start.row()..edit_end.row() + 1);
        let longest_line_width = if visible_row_range.contains(&longest_row) {
            line_layouts[(longest_row.0 - visible_row_range.start.0) as usize].width
        } else {
            layout_line(
                longest_row,
                editor_snapshot,
                style,
                editor_width,
                |_| false,
                window,
                cx,
            )
            .width
        };

        let viewport_bounds =
            Bounds::new(Default::default(), window.viewport_size()).extend(Edges {
                right: -right_margin,
                ..Default::default()
            });

        let x_after_longest = Pixels::from(
            ScrollPixelOffset::from(
                text_bounds.origin.x + longest_line_width + Self::EDIT_PREDICTION_POPOVER_PADDING_X,
            ) - scroll_pixel_position.x,
        );

        let element_bounds = element.layout_as_root(AvailableSpace::min_size(), window, cx);

        // Fully visible if it can be displayed within the window (allow overlapping other
        // panes). However, this is only allowed if the popover starts within text_bounds.
        let can_position_to_the_right = x_after_longest < text_bounds.right()
            && x_after_longest + element_bounds.width < viewport_bounds.right();

        let mut origin = if can_position_to_the_right {
            point(
                x_after_longest,
                text_bounds.origin.y
                    + Pixels::from(
                        edit_start.row().as_f64() * ScrollPixelOffset::from(line_height)
                            - scroll_pixel_position.y,
                    ),
            )
        } else {
            let cursor_row = newest_selection_head.map(|head| head.row());
            let above_edit = edit_start
                .row()
                .0
                .checked_sub(line_count as u32)
                .map(DisplayRow);
            let below_edit = Some(edit_end.row() + 1);
            let above_cursor =
                cursor_row.and_then(|row| row.0.checked_sub(line_count as u32).map(DisplayRow));
            let below_cursor = cursor_row.map(|cursor_row| cursor_row + 1);

            // Place the edit popover adjacent to the edit if there is a location
            // available that is onscreen and does not obscure the cursor. Otherwise,
            // place it adjacent to the cursor.
            let row_target = [above_edit, below_edit, above_cursor, below_cursor]
                .into_iter()
                .flatten()
                .find(|&start_row| {
                    let end_row = start_row + line_count as u32;
                    visible_row_range.contains(&start_row)
                        && visible_row_range.contains(&end_row)
                        && cursor_row
                            .is_none_or(|cursor_row| !((start_row..end_row).contains(&cursor_row)))
                })?;

            content_origin
                + point(
                    Pixels::from(-scroll_pixel_position.x),
                    Pixels::from(
                        (row_target.as_f64() - scroll_position.y) * f64::from(line_height),
                    ),
                )
        };

        origin.x -= BORDER_WIDTH;

        window.with_content_mask(
            Some(gpui::ContentMask {
                bounds: *text_bounds,
            }),
            |window| {
                window.defer_draw(element, origin, 1, Some(window.content_mask()));
            },
        );

        // Do not return an element, since it will already be drawn due to defer_draw.
        None
    }

    fn render_edit_prediction_inline_keystroke(
        &self,
        keystroke: &gpui::KeybindingKeystroke,
        modifiers_color: Color,
        cx: &App,
    ) -> AnyElement {
        let is_platform_style_mac = PlatformStyle::platform() == PlatformStyle::Mac;

        h_flex()
            .px_0p5()
            .when(is_platform_style_mac, |parent| parent.gap_0p5())
            .font(
                theme_settings::ThemeSettings::get_global(cx)
                    .buffer_font
                    .clone(),
            )
            .text_size(TextSize::XSmall.rems(cx))
            .child(h_flex().children(ui::render_modifiers(
                keystroke.modifiers(),
                PlatformStyle::platform(),
                Some(modifiers_color),
                Some(IconSize::XSmall.rems().into()),
                true,
            )))
            .when(is_platform_style_mac, |parent| {
                parent.child(keystroke.key().to_string())
            })
            .when(!is_platform_style_mac, |parent| {
                parent.child(
                    Key::new(ui::utils::capitalize(keystroke.key()), Some(Color::Default))
                        .size(Some(IconSize::XSmall.rems().into())),
                )
            })
            .into_any()
    }

    fn render_edit_prediction_popover_keystroke(
        &self,
        keystroke: &gpui::KeybindingKeystroke,
        color: Color,
        cx: &App,
    ) -> AnyElement {
        let is_platform_style_mac = PlatformStyle::platform() == PlatformStyle::Mac;

        if keystroke.modifiers().modified() {
            h_flex()
                .font(
                    theme_settings::ThemeSettings::get_global(cx)
                        .buffer_font
                        .clone(),
                )
                .when(is_platform_style_mac, |parent| parent.gap_1())
                .child(h_flex().children(ui::render_modifiers(
                    keystroke.modifiers(),
                    PlatformStyle::platform(),
                    Some(color),
                    None,
                    false,
                )))
                .into_any()
        } else {
            Key::new(ui::utils::capitalize(keystroke.key()), Some(color))
                .size(Some(IconSize::XSmall.rems().into()))
                .into_any_element()
        }
    }

    fn render_edit_prediction_keybind(
        &self,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        let keybind_display =
            self.edit_prediction_keybind_display(EditPredictionKeybindSurface::Inline, window, cx);
        let keystroke = keybind_display.displayed_keystroke.as_ref()?;

        let modifiers_color = if *keystroke.modifiers() == window.modifiers() {
            Color::Accent
        } else {
            Color::Muted
        };

        Some(self.render_edit_prediction_inline_keystroke(keystroke, modifiers_color, cx))
    }

    fn render_edit_prediction_line_popover(
        &self,
        label: impl Into<SharedString>,
        icon: Option<IconName>,
        window: &mut Window,
        cx: &mut App,
    ) -> Stateful<Div> {
        let padding_right = if icon.is_some() { px(4.) } else { px(8.) };

        let keybind = self.render_edit_prediction_keybind(window, cx);
        let has_keybind = keybind.is_some();
        let icons = Self::get_prediction_provider_icons(&self.edit_prediction_provider, cx);

        h_flex()
            .id("ep-line-popover")
            .py_0p5()
            .pl_1()
            .pr(padding_right)
            .gap_1()
            .rounded_md()
            .border_1()
            .bg(Self::edit_prediction_line_popover_bg_color(cx))
            .border_color(Self::edit_prediction_callout_popover_border_color(cx))
            .shadow_xs()
            .when(!has_keybind, |el| {
                let status_colors = cx.theme().status();

                el.bg(status_colors.error_background)
                    .border_color(status_colors.error.opacity(0.6))
                    .pl_2()
                    .child(Icon::new(icons.error).color(Color::Error))
                    .cursor_default()
                    .hoverable_tooltip(move |_window, cx| {
                        cx.new(|_| MissingEditPredictionKeybindingTooltip).into()
                    })
            })
            .children(keybind)
            .child(
                Label::new(label)
                    .size(LabelSize::Small)
                    .when(!has_keybind, |el| {
                        el.color(cx.theme().status().error.into()).strikethrough()
                    }),
            )
            .when(!has_keybind, |el| {
                el.child(
                    h_flex().ml_1().child(
                        Icon::new(IconName::Info)
                            .size(IconSize::Small)
                            .color(cx.theme().status().error.into()),
                    ),
                )
            })
            .when_some(icon, |element, icon| {
                element.child(
                    div()
                        .mt(px(1.5))
                        .child(Icon::new(icon).size(IconSize::Small)),
                )
            })
    }

    fn render_edit_prediction_jump_outside_popover(
        &self,
        snapshot: &BufferSnapshot,
        window: &mut Window,
        cx: &mut App,
    ) -> Stateful<Div> {
        let keybind = self.render_edit_prediction_keybind(window, cx);
        let has_keybind = keybind.is_some();
        let icons = Self::get_prediction_provider_icons(&self.edit_prediction_provider, cx);

        let file_name = snapshot
            .file()
            .map(|file| SharedString::new(file.file_name(cx)))
            .unwrap_or(SharedString::new_static("untitled"));

        h_flex()
            .id("ep-jump-outside-popover")
            .py_1()
            .px_2()
            .gap_1()
            .rounded_md()
            .border_1()
            .bg(Self::edit_prediction_line_popover_bg_color(cx))
            .border_color(Self::edit_prediction_callout_popover_border_color(cx))
            .shadow_xs()
            .when(!has_keybind, |el| {
                let status_colors = cx.theme().status();

                el.bg(status_colors.error_background)
                    .border_color(status_colors.error.opacity(0.6))
                    .pl_2()
                    .child(Icon::new(icons.error).color(Color::Error))
                    .cursor_default()
                    .hoverable_tooltip(move |_window, cx| {
                        cx.new(|_| MissingEditPredictionKeybindingTooltip).into()
                    })
            })
            .children(keybind)
            .child(
                Label::new(file_name)
                    .size(LabelSize::Small)
                    .buffer_font(cx)
                    .when(!has_keybind, |el| {
                        el.color(cx.theme().status().error.into()).strikethrough()
                    }),
            )
            .when(!has_keybind, |el| {
                el.child(
                    h_flex().ml_1().child(
                        Icon::new(IconName::Info)
                            .size(IconSize::Small)
                            .color(cx.theme().status().error.into()),
                    ),
                )
            })
            .child(
                div()
                    .mt(px(1.5))
                    .child(Icon::new(IconName::ArrowUpRight).size(IconSize::Small)),
            )
    }

    fn edit_prediction_line_popover_bg_color(cx: &App) -> Hsla {
        let accent_color = cx.theme().colors().text_accent;
        let editor_bg_color = cx.theme().colors().editor_background;
        editor_bg_color.blend(accent_color.opacity(0.1))
    }

    fn edit_prediction_callout_popover_border_color(cx: &App) -> Hsla {
        let accent_color = cx.theme().colors().text_accent;
        let editor_bg_color = cx.theme().colors().editor_background;
        editor_bg_color.blend(accent_color.opacity(0.6))
    }

    fn get_prediction_provider_icons(
        provider: &Option<RegisteredEditPredictionDelegate>,
        cx: &App,
    ) -> edit_prediction_types::EditPredictionIconSet {
        match provider {
            Some(provider) => provider.provider.icons(cx),
            None => edit_prediction_types::EditPredictionIconSet::new(IconName::ZedPredict),
        }
    }

    fn render_edit_prediction_cursor_popover_preview(
        &self,
        completion: &EditPredictionState,
        cursor_point: Point,
        style: &EditorStyle,
        cx: &mut Context<Editor>,
    ) -> Option<Div> {
        use text::ToPoint as _;

        fn render_relative_row_jump(
            prefix: impl Into<String>,
            current_row: u32,
            target_row: u32,
        ) -> Div {
            let (row_diff, arrow) = if target_row < current_row {
                (current_row - target_row, IconName::ArrowUp)
            } else {
                (target_row - current_row, IconName::ArrowDown)
            };

            h_flex()
                .child(
                    Label::new(format!("{}{}", prefix.into(), row_diff))
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                )
                .child(Icon::new(arrow).color(Color::Muted).size(IconSize::Small))
        }

        let supports_jump = self
            .edit_prediction_provider
            .as_ref()
            .map(|provider| provider.provider.supports_jump_to_edit())
            .unwrap_or(true);

        let icons = Self::get_prediction_provider_icons(&self.edit_prediction_provider, cx);

        match &completion.completion {
            EditPrediction::MoveWithin {
                target, snapshot, ..
            } => {
                if !supports_jump {
                    return None;
                }
                let (target, _) = self.display_snapshot(cx).anchor_to_buffer_anchor(*target)?;

                Some(
                    h_flex()
                        .px_2()
                        .gap_2()
                        .flex_1()
                        .child(if target.to_point(snapshot).row > cursor_point.row {
                            Icon::new(icons.down)
                        } else {
                            Icon::new(icons.up)
                        })
                        .child(Label::new("Jump to Edit")),
                )
            }
            EditPrediction::MoveOutside { snapshot, .. } => {
                let file_name = snapshot
                    .file()
                    .map(|file| file.file_name(cx))
                    .unwrap_or("untitled");
                Some(
                    h_flex()
                        .px_2()
                        .gap_2()
                        .flex_1()
                        .child(Icon::new(icons.base))
                        .child(Label::new(format!("Jump to {file_name}"))),
                )
            }
            EditPrediction::Edit {
                edits,
                edit_preview,
                snapshot,
                ..
            } => {
                let first_edit_row = self
                    .display_snapshot(cx)
                    .anchor_to_buffer_anchor(edits.first()?.0.start)?
                    .0
                    .to_point(snapshot)
                    .row;

                let (highlighted_edits, has_more_lines) =
                    if let Some(edit_preview) = edit_preview.as_ref() {
                        edit_prediction_edit_text(
                            snapshot,
                            edits,
                            edit_preview,
                            true,
                            &self.display_snapshot(cx),
                            cx,
                        )
                        .first_line_preview()
                    } else {
                        edit_prediction_fallback_text(edits, cx).first_line_preview()
                    };

                let styled_text = gpui::StyledText::new(highlighted_edits.text)
                    .with_default_highlights(&style.text, highlighted_edits.highlights);

                let preview = h_flex()
                    .gap_1()
                    .min_w_16()
                    .child(styled_text)
                    .when(has_more_lines, |parent| parent.child("…"));

                let left = if supports_jump && first_edit_row != cursor_point.row {
                    render_relative_row_jump("", cursor_point.row, first_edit_row)
                        .into_any_element()
                } else {
                    Icon::new(icons.base).into_any_element()
                };

                Some(
                    h_flex()
                        .h_full()
                        .flex_1()
                        .gap_2()
                        .pr_1()
                        .overflow_x_hidden()
                        .font(
                            theme_settings::ThemeSettings::get_global(cx)
                                .buffer_font
                                .clone(),
                        )
                        .child(left)
                        .child(preview),
                )
            }
        }
    }
}

#[cfg(test)]
impl Editor {
    pub(super) fn set_menu_edit_predictions_policy(&mut self, value: MenuEditPredictionsPolicy) {
        self.menu_edit_predictions_policy = value;
    }
}

struct MissingEditPredictionKeybindingTooltip;

impl Render for MissingEditPredictionKeybindingTooltip {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        ui::tooltip_container(cx, |container, cx| {
            container
                .flex_shrink_0()
                .max_w_80()
                .min_h(rems_from_px(124.))
                .justify_between()
                .child(
                    v_flex()
                        .flex_1()
                        .text_ui_sm(cx)
                        .child(Label::new("Conflict with Accept Keybinding"))
                        .child("Your keymap currently overrides the default accept keybinding. To continue, assign one keybinding for the `editor::AcceptEditPrediction` action.")
                )
                .child(
                    h_flex()
                        .pb_1()
                        .gap_1()
                        .items_end()
                        .w_full()
                        .child(Button::new("open-keymap", "Assign Keybinding").size(ButtonSize::Compact).on_click(|_ev, window, cx| {
                            window.dispatch_action(zed_actions::OpenKeymapFile.boxed_clone(), cx)
                        }))
                        .child(Button::new("see-docs", "See Docs").size(ButtonSize::Compact).on_click(|_ev, _window, cx| {
                            cx.open_url("https://zed.dev/docs/completions#edit-predictions-missing-keybinding");
                        })),
                )
        })
    }
}

fn edit_prediction_fallback_text(edits: &[(Range<Anchor>, Arc<str>)], cx: &App) -> HighlightedText {
    // Fallback for providers that don't provide edit_preview (like Copilot)
    // Just show the raw edit text with basic styling
    let mut text = String::new();
    let mut highlights = Vec::new();

    let insertion_highlight_style = HighlightStyle {
        color: Some(cx.theme().colors().text),
        ..Default::default()
    };

    for (_, edit_text) in edits {
        let start_offset = text.len();
        text.push_str(edit_text);
        let end_offset = text.len();

        if start_offset < end_offset {
            highlights.push((start_offset..end_offset, insertion_highlight_style));
        }
    }

    HighlightedText {
        text: text.into(),
        highlights,
    }
}

fn all_edits_insertions_or_deletions(
    edits: &Vec<(Range<Anchor>, Arc<str>)>,
    snapshot: &MultiBufferSnapshot,
) -> bool {
    let mut all_insertions = true;
    let mut all_deletions = true;

    for (range, new_text) in edits.iter() {
        let range_is_empty = range.to_offset(snapshot).is_empty();
        let text_is_empty = new_text.is_empty();

        if range_is_empty != text_is_empty {
            if range_is_empty {
                all_deletions = false;
            } else {
                all_insertions = false;
            }
        } else {
            return false;
        }

        if !all_insertions && !all_deletions {
            return false;
        }
    }
    all_insertions || all_deletions
}
