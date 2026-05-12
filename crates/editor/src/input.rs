use super::*;

const ORDERED_LIST_MAX_MARKER_LEN: usize = 16;

impl Editor {
    pub fn set_input_enabled(&mut self, input_enabled: bool) {
        self.input_enabled = input_enabled;
    }

    pub fn set_expects_character_input(&mut self, expects_character_input: bool) {
        self.expects_character_input = expects_character_input;
    }

    pub fn set_autoindent(&mut self, autoindent: bool) {
        if autoindent {
            self.autoindent_mode = Some(AutoindentMode::EachLine);
        } else {
            self.autoindent_mode = None;
        }
    }

    pub fn set_use_autoclose(&mut self, autoclose: bool) {
        self.use_autoclose = autoclose;
    }

    pub fn replay_insert_event(
        &mut self,
        text: &str,
        relative_utf16_range: Option<Range<isize>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.input_enabled {
            cx.emit(EditorEvent::InputIgnored { text: text.into() });
            return;
        }
        if let Some(relative_utf16_range) = relative_utf16_range {
            let selections = self
                .selections
                .all::<MultiBufferOffsetUtf16>(&self.display_snapshot(cx));
            self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                let new_ranges = selections.into_iter().map(|range| {
                    let start = MultiBufferOffsetUtf16(OffsetUtf16(
                        range
                            .head()
                            .0
                            .0
                            .saturating_add_signed(relative_utf16_range.start),
                    ));
                    let end = MultiBufferOffsetUtf16(OffsetUtf16(
                        range
                            .head()
                            .0
                            .0
                            .saturating_add_signed(relative_utf16_range.end),
                    ));
                    start..end
                });
                s.select_ranges(new_ranges);
            });
        }

        self.handle_input(text, window, cx);
    }

    pub fn handle_input(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        let text: Arc<str> = text.into();

        if self.read_only(cx) {
            return;
        }

        self.unfold_buffers_with_selections(cx);

        let selections = self.selections.all_adjusted(&self.display_snapshot(cx));
        let mut bracket_inserted = false;
        let mut edits = Vec::new();
        let mut linked_edits = LinkedEdits::new();
        let mut new_selections = Vec::with_capacity(selections.len());
        let mut new_autoclose_regions = Vec::new();
        let snapshot = self.buffer.read(cx).read(cx);
        let mut clear_linked_edit_ranges = false;
        let mut all_selections_read_only = true;
        let mut has_adjacent_edits = false;
        let mut in_adjacent_group = false;

        let mut regions = self
            .selections_with_autoclose_regions(selections, &snapshot)
            .peekable();

        while let Some((selection, autoclose_region)) = regions.next() {
            if snapshot
                .point_to_buffer_point(selection.head())
                .is_none_or(|(snapshot, ..)| !snapshot.capability.editable())
            {
                continue;
            }
            if snapshot
                .point_to_buffer_point(selection.tail())
                .is_none_or(|(snapshot, ..)| !snapshot.capability.editable())
            {
                // note, ideally we'd clip the tail to the closest writeable region towards the head
                continue;
            }
            all_selections_read_only = false;

            if let Some(scope) = snapshot.language_scope_at(selection.head()) {
                // Determine if the inserted text matches the opening or closing
                // bracket of any of this language's bracket pairs.
                let mut bracket_pair = None;
                let mut is_bracket_pair_start = false;
                let mut is_bracket_pair_end = false;
                if !text.is_empty() {
                    let mut bracket_pair_matching_end = None;
                    // `text` can be empty when a user is using IME (e.g. Chinese Wubi Simplified)
                    //  and they are removing the character that triggered IME popup.
                    for (pair, enabled) in scope.brackets() {
                        if !pair.close && !pair.surround {
                            continue;
                        }

                        if enabled && pair.start.ends_with(text.as_ref()) {
                            let prefix_len = pair.start.len() - text.len();
                            let preceding_text_matches_prefix = prefix_len == 0
                                || (selection.start.column >= (prefix_len as u32)
                                    && snapshot.contains_str_at(
                                        Point::new(
                                            selection.start.row,
                                            selection.start.column - (prefix_len as u32),
                                        ),
                                        &pair.start[..prefix_len],
                                    ));
                            if preceding_text_matches_prefix {
                                bracket_pair = Some(pair.clone());
                                is_bracket_pair_start = true;
                                break;
                            }
                        }
                        if pair.end.as_str() == text.as_ref() && bracket_pair_matching_end.is_none()
                        {
                            // take first bracket pair matching end, but don't break in case a later bracket
                            // pair matches start
                            bracket_pair_matching_end = Some(pair.clone());
                        }
                    }
                    if let Some(end) = bracket_pair_matching_end
                        && bracket_pair.is_none()
                    {
                        bracket_pair = Some(end);
                        is_bracket_pair_end = true;
                    }
                }

                if let Some(bracket_pair) = bracket_pair {
                    let snapshot_settings = snapshot.language_settings_at(selection.start, cx);
                    let autoclose = self.use_autoclose && snapshot_settings.use_autoclose;
                    let auto_surround =
                        self.use_auto_surround && snapshot_settings.use_auto_surround;
                    if selection.is_empty() {
                        if is_bracket_pair_start {
                            // If the inserted text is a suffix of an opening bracket and the
                            // selection is preceded by the rest of the opening bracket, then
                            // insert the closing bracket.
                            let following_text_allows_autoclose = snapshot
                                .chars_at(selection.start)
                                .next()
                                .is_none_or(|c| scope.should_autoclose_before(c));

                            let preceding_text_allows_autoclose = selection.start.column == 0
                                || snapshot
                                    .reversed_chars_at(selection.start)
                                    .next()
                                    .is_none_or(|c| {
                                        bracket_pair.start != bracket_pair.end
                                            || !snapshot
                                                .char_classifier_at(selection.start)
                                                .is_word(c)
                                    });

                            let is_closing_quote = if bracket_pair.end == bracket_pair.start
                                && bracket_pair.start.len() == 1
                            {
                                if let Some(target) = bracket_pair.start.chars().next() {
                                    let mut byte_offset = 0u32;
                                    let current_line_count = snapshot
                                        .reversed_chars_at(selection.start)
                                        .take_while(|&c| c != '\n')
                                        .filter(|c| {
                                            byte_offset += c.len_utf8() as u32;
                                            if *c != target {
                                                return false;
                                            }

                                            let point = Point::new(
                                                selection.start.row,
                                                selection.start.column.saturating_sub(byte_offset),
                                            );

                                            let is_enabled = snapshot
                                                .language_scope_at(point)
                                                .and_then(|scope| {
                                                    scope
                                                        .brackets()
                                                        .find(|(pair, _)| {
                                                            pair.start == bracket_pair.start
                                                        })
                                                        .map(|(_, enabled)| enabled)
                                                })
                                                .unwrap_or(true);

                                            let is_delimiter = snapshot
                                                .language_scope_at(Point::new(
                                                    point.row,
                                                    point.column + 1,
                                                ))
                                                .and_then(|scope| {
                                                    scope
                                                        .brackets()
                                                        .find(|(pair, _)| {
                                                            pair.start == bracket_pair.start
                                                        })
                                                        .map(|(_, enabled)| !enabled)
                                                })
                                                .unwrap_or(false);

                                            is_enabled && !is_delimiter
                                        })
                                        .count();
                                    current_line_count % 2 == 1
                                } else {
                                    false
                                }
                            } else {
                                false
                            };

                            if autoclose
                                && bracket_pair.close
                                && following_text_allows_autoclose
                                && preceding_text_allows_autoclose
                                && !is_closing_quote
                            {
                                let anchor = snapshot.anchor_before(selection.end);
                                new_selections.push((selection.map(|_| anchor), text.len()));
                                new_autoclose_regions.push((
                                    anchor,
                                    text.len(),
                                    selection.id,
                                    bracket_pair.clone(),
                                ));
                                edits.push((
                                    selection.range(),
                                    format!("{}{}", text, bracket_pair.end).into(),
                                ));
                                bracket_inserted = true;
                                continue;
                            }
                        }

                        if let Some(region) = autoclose_region {
                            // If the selection is followed by an auto-inserted closing bracket,
                            // then don't insert that closing bracket again; just move the selection
                            // past the closing bracket.
                            let should_skip = selection.end == region.range.end.to_point(&snapshot)
                                && text.as_ref() == region.pair.end.as_str()
                                && snapshot.contains_str_at(region.range.end, text.as_ref());
                            if should_skip {
                                let anchor = snapshot.anchor_after(selection.end);
                                new_selections
                                    .push((selection.map(|_| anchor), region.pair.end.len()));
                                continue;
                            }
                        }

                        let always_treat_brackets_as_autoclosed = snapshot
                            .language_settings_at(selection.start, cx)
                            .always_treat_brackets_as_autoclosed;
                        if always_treat_brackets_as_autoclosed
                            && is_bracket_pair_end
                            && snapshot.contains_str_at(selection.end, text.as_ref())
                        {
                            // Otherwise, when `always_treat_brackets_as_autoclosed` is set to `true
                            // and the inserted text is a closing bracket and the selection is followed
                            // by the closing bracket then move the selection past the closing bracket.
                            let anchor = snapshot.anchor_after(selection.end);
                            new_selections.push((selection.map(|_| anchor), text.len()));
                            continue;
                        }
                    }
                    // If an opening bracket is 1 character long and is typed while
                    // text is selected, then surround that text with the bracket pair.
                    else if auto_surround
                        && bracket_pair.surround
                        && is_bracket_pair_start
                        && bracket_pair.start.chars().count() == 1
                    {
                        edits.push((selection.start..selection.start, text.clone()));
                        edits.push((
                            selection.end..selection.end,
                            bracket_pair.end.as_str().into(),
                        ));
                        bracket_inserted = true;
                        new_selections.push((
                            Selection {
                                id: selection.id,
                                start: snapshot.anchor_after(selection.start),
                                end: snapshot.anchor_before(selection.end),
                                reversed: selection.reversed,
                                goal: selection.goal,
                            },
                            0,
                        ));
                        continue;
                    }
                }
            }

            if self.auto_replace_emoji_shortcode
                && selection.is_empty()
                && text.as_ref().ends_with(':')
                && let Some(possible_emoji_short_code) =
                    Self::find_possible_emoji_shortcode_at_position(&snapshot, selection.start)
                && !possible_emoji_short_code.is_empty()
                && let Some(emoji) = emojis::get_by_shortcode(&possible_emoji_short_code)
            {
                let emoji_shortcode_start = Point::new(
                    selection.start.row,
                    selection.start.column - possible_emoji_short_code.len() as u32 - 1,
                );

                // Remove shortcode from buffer
                edits.push((
                    emoji_shortcode_start..selection.start,
                    "".to_string().into(),
                ));
                new_selections.push((
                    Selection {
                        id: selection.id,
                        start: snapshot.anchor_after(emoji_shortcode_start),
                        end: snapshot.anchor_before(selection.start),
                        reversed: selection.reversed,
                        goal: selection.goal,
                    },
                    0,
                ));

                // Insert emoji
                let selection_start_anchor = snapshot.anchor_after(selection.start);
                new_selections.push((selection.map(|_| selection_start_anchor), 0));
                edits.push((selection.start..selection.end, emoji.to_string().into()));

                continue;
            }

            let next_is_adjacent = regions
                .peek()
                .is_some_and(|(next, _)| selection.end == next.start);

            // If not handling any auto-close operation, then just replace the selected
            // text with the given input and move the selection to the end of the
            // newly inserted text.
            let anchor = if in_adjacent_group || next_is_adjacent {
                // After edits the right bias would shift those anchor to the next visible fragment
                // but we want to resolve to the previous one
                snapshot.anchor_before(selection.end)
            } else {
                snapshot.anchor_after(selection.end)
            };

            if !self.linked_edit_ranges.is_empty() {
                let start_anchor = snapshot.anchor_before(selection.start);
                let classifier = snapshot
                    .char_classifier_at(start_anchor)
                    .scope_context(Some(CharScopeContext::LinkedEdit));

                if let Some((_, anchor_range)) =
                    snapshot.anchor_range_to_buffer_anchor_range(start_anchor..anchor)
                {
                    let is_word_char = text
                        .chars()
                        .next()
                        .is_none_or(|char| classifier.is_word(char));

                    let is_dot = text.as_ref() == ".";
                    let should_apply_linked_edit = is_word_char || is_dot;

                    if should_apply_linked_edit {
                        linked_edits.push(&self, anchor_range, text.clone(), cx);
                    } else {
                        clear_linked_edit_ranges = true;
                    }
                }
            }

            new_selections.push((selection.map(|_| anchor), 0));
            edits.push((selection.start..selection.end, text.clone()));

            has_adjacent_edits |= next_is_adjacent;
            in_adjacent_group = next_is_adjacent;
        }

        if all_selections_read_only {
            return;
        }

        drop(regions);
        drop(snapshot);

        self.transact(window, cx, |this, window, cx| {
            if clear_linked_edit_ranges {
                this.linked_edit_ranges.clear();
            }
            let initial_buffer_versions =
                jsx_tag_auto_close::construct_initial_buffer_versions_map(this, &edits, cx);

            this.buffer.update(cx, |buffer, cx| {
                if has_adjacent_edits {
                    buffer.edit_non_coalesce(edits, this.autoindent_mode.clone(), cx);
                } else {
                    buffer.edit(edits, this.autoindent_mode.clone(), cx);
                }
            });
            linked_edits.apply(cx);
            let new_anchor_selections = new_selections.iter().map(|e| &e.0);
            let new_selection_deltas = new_selections.iter().map(|e| e.1);
            let map = this.display_map.update(cx, |map, cx| map.snapshot(cx));
            let new_selections = resolve_selections_wrapping_blocks::<MultiBufferOffset, _>(
                new_anchor_selections,
                &map,
            )
            .zip(new_selection_deltas)
            .map(|(selection, delta)| Selection {
                id: selection.id,
                start: selection.start + delta,
                end: selection.end + delta,
                reversed: selection.reversed,
                goal: SelectionGoal::None,
            })
            .collect::<Vec<_>>();

            let mut i = 0;
            for (position, delta, selection_id, pair) in new_autoclose_regions {
                let position = position.to_offset(map.buffer_snapshot()) + delta;
                let start = map.buffer_snapshot().anchor_before(position);
                let end = map.buffer_snapshot().anchor_after(position);
                while let Some(existing_state) = this.autoclose_regions.get(i) {
                    match existing_state
                        .range
                        .start
                        .cmp(&start, map.buffer_snapshot())
                    {
                        Ordering::Less => i += 1,
                        Ordering::Greater => break,
                        Ordering::Equal => {
                            match end.cmp(&existing_state.range.end, map.buffer_snapshot()) {
                                Ordering::Less => i += 1,
                                Ordering::Equal => break,
                                Ordering::Greater => break,
                            }
                        }
                    }
                }
                this.autoclose_regions.insert(
                    i,
                    AutocloseRegion {
                        selection_id,
                        range: start..end,
                        pair,
                    },
                );
            }

            let had_active_edit_prediction = this.has_active_edit_prediction();
            this.change_selections(
                SelectionEffects::scroll(Autoscroll::fit()).completions(false),
                window,
                cx,
                |s| s.select(new_selections),
            );

            if !bracket_inserted
                && let Some(on_type_format_task) =
                    this.trigger_on_type_formatting(text.to_string(), window, cx)
            {
                on_type_format_task.detach_and_log_err(cx);
            }

            let editor_settings = EditorSettings::get_global(cx);
            if bracket_inserted
                && (editor_settings.auto_signature_help
                    || editor_settings.show_signature_help_after_edits)
            {
                this.show_signature_help(&ShowSignatureHelp, window, cx);
            }

            let trigger_in_words =
                this.show_edit_predictions_in_menu() || !had_active_edit_prediction;
            if this.hard_wrap.is_some() {
                let latest: Range<Point> = this.selections.newest(&map).range();
                if latest.is_empty()
                    && this
                        .buffer()
                        .read(cx)
                        .snapshot(cx)
                        .line_len(MultiBufferRow(latest.start.row))
                        == latest.start.column
                {
                    this.rewrap(
                        RewrapOptions {
                            override_language_settings: true,
                            preserve_existing_whitespace: true,
                            line_length: None,
                        },
                        cx,
                    )
                }
            }
            this.trigger_completion_on_input(&text, trigger_in_words, window, cx);
            refresh_linked_ranges(this, window, cx);
            this.refresh_edit_prediction(true, false, window, cx);
            jsx_tag_auto_close::handle_from(this, initial_buffer_versions, window, cx);
        });
    }

    pub fn newline(&mut self, _: &Newline, window: &mut Window, cx: &mut Context<Self>) {
        if self.read_only(cx) {
            return;
        }

        self.transact(window, cx, |this, window, cx| {
            let (edits_with_flags, selection_info): (Vec<_>, Vec<_>) = {
                let selections = this
                    .selections
                    .all::<MultiBufferOffset>(&this.display_snapshot(cx));
                let multi_buffer = this.buffer.read(cx);
                let buffer = multi_buffer.snapshot(cx);
                selections
                    .iter()
                    .map(|selection| {
                        let start_point = selection.start.to_point(&buffer);
                        let mut existing_indent =
                            buffer.indent_size_for_line(MultiBufferRow(start_point.row));
                        let full_indent_len = existing_indent.len;
                        existing_indent.len = cmp::min(existing_indent.len, start_point.column);
                        let mut start = selection.start;
                        let end = selection.end;
                        let selection_is_empty = start == end;
                        let language_scope = buffer.language_scope_at(start);
                        let (delimiter, newline_config) = if let Some(language) = &language_scope {
                            let needs_extra_newline = NewlineConfig::insert_extra_newline_brackets(
                                &buffer,
                                start..end,
                                language,
                            )
                                || NewlineConfig::insert_extra_newline_tree_sitter(
                                    &buffer,
                                    start..end,
                                );

                            let mut newline_config = NewlineConfig::Newline {
                                additional_indent: IndentSize::spaces(0),
                                extra_line_additional_indent: if needs_extra_newline {
                                    Some(IndentSize::spaces(0))
                                } else {
                                    None
                                },
                                prevent_auto_indent: false,
                            };

                            let comment_delimiter = maybe!({
                                if !selection_is_empty {
                                    return None;
                                }

                                if !multi_buffer.language_settings(cx).extend_comment_on_newline {
                                    return None;
                                }

                                return comment_delimiter_for_newline(
                                    &start_point,
                                    &buffer,
                                    language,
                                );
                            });

                            let doc_delimiter = maybe!({
                                if !selection_is_empty {
                                    return None;
                                }

                                if !multi_buffer.language_settings(cx).extend_comment_on_newline {
                                    return None;
                                }

                                return documentation_delimiter_for_newline(
                                    &start_point,
                                    &buffer,
                                    language,
                                    &mut newline_config,
                                );
                            });

                            let list_delimiter = maybe!({
                                if !selection_is_empty {
                                    return None;
                                }

                                if !multi_buffer.language_settings(cx).extend_list_on_newline {
                                    return None;
                                }

                                return list_delimiter_for_newline(
                                    &start_point,
                                    &buffer,
                                    language,
                                    &mut newline_config,
                                );
                            });

                            (
                                comment_delimiter.or(doc_delimiter).or(list_delimiter),
                                newline_config,
                            )
                        } else {
                            (
                                None,
                                NewlineConfig::Newline {
                                    additional_indent: IndentSize::spaces(0),
                                    extra_line_additional_indent: None,
                                    prevent_auto_indent: false,
                                },
                            )
                        };

                        let (edit_start, new_text, prevent_auto_indent) = match &newline_config {
                            NewlineConfig::ClearCurrentLine => {
                                let row_start =
                                    buffer.point_to_offset(Point::new(start_point.row, 0));
                                (row_start, String::new(), false)
                            }
                            NewlineConfig::UnindentCurrentLine { continuation } => {
                                let row_start =
                                    buffer.point_to_offset(Point::new(start_point.row, 0));
                                let tab_size = buffer.language_settings_at(start, cx).tab_size;
                                let tab_size_indent = IndentSize::spaces(tab_size.get());
                                let reduced_indent =
                                    existing_indent.with_delta(Ordering::Less, tab_size_indent);
                                let mut new_text = String::new();
                                new_text.extend(reduced_indent.chars());
                                new_text.push_str(continuation);
                                (row_start, new_text, true)
                            }
                            NewlineConfig::Newline {
                                additional_indent,
                                extra_line_additional_indent,
                                prevent_auto_indent,
                            } => {
                                let auto_indent_mode =
                                    buffer.language_settings_at(start, cx).auto_indent;
                                let preserve_indent =
                                    auto_indent_mode != language::AutoIndentMode::None;
                                let apply_syntax_indent =
                                    auto_indent_mode == language::AutoIndentMode::SyntaxAware;
                                let capacity_for_delimiter =
                                    delimiter.as_deref().map(str::len).unwrap_or_default();
                                let existing_indent_len = if preserve_indent {
                                    existing_indent.len as usize
                                } else {
                                    0
                                };
                                let extra_line_len = extra_line_additional_indent
                                    .map(|i| 1 + existing_indent_len + i.len as usize)
                                    .unwrap_or(0);
                                let mut new_text = String::with_capacity(
                                    1 + capacity_for_delimiter
                                        + existing_indent_len
                                        + additional_indent.len as usize
                                        + extra_line_len,
                                );
                                new_text.push('\n');
                                if preserve_indent {
                                    new_text.extend(existing_indent.chars());
                                }
                                new_text.extend(additional_indent.chars());
                                if let Some(delimiter) = &delimiter {
                                    new_text.push_str(delimiter);
                                }
                                if let Some(extra_indent) = extra_line_additional_indent {
                                    new_text.push('\n');
                                    if preserve_indent {
                                        new_text.extend(existing_indent.chars());
                                    }
                                    new_text.extend(extra_indent.chars());
                                }
                                // Extend the edit to the beginning of the line
                                // to clear auto-indent whitespace that would
                                // otherwise remain as trailing whitespace. This
                                // applies to blank lines and lines where only
                                // indentation remains before the cursor.
                                if selection_is_empty
                                    && preserve_indent
                                    && full_indent_len > 0
                                    && start_point.column == full_indent_len
                                {
                                    start = buffer.point_to_offset(Point::new(start_point.row, 0));
                                }

                                (
                                    start,
                                    new_text,
                                    *prevent_auto_indent || !apply_syntax_indent,
                                )
                            }
                        };

                        let anchor = buffer.anchor_after(end);
                        let new_selection = selection.map(|_| anchor);
                        (
                            ((edit_start..end, new_text), prevent_auto_indent),
                            (newline_config.has_extra_line(), new_selection),
                        )
                    })
                    .unzip()
            };

            let mut auto_indent_edits = Vec::new();
            let mut edits = Vec::new();
            for (edit, prevent_auto_indent) in edits_with_flags {
                if prevent_auto_indent {
                    edits.push(edit);
                } else {
                    auto_indent_edits.push(edit);
                }
            }
            if !edits.is_empty() {
                this.edit(edits, cx);
            }
            if !auto_indent_edits.is_empty() {
                this.edit_with_autoindent(auto_indent_edits, cx);
            }

            let buffer = this.buffer.read(cx).snapshot(cx);
            let new_selections = selection_info
                .into_iter()
                .map(|(extra_newline_inserted, new_selection)| {
                    let mut cursor = new_selection.end.to_point(&buffer);
                    if extra_newline_inserted {
                        cursor.row -= 1;
                        cursor.column = buffer.line_len(MultiBufferRow(cursor.row));
                    }
                    new_selection.map(|_| cursor)
                })
                .collect();

            this.change_selections(Default::default(), window, cx, |s| s.select(new_selections));
            this.refresh_edit_prediction(true, false, window, cx);
            if let Some(task) = this.trigger_on_type_formatting("\n".to_owned(), window, cx) {
                task.detach_and_log_err(cx);
            }
        });
    }

    pub fn newline_above(&mut self, _: &NewlineAbove, window: &mut Window, cx: &mut Context<Self>) {
        if self.read_only(cx) {
            return;
        }

        let buffer = self.buffer.read(cx);
        let snapshot = buffer.snapshot(cx);

        let mut edits = Vec::new();
        let mut rows = Vec::new();

        for (rows_inserted, selection) in self
            .selections
            .all_adjusted(&self.display_snapshot(cx))
            .into_iter()
            .enumerate()
        {
            let cursor = selection.head();
            let row = cursor.row;

            let start_of_line = snapshot.clip_point(Point::new(row, 0), Bias::Left);

            let newline = "\n".to_string();
            edits.push((start_of_line..start_of_line, newline));

            rows.push(row + rows_inserted as u32);
        }

        self.transact(window, cx, |editor, window, cx| {
            editor.edit(edits, cx);

            editor.change_selections(Default::default(), window, cx, |s| {
                let mut index = 0;
                s.move_cursors_with(&mut |map, _, _| {
                    let row = rows[index];
                    index += 1;

                    let point = Point::new(row, 0);
                    let boundary = map.next_line_boundary(point).1;
                    let clipped = map.clip_point(boundary, Bias::Left);

                    (clipped, SelectionGoal::None)
                });
            });

            let mut indent_edits = Vec::new();
            let multibuffer_snapshot = editor.buffer.read(cx).snapshot(cx);
            for row in rows {
                let indents = multibuffer_snapshot.suggested_indents(row..row + 1, cx);
                for (row, indent) in indents {
                    if indent.len == 0 {
                        continue;
                    }

                    let text = match indent.kind {
                        IndentKind::Space => " ".repeat(indent.len as usize),
                        IndentKind::Tab => "\t".repeat(indent.len as usize),
                    };
                    let point = Point::new(row.0, 0);
                    indent_edits.push((point..point, text));
                }
            }
            editor.edit(indent_edits, cx);
            if let Some(format) = editor.trigger_on_type_formatting("\n".to_owned(), window, cx) {
                format.detach_and_log_err(cx);
            }
        });
    }

    pub fn newline_below(&mut self, _: &NewlineBelow, window: &mut Window, cx: &mut Context<Self>) {
        if self.read_only(cx) {
            return;
        }

        let mut buffer_edits: HashMap<EntityId, (Entity<Buffer>, Vec<Point>)> = HashMap::default();
        let mut rows = Vec::new();
        let mut rows_inserted = 0;

        for selection in self.selections.all_adjusted(&self.display_snapshot(cx)) {
            let cursor = selection.head();
            let row = cursor.row;

            let point = Point::new(row, 0);
            let Some((buffer_handle, buffer_point)) =
                self.buffer.read(cx).point_to_buffer_point(point, cx)
            else {
                continue;
            };

            buffer_edits
                .entry(buffer_handle.entity_id())
                .or_insert_with(|| (buffer_handle, Vec::new()))
                .1
                .push(buffer_point);

            rows_inserted += 1;
            rows.push(row + rows_inserted);
        }

        self.transact(window, cx, |editor, window, cx| {
            for (_, (buffer_handle, points)) in &buffer_edits {
                buffer_handle.update(cx, |buffer, cx| {
                    let edits: Vec<_> = points
                        .iter()
                        .map(|point| {
                            let target = Point::new(point.row + 1, 0);
                            let start_of_line = buffer.point_to_offset(target).min(buffer.len());
                            (start_of_line..start_of_line, "\n")
                        })
                        .collect();
                    buffer.edit(edits, None, cx);
                });
            }

            editor.change_selections(Default::default(), window, cx, |s| {
                let mut index = 0;
                s.move_cursors_with(&mut |map, _, _| {
                    let row = rows[index];
                    index += 1;

                    let point = Point::new(row, 0);
                    let boundary = map.next_line_boundary(point).1;
                    let clipped = map.clip_point(boundary, Bias::Left);

                    (clipped, SelectionGoal::None)
                });
            });

            let mut indent_edits = Vec::new();
            let multibuffer_snapshot = editor.buffer.read(cx).snapshot(cx);
            for row in rows {
                let indents = multibuffer_snapshot.suggested_indents(row..row + 1, cx);
                for (row, indent) in indents {
                    if indent.len == 0 {
                        continue;
                    }

                    let text = match indent.kind {
                        IndentKind::Space => " ".repeat(indent.len as usize),
                        IndentKind::Tab => "\t".repeat(indent.len as usize),
                    };
                    let point = Point::new(row.0, 0);
                    indent_edits.push((point..point, text));
                }
            }
            editor.edit(indent_edits, cx);
            if let Some(format) = editor.trigger_on_type_formatting("\n".to_owned(), window, cx) {
                format.detach_and_log_err(cx);
            }
        });
    }

    pub fn insert(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        let autoindent = text.is_empty().not().then(|| AutoindentMode::Block {
            original_indent_columns: Vec::new(),
        });
        self.replace_selections(text, autoindent, window, cx, false);
    }

    /// Collects linked edits for the current selections, pairing each linked
    /// range with `text`.
    pub fn linked_edits_for_selections(&self, text: Arc<str>, cx: &App) -> LinkedEdits {
        let multibuffer_snapshot = self.buffer().read(cx).snapshot(cx);
        let mut linked_edits = LinkedEdits::new();
        if !self.linked_edit_ranges.is_empty() {
            for selection in self.selections.disjoint_anchors() {
                let Some((_, range)) =
                    multibuffer_snapshot.anchor_range_to_buffer_anchor_range(selection.range())
                else {
                    continue;
                };
                linked_edits.push(self, range, text.clone(), cx);
            }
        }
        linked_edits
    }

    /// Deletes the content covered by the current selections and applies
    /// linked edits.
    pub fn delete_selections_with_linked_edits(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.replace_selections("", None, window, cx, true);
    }

    pub(super) fn observe_pending_input(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let mut pending: String = window
            .pending_input_keystrokes()
            .into_iter()
            .flatten()
            .filter_map(|keystroke| keystroke.key_char.clone())
            .collect();

        if !self.input_enabled || self.read_only || !self.focus_handle.is_focused(window) {
            pending = "".to_string();
        }

        let existing_pending = self
            .text_highlights(HighlightKey::PendingInput, cx)
            .map(|(_, ranges)| ranges.to_vec());
        if existing_pending.is_none() && pending.is_empty() {
            return;
        }
        let transaction =
            self.transact(window, cx, |this, window, cx| {
                let selections = this
                    .selections
                    .all::<MultiBufferOffset>(&this.display_snapshot(cx));
                let edits = selections
                    .iter()
                    .map(|selection| (selection.end..selection.end, pending.clone()));
                this.edit(edits, cx);
                this.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges(selections.into_iter().enumerate().map(|(ix, sel)| {
                        sel.start + ix * pending.len()..sel.end + ix * pending.len()
                    }));
                });
                if let Some(existing_ranges) = existing_pending {
                    let edits = existing_ranges.iter().map(|range| (range.clone(), ""));
                    this.edit(edits, cx);
                }
            });

        let snapshot = self.snapshot(window, cx);
        let ranges = self
            .selections
            .all::<MultiBufferOffset>(&snapshot.display_snapshot)
            .into_iter()
            .map(|selection| {
                snapshot.buffer_snapshot().anchor_after(selection.end)
                    ..snapshot
                        .buffer_snapshot()
                        .anchor_before(selection.end + pending.len())
            })
            .collect();

        if pending.is_empty() {
            self.clear_highlights(HighlightKey::PendingInput, cx);
        } else {
            self.highlight_text(
                HighlightKey::PendingInput,
                ranges,
                HighlightStyle {
                    underline: Some(UnderlineStyle {
                        thickness: px(1.),
                        color: None,
                        wavy: false,
                    }),
                    ..Default::default()
                },
                cx,
            );
        }

        self.ime_transaction = self.ime_transaction.or(transaction);
        if let Some(transaction) = self.ime_transaction {
            self.buffer.update(cx, |buffer, cx| {
                buffer.group_until_transaction(transaction, cx);
            });
        }

        if self
            .text_highlights(HighlightKey::PendingInput, cx)
            .is_none()
        {
            self.ime_transaction.take();
        }
    }

    pub(super) fn linked_editing_ranges_for(
        &self,
        query_range: Range<text::Anchor>,
        cx: &App,
    ) -> Option<HashMap<Entity<Buffer>, Vec<Range<text::Anchor>>>> {
        use text::ToOffset as TO;

        if self.linked_edit_ranges.is_empty() {
            return None;
        }
        if query_range.start.buffer_id != query_range.end.buffer_id {
            return None;
        };
        let multibuffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let buffer = self.buffer.read(cx).buffer(query_range.end.buffer_id)?;
        let buffer_snapshot = buffer.read(cx).snapshot();
        let (base_range, linked_ranges) = self.linked_edit_ranges.get(
            buffer_snapshot.remote_id(),
            query_range.clone(),
            &buffer_snapshot,
        )?;
        // find offset from the start of current range to current cursor position
        let start_byte_offset = TO::to_offset(&base_range.start, &buffer_snapshot);

        let start_offset = TO::to_offset(&query_range.start, &buffer_snapshot);
        let start_difference = start_offset - start_byte_offset;
        let end_offset = TO::to_offset(&query_range.end, &buffer_snapshot);
        let end_difference = end_offset - start_byte_offset;

        // Current range has associated linked ranges.
        let mut linked_edits = HashMap::<_, Vec<_>>::default();
        for range in linked_ranges.iter() {
            let start_offset = TO::to_offset(&range.start, &buffer_snapshot);
            let end_offset = start_offset + end_difference;
            let start_offset = start_offset + start_difference;
            if start_offset > buffer_snapshot.len() || end_offset > buffer_snapshot.len() {
                continue;
            }
            if self.selections.disjoint_anchor_ranges().any(|s| {
                let Some((selection_start, _)) =
                    multibuffer_snapshot.anchor_to_buffer_anchor(s.start)
                else {
                    return false;
                };
                let Some((selection_end, _)) = multibuffer_snapshot.anchor_to_buffer_anchor(s.end)
                else {
                    return false;
                };
                if selection_start.buffer_id != query_range.start.buffer_id
                    || selection_end.buffer_id != query_range.end.buffer_id
                {
                    return false;
                }
                TO::to_offset(&selection_start, &buffer_snapshot) <= end_offset
                    && TO::to_offset(&selection_end, &buffer_snapshot) >= start_offset
            }) {
                continue;
            }
            let start = buffer_snapshot.anchor_after(start_offset);
            let end = buffer_snapshot.anchor_after(end_offset);
            linked_edits
                .entry(buffer.clone())
                .or_default()
                .push(start..end);
        }
        Some(linked_edits)
    }

    pub(super) fn marked_text_ranges(
        &self,
        cx: &App,
    ) -> Option<Vec<Range<MultiBufferOffsetUtf16>>> {
        let snapshot = self.buffer.read(cx).read(cx);
        let (_, ranges) = self.text_highlights(HighlightKey::InputComposition, cx)?;
        Some(
            ranges
                .iter()
                .map(move |range| {
                    range.start.to_offset_utf16(&snapshot)..range.end.to_offset_utf16(&snapshot)
                })
                .collect(),
        )
    }

    /// Replaces the editor's selections with the provided `text`, applying the
    /// given `autoindent_mode` (`None` will skip autoindentation).
    ///
    /// Early returns if the editor is in read-only mode, without applying any
    /// edits.
    pub(super) fn replace_selections(
        &mut self,
        text: &str,
        autoindent_mode: Option<AutoindentMode>,
        window: &mut Window,
        cx: &mut Context<Self>,
        apply_linked_edits: bool,
    ) {
        if self.read_only(cx) {
            return;
        }

        let text: Arc<str> = text.into();
        self.transact(window, cx, |this, window, cx| {
            let old_selections = this.selections.all_adjusted(&this.display_snapshot(cx));
            let linked_edits = if apply_linked_edits {
                this.linked_edits_for_selections(text.clone(), cx)
            } else {
                LinkedEdits::new()
            };

            let selection_anchors = this.buffer.update(cx, |buffer, cx| {
                let anchors = {
                    let snapshot = buffer.read(cx);
                    old_selections
                        .iter()
                        .map(|s| {
                            let anchor = snapshot.anchor_after(s.head());
                            s.map(|_| anchor)
                        })
                        .collect::<Vec<_>>()
                };
                buffer.edit(
                    old_selections
                        .iter()
                        .map(|s| (s.start..s.end, text.clone())),
                    autoindent_mode,
                    cx,
                );
                anchors
            });

            linked_edits.apply(cx);

            this.change_selections(Default::default(), window, cx, |s| {
                s.select_anchors(selection_anchors);
            });

            if apply_linked_edits {
                refresh_linked_ranges(this, window, cx);
            }

            cx.notify();
        });
    }

    /// If any empty selections is touching the start of its innermost containing autoclose
    /// region, expand it to select the brackets.
    pub(super) fn select_autoclose_pair(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let selections = self
            .selections
            .all::<MultiBufferOffset>(&self.display_snapshot(cx));
        let buffer = self.buffer.read(cx).read(cx);
        let new_selections = self
            .selections_with_autoclose_regions(selections, &buffer)
            .map(|(mut selection, region)| {
                if !selection.is_empty() {
                    return selection;
                }

                if let Some(region) = region {
                    let mut range = region.range.to_offset(&buffer);
                    if selection.start == range.start && range.start.0 >= region.pair.start.len() {
                        range.start -= region.pair.start.len();
                        if buffer.contains_str_at(range.start, &region.pair.start)
                            && buffer.contains_str_at(range.end, &region.pair.end)
                        {
                            range.end += region.pair.end.len();
                            selection.start = range.start;
                            selection.end = range.end;

                            return selection;
                        }
                    }
                }

                let always_treat_brackets_as_autoclosed = buffer
                    .language_settings_at(selection.start, cx)
                    .always_treat_brackets_as_autoclosed;

                if !always_treat_brackets_as_autoclosed {
                    return selection;
                }

                if let Some(scope) = buffer.language_scope_at(selection.start) {
                    for (pair, enabled) in scope.brackets() {
                        if !enabled || !pair.close {
                            continue;
                        }

                        if buffer.contains_str_at(selection.start, &pair.end) {
                            let pair_start_len = pair.start.len();
                            if buffer.contains_str_at(
                                selection.start.saturating_sub_usize(pair_start_len),
                                &pair.start,
                            ) {
                                selection.start -= pair_start_len;
                                selection.end += pair.end.len();

                                return selection;
                            }
                        }
                    }
                }

                selection
            })
            .collect();

        drop(buffer);
        self.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
            selections.select(new_selections)
        });
    }

    /// Remove any autoclose regions that no longer contain their selection or have invalid anchors in ranges.
    pub(super) fn invalidate_autoclose_regions(
        &mut self,
        mut selections: &[Selection<Anchor>],
        buffer: &MultiBufferSnapshot,
    ) {
        self.autoclose_regions.retain(|state| {
            if !state.range.start.is_valid(buffer) || !state.range.end.is_valid(buffer) {
                return false;
            }

            let mut i = 0;
            while let Some(selection) = selections.get(i) {
                if selection.end.cmp(&state.range.start, buffer).is_lt() {
                    selections = &selections[1..];
                    continue;
                }
                if selection.start.cmp(&state.range.end, buffer).is_gt() {
                    break;
                }
                if selection.id == state.selection_id {
                    return true;
                } else {
                    i += 1;
                }
            }
            false
        });
    }

    fn set_use_auto_surround(&mut self, auto_surround: bool) {
        self.use_auto_surround = auto_surround;
    }

    fn find_possible_emoji_shortcode_at_position(
        snapshot: &MultiBufferSnapshot,
        position: Point,
    ) -> Option<String> {
        let mut chars = Vec::new();
        let mut found_colon = false;
        for char in snapshot.reversed_chars_at(position).take(100) {
            // Found a possible emoji shortcode in the middle of the buffer
            if found_colon {
                if char.is_whitespace() {
                    chars.reverse();
                    return Some(chars.iter().collect());
                }
                // If the previous character is not a whitespace, we are in the middle of a word
                // and we only want to complete the shortcode if the word is made up of other emojis
                let mut containing_word = String::new();
                for ch in snapshot
                    .reversed_chars_at(position)
                    .skip(chars.len() + 1)
                    .take(100)
                {
                    if ch.is_whitespace() {
                        break;
                    }
                    containing_word.push(ch);
                }
                let containing_word = containing_word.chars().rev().collect::<String>();
                if util::word_consists_of_emojis(containing_word.as_str()) {
                    chars.reverse();
                    return Some(chars.iter().collect());
                }
            }

            if char.is_whitespace() || !char.is_ascii() {
                return None;
            }
            if char == ':' {
                found_colon = true;
            } else {
                chars.push(char);
            }
        }
        // Found a possible emoji shortcode at the beginning of the buffer
        chars.reverse();
        Some(chars.iter().collect())
    }

    /// Iterate the given selections, and for each one, find the smallest surrounding
    /// autoclose region. This uses the ordering of the selections and the autoclose
    /// regions to avoid repeated comparisons.
    fn selections_with_autoclose_regions<'a, D: ToOffset + Clone>(
        &'a self,
        selections: impl IntoIterator<Item = Selection<D>>,
        buffer: &'a MultiBufferSnapshot,
    ) -> impl Iterator<Item = (Selection<D>, Option<&'a AutocloseRegion>)> {
        let mut i = 0;
        let mut regions = self.autoclose_regions.as_slice();
        selections.into_iter().map(move |selection| {
            let range = selection.start.to_offset(buffer)..selection.end.to_offset(buffer);

            let mut enclosing = None;
            while let Some(pair_state) = regions.get(i) {
                if pair_state.range.end.to_offset(buffer) < range.start {
                    regions = &regions[i + 1..];
                    i = 0;
                } else if pair_state.range.start.to_offset(buffer) > range.end {
                    break;
                } else {
                    if pair_state.selection_id == selection.id {
                        enclosing = Some(pair_state);
                    }
                    i += 1;
                }
            }

            (selection, enclosing)
        })
    }
}

#[cfg(any(test, feature = "test-support"))]
impl Editor {
    pub fn set_linked_edit_ranges_for_testing(
        &mut self,
        ranges: Vec<(Range<Point>, Vec<Range<Point>>)>,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let Some((buffer, _)) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(self.selections.newest_anchor().start, cx)
        else {
            return None;
        };
        let buffer = buffer.read(cx);
        let buffer_id = buffer.remote_id();
        let mut linked_ranges = Vec::with_capacity(ranges.len());
        for (base_range, linked_ranges_points) in ranges {
            let base_anchor =
                buffer.anchor_before(base_range.start)..buffer.anchor_after(base_range.end);
            let linked_anchors = linked_ranges_points
                .into_iter()
                .map(|range| buffer.anchor_before(range.start)..buffer.anchor_after(range.end))
                .collect();
            linked_ranges.push((base_anchor, linked_anchors));
        }
        let mut map = HashMap::default();
        map.insert(buffer_id, linked_ranges);
        self.linked_edit_ranges = linked_editing_ranges::LinkedEditingRanges(map);
        Some(())
    }

    #[cfg(test)]
    pub(super) fn set_auto_replace_emoji_shortcode(&mut self, auto_replace: bool) {
        self.auto_replace_emoji_shortcode = auto_replace;
    }
}

pub(super) fn is_list_prefix_row(
    row: MultiBufferRow,
    buffer: &MultiBufferSnapshot,
    language: &LanguageScope,
) -> bool {
    let Some((snapshot, range)) = buffer.buffer_line_for_row(row) else {
        return false;
    };

    let num_of_whitespaces = snapshot
        .chars_for_range(range.clone())
        .take_while(|c| c.is_whitespace())
        .count();

    let task_list_prefixes: Vec<_> = language
        .task_list()
        .into_iter()
        .flat_map(|config| {
            config
                .prefixes
                .iter()
                .map(|p| p.as_ref())
                .collect::<Vec<_>>()
        })
        .collect();
    let unordered_list_markers: Vec<_> = language
        .unordered_list()
        .iter()
        .map(|marker| marker.as_ref())
        .collect();
    let all_prefixes: Vec<_> = task_list_prefixes
        .into_iter()
        .chain(unordered_list_markers)
        .collect();
    if let Some(max_prefix_len) = all_prefixes.iter().map(|p| p.len()).max() {
        let candidate: String = snapshot
            .chars_for_range(range.clone())
            .skip(num_of_whitespaces)
            .take(max_prefix_len)
            .collect();
        if all_prefixes
            .iter()
            .any(|prefix| candidate.starts_with(*prefix))
        {
            return true;
        }
    }

    let ordered_list_candidate: String = snapshot
        .chars_for_range(range)
        .skip(num_of_whitespaces)
        .take(ORDERED_LIST_MAX_MARKER_LEN)
        .collect();
    for ordered_config in language.ordered_list() {
        let regex = match Regex::new(&ordered_config.pattern) {
            Ok(r) => r,
            Err(_) => continue,
        };
        if let Some(captures) = regex.captures(&ordered_list_candidate) {
            return captures.get(0).is_some();
        }
    }

    false
}

#[derive(Debug)]
enum NewlineConfig {
    /// Insert newline with optional additional indent and optional extra blank line
    Newline {
        additional_indent: IndentSize,
        extra_line_additional_indent: Option<IndentSize>,
        prevent_auto_indent: bool,
    },
    /// Clear the current line
    ClearCurrentLine,
    /// Unindent the current line and add continuation
    UnindentCurrentLine { continuation: Arc<str> },
}

impl NewlineConfig {
    fn has_extra_line(&self) -> bool {
        matches!(
            self,
            Self::Newline {
                extra_line_additional_indent: Some(_),
                ..
            }
        )
    }

    fn insert_extra_newline_brackets(
        buffer: &MultiBufferSnapshot,
        range: Range<MultiBufferOffset>,
        language: &language::LanguageScope,
    ) -> bool {
        let leading_whitespace_len = buffer
            .reversed_chars_at(range.start)
            .take_while(|c| c.is_whitespace() && *c != '\n')
            .map(|c| c.len_utf8())
            .sum::<usize>();
        let trailing_whitespace_len = buffer
            .chars_at(range.end)
            .take_while(|c| c.is_whitespace() && *c != '\n')
            .map(|c| c.len_utf8())
            .sum::<usize>();
        let range = range.start - leading_whitespace_len..range.end + trailing_whitespace_len;

        language.brackets().any(|(pair, enabled)| {
            let pair_start = pair.start.trim_end();
            let pair_end = pair.end.trim_start();

            enabled
                && pair.newline
                && buffer.contains_str_at(range.end, pair_end)
                && buffer.contains_str_at(
                    range.start.saturating_sub_usize(pair_start.len()),
                    pair_start,
                )
        })
    }

    fn insert_extra_newline_tree_sitter(
        buffer: &MultiBufferSnapshot,
        range: Range<MultiBufferOffset>,
    ) -> bool {
        let (buffer, range) = match buffer
            .range_to_buffer_ranges(range.start..range.end)
            .as_slice()
        {
            [(buffer_snapshot, range, _)] => (buffer_snapshot.clone(), range.clone()),
            _ => return false,
        };
        let pair = {
            let mut result: Option<BracketMatch<usize>> = None;

            for pair in buffer
                .all_bracket_ranges(range.start.0..range.end.0)
                .filter(move |pair| {
                    pair.open_range.start <= range.start.0 && pair.close_range.end >= range.end.0
                })
            {
                let len = pair.close_range.end - pair.open_range.start;

                if let Some(existing) = &result {
                    let existing_len = existing.close_range.end - existing.open_range.start;
                    if len > existing_len {
                        continue;
                    }
                }

                result = Some(pair);
            }

            result
        };
        let Some(pair) = pair else {
            return false;
        };
        pair.newline_only
            && buffer
                .chars_for_range(pair.open_range.end..range.start.0)
                .chain(buffer.chars_for_range(range.end.0..pair.close_range.start))
                .all(|c| c.is_whitespace() && c != '\n')
    }
}

fn comment_delimiter_for_newline(
    start_point: &Point,
    buffer: &MultiBufferSnapshot,
    language: &LanguageScope,
) -> Option<Arc<str>> {
    let delimiters = language.line_comment_prefixes();
    let max_len_of_delimiter = delimiters.iter().map(|delimiter| delimiter.len()).max()?;
    let (snapshot, range) = buffer.buffer_line_for_row(MultiBufferRow(start_point.row))?;

    let num_of_whitespaces = snapshot
        .chars_for_range(range.clone())
        .take_while(|c| c.is_whitespace())
        .count();
    let comment_candidate = snapshot
        .chars_for_range(range.clone())
        .skip(num_of_whitespaces)
        .take(max_len_of_delimiter + 2)
        .collect::<String>();
    let (delimiter, trimmed_len, is_repl) = delimiters
        .iter()
        .filter_map(|delimiter| {
            let prefix = delimiter.trim_end();
            if comment_candidate.starts_with(prefix) {
                let is_repl = if let Some(stripped_comment) = comment_candidate.strip_prefix(prefix)
                {
                    stripped_comment.starts_with(" %%")
                } else {
                    false
                };
                Some((delimiter, prefix.len(), is_repl))
            } else {
                None
            }
        })
        .max_by_key(|(_, len, _)| *len)?;

    if let Some(BlockCommentConfig {
        start: block_start, ..
    }) = language.block_comment()
    {
        let block_start_trimmed = block_start.trim_end();
        if block_start_trimmed.starts_with(delimiter.trim_end()) {
            let line_content = snapshot
                .chars_for_range(range.clone())
                .skip(num_of_whitespaces)
                .take(block_start_trimmed.len())
                .collect::<String>();

            if line_content.starts_with(block_start_trimmed) {
                return None;
            }
        }
    }

    let cursor_is_placed_after_comment_marker =
        num_of_whitespaces + trimmed_len <= start_point.column as usize;
    if cursor_is_placed_after_comment_marker {
        if !is_repl {
            return Some(delimiter.clone());
        }

        let line_content_after_cursor: String = snapshot
            .chars_for_range(range)
            .skip(start_point.column as usize)
            .collect();

        if line_content_after_cursor.trim().is_empty() {
            return None;
        } else {
            return Some(delimiter.clone());
        }
    } else {
        None
    }
}

fn documentation_delimiter_for_newline(
    start_point: &Point,
    buffer: &MultiBufferSnapshot,
    language: &LanguageScope,
    newline_config: &mut NewlineConfig,
) -> Option<Arc<str>> {
    let BlockCommentConfig {
        start: start_tag,
        end: end_tag,
        prefix: delimiter,
        tab_size: len,
    } = language.documentation_comment()?;
    let is_within_block_comment = buffer
        .language_scope_at(*start_point)
        .is_some_and(|scope| scope.override_name() == Some("comment"));
    if !is_within_block_comment {
        return None;
    }

    let (snapshot, range) = buffer.buffer_line_for_row(MultiBufferRow(start_point.row))?;

    let num_of_whitespaces = snapshot
        .chars_for_range(range.clone())
        .take_while(|c| c.is_whitespace())
        .count();

    // It is safe to use a column from MultiBufferPoint in context of a single buffer ranges, because we're only ever looking at a single line at a time.
    let column = start_point.column;
    let cursor_is_after_start_tag = {
        let start_tag_len = start_tag.len();
        let start_tag_line = snapshot
            .chars_for_range(range.clone())
            .skip(num_of_whitespaces)
            .take(start_tag_len)
            .collect::<String>();
        if start_tag_line.starts_with(start_tag.as_ref()) {
            num_of_whitespaces + start_tag_len <= column as usize
        } else {
            false
        }
    };

    let cursor_is_after_delimiter = {
        let delimiter_trim = delimiter.trim_end();
        let delimiter_line = snapshot
            .chars_for_range(range.clone())
            .skip(num_of_whitespaces)
            .take(delimiter_trim.len())
            .collect::<String>();
        if delimiter_line.starts_with(delimiter_trim) {
            num_of_whitespaces + delimiter_trim.len() <= column as usize
        } else {
            false
        }
    };

    let mut needs_extra_line = false;
    let mut extra_line_additional_indent = IndentSize::spaces(0);

    let cursor_is_before_end_tag_if_exists = {
        let mut char_position = 0u32;
        let mut end_tag_offset = None;

        'outer: for chunk in snapshot.text_for_range(range) {
            if let Some(byte_pos) = chunk.find(&**end_tag) {
                let chars_before_match = chunk[..byte_pos].chars().count() as u32;
                end_tag_offset = Some(char_position + chars_before_match);
                break 'outer;
            }
            char_position += chunk.chars().count() as u32;
        }

        if let Some(end_tag_offset) = end_tag_offset {
            let cursor_is_before_end_tag = column <= end_tag_offset;
            if cursor_is_after_start_tag {
                if cursor_is_before_end_tag {
                    needs_extra_line = true;
                }
                let cursor_is_at_start_of_end_tag = column == end_tag_offset;
                if cursor_is_at_start_of_end_tag {
                    extra_line_additional_indent.len = *len;
                }
            }
            cursor_is_before_end_tag
        } else {
            true
        }
    };

    if (cursor_is_after_start_tag || cursor_is_after_delimiter)
        && cursor_is_before_end_tag_if_exists
    {
        let additional_indent = if cursor_is_after_start_tag {
            IndentSize::spaces(*len)
        } else {
            IndentSize::spaces(0)
        };

        *newline_config = NewlineConfig::Newline {
            additional_indent,
            extra_line_additional_indent: if needs_extra_line {
                Some(extra_line_additional_indent)
            } else {
                None
            },
            prevent_auto_indent: true,
        };
        Some(delimiter.clone())
    } else {
        None
    }
}

fn list_delimiter_for_newline(
    start_point: &Point,
    buffer: &MultiBufferSnapshot,
    language: &LanguageScope,
    newline_config: &mut NewlineConfig,
) -> Option<Arc<str>> {
    let (snapshot, range) = buffer.buffer_line_for_row(MultiBufferRow(start_point.row))?;

    let num_of_whitespaces = snapshot
        .chars_for_range(range.clone())
        .take_while(|c| c.is_whitespace())
        .count();

    let task_list_entries: Vec<_> = language
        .task_list()
        .into_iter()
        .flat_map(|config| {
            config
                .prefixes
                .iter()
                .map(|prefix| (prefix.as_ref(), config.continuation.as_ref()))
        })
        .collect();
    let unordered_list_entries: Vec<_> = language
        .unordered_list()
        .iter()
        .map(|marker| (marker.as_ref(), marker.as_ref()))
        .collect();

    let all_entries: Vec<_> = task_list_entries
        .into_iter()
        .chain(unordered_list_entries)
        .collect();

    if let Some(max_prefix_len) = all_entries.iter().map(|(p, _)| p.len()).max() {
        let candidate: String = snapshot
            .chars_for_range(range.clone())
            .skip(num_of_whitespaces)
            .take(max_prefix_len)
            .collect();

        if let Some((prefix, continuation)) = all_entries
            .iter()
            .filter(|(prefix, _)| candidate.starts_with(*prefix))
            .max_by_key(|(prefix, _)| prefix.len())
        {
            let end_of_prefix = num_of_whitespaces + prefix.len();
            let cursor_is_after_prefix = end_of_prefix <= start_point.column as usize;
            let has_content_after_marker = snapshot
                .chars_for_range(range)
                .skip(end_of_prefix)
                .any(|c| !c.is_whitespace());

            if has_content_after_marker && cursor_is_after_prefix {
                return Some((*continuation).into());
            }

            if start_point.column as usize == end_of_prefix {
                if num_of_whitespaces == 0 {
                    *newline_config = NewlineConfig::ClearCurrentLine;
                } else {
                    *newline_config = NewlineConfig::UnindentCurrentLine {
                        continuation: (*continuation).into(),
                    };
                }
            }

            return None;
        }
    }

    let candidate: String = snapshot
        .chars_for_range(range.clone())
        .skip(num_of_whitespaces)
        .take(ORDERED_LIST_MAX_MARKER_LEN)
        .collect();

    for ordered_config in language.ordered_list() {
        let regex = match Regex::new(&ordered_config.pattern) {
            Ok(r) => r,
            Err(_) => continue,
        };

        if let Some(captures) = regex.captures(&candidate) {
            let full_match = captures.get(0)?;
            let marker_len = full_match.len();
            let end_of_prefix = num_of_whitespaces + marker_len;
            let cursor_is_after_prefix = end_of_prefix <= start_point.column as usize;

            let has_content_after_marker = snapshot
                .chars_for_range(range)
                .skip(end_of_prefix)
                .any(|c| !c.is_whitespace());

            if has_content_after_marker && cursor_is_after_prefix {
                let number: u32 = captures.get(1)?.as_str().parse().ok()?;
                let continuation = ordered_config
                    .format
                    .replace("{1}", &(number + 1).to_string());
                return Some(continuation.into());
            }

            if start_point.column as usize == end_of_prefix {
                let continuation = ordered_config.format.replace("{1}", "1");
                if num_of_whitespaces == 0 {
                    *newline_config = NewlineConfig::ClearCurrentLine;
                } else {
                    *newline_config = NewlineConfig::UnindentCurrentLine {
                        continuation: continuation.into(),
                    };
                }
            }

            return None;
        }
    }

    None
}

impl EntityInputHandler for Editor {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        adjusted_range: &mut Option<Range<usize>>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<String> {
        let snapshot = self.buffer.read(cx).read(cx);
        let start = snapshot.clip_offset_utf16(
            MultiBufferOffsetUtf16(OffsetUtf16(range_utf16.start)),
            Bias::Left,
        );
        let end = snapshot.clip_offset_utf16(
            MultiBufferOffsetUtf16(OffsetUtf16(range_utf16.end)),
            Bias::Right,
        );
        if (start.0.0..end.0.0) != range_utf16 {
            adjusted_range.replace(start.0.0..end.0.0);
        }
        Some(snapshot.text_for_range(start..end).collect())
    }

    fn selected_text_range(
        &mut self,
        ignore_disabled_input: bool,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        // Prevent the IME menu from appearing when holding down an alphabetic key
        // while input is disabled.
        if !ignore_disabled_input && !self.input_enabled {
            return None;
        }

        let selection = self
            .selections
            .newest::<MultiBufferOffsetUtf16>(&self.display_snapshot(cx));
        let range = selection.range();

        Some(UTF16Selection {
            range: range.start.0.0..range.end.0.0,
            reversed: selection.reversed,
        })
    }

    fn marked_text_range(&self, _: &mut Window, cx: &mut Context<Self>) -> Option<Range<usize>> {
        let snapshot = self.buffer.read(cx).read(cx);
        let range = self
            .text_highlights(HighlightKey::InputComposition, cx)?
            .1
            .first()?;
        Some(range.start.to_offset_utf16(&snapshot).0.0..range.end.to_offset_utf16(&snapshot).0.0)
    }

    fn unmark_text(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.clear_highlights(HighlightKey::InputComposition, cx);
        self.ime_transaction.take();
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.input_enabled {
            cx.emit(EditorEvent::InputIgnored { text: text.into() });
            return;
        }

        self.transact(window, cx, |this, window, cx| {
            let new_selected_ranges = if let Some(range_utf16) = range_utf16 {
                if let Some(marked_ranges) = this.marked_text_ranges(cx) {
                    // During IME composition, macOS reports the replacement range
                    // relative to the first marked region (the only one visible via
                    // marked_text_range). The correct targets for replacement are the
                    // marked ranges themselves — one per cursor — so use them directly.
                    Some(marked_ranges)
                } else if range_utf16.start == range_utf16.end {
                    // An empty replacement range means "insert at cursor" with no text
                    // to replace. macOS reports the cursor position from its own
                    // (single-cursor) view of the buffer, which diverges from our actual
                    // cursor positions after multi-cursor edits have shifted offsets.
                    // Treating this as range_utf16=None lets each cursor insert in place.
                    None
                } else {
                    // Outside of IME composition (e.g. Accessibility Keyboard word
                    // completion), the range is an absolute document offset for the
                    // newest cursor. Fan it out to all cursors via
                    // selection_replacement_ranges, which applies the delta relative
                    // to the newest selection to every cursor.
                    let range_utf16 = MultiBufferOffsetUtf16(OffsetUtf16(range_utf16.start))
                        ..MultiBufferOffsetUtf16(OffsetUtf16(range_utf16.end));
                    Some(this.selection_replacement_ranges(range_utf16, cx))
                }
            } else {
                this.marked_text_ranges(cx)
            };

            let range_to_replace = new_selected_ranges.as_ref().and_then(|ranges_to_replace| {
                let newest_selection_id = this.selections.newest_anchor().id;
                this.selections
                    .all::<MultiBufferOffsetUtf16>(&this.display_snapshot(cx))
                    .iter()
                    .zip(ranges_to_replace.iter())
                    .find_map(|(selection, range)| {
                        if selection.id == newest_selection_id {
                            Some(
                                (range.start.0.0 as isize - selection.head().0.0 as isize)
                                    ..(range.end.0.0 as isize - selection.head().0.0 as isize),
                            )
                        } else {
                            None
                        }
                    })
            });

            cx.emit(EditorEvent::InputHandled {
                utf16_range_to_replace: range_to_replace,
                text: text.into(),
            });

            if let Some(new_selected_ranges) = new_selected_ranges {
                // Only backspace if at least one range covers actual text. When all
                // ranges are empty (e.g. a trailing-space insertion from Accessibility
                // Keyboard sends replacementRange=cursor..cursor), backspace would
                // incorrectly delete the character just before the cursor.
                let should_backspace = new_selected_ranges.iter().any(|r| r.start != r.end);
                this.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select_ranges(new_selected_ranges)
                });
                if should_backspace {
                    this.backspace(&Default::default(), window, cx);
                }
            }

            this.handle_input(text, window, cx);
        });

        if let Some(transaction) = self.ime_transaction {
            self.buffer.update(cx, |buffer, cx| {
                buffer.group_until_transaction(transaction, cx);
            });
        }

        self.unmark_text(window, cx);
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.input_enabled {
            return;
        }

        let transaction = self.transact(window, cx, |this, window, cx| {
            let ranges_to_replace = if let Some(mut marked_ranges) = this.marked_text_ranges(cx) {
                let snapshot = this.buffer.read(cx).read(cx);
                if let Some(relative_range_utf16) = range_utf16.as_ref() {
                    for marked_range in &mut marked_ranges {
                        marked_range.end = marked_range.start + relative_range_utf16.end;
                        marked_range.start += relative_range_utf16.start;
                        marked_range.start =
                            snapshot.clip_offset_utf16(marked_range.start, Bias::Left);
                        marked_range.end =
                            snapshot.clip_offset_utf16(marked_range.end, Bias::Right);
                    }
                }
                Some(marked_ranges)
            } else if let Some(range_utf16) = range_utf16 {
                let range_utf16 = MultiBufferOffsetUtf16(OffsetUtf16(range_utf16.start))
                    ..MultiBufferOffsetUtf16(OffsetUtf16(range_utf16.end));
                Some(this.selection_replacement_ranges(range_utf16, cx))
            } else {
                None
            };

            let range_to_replace = ranges_to_replace.as_ref().and_then(|ranges_to_replace| {
                let newest_selection_id = this.selections.newest_anchor().id;
                this.selections
                    .all::<MultiBufferOffsetUtf16>(&this.display_snapshot(cx))
                    .iter()
                    .zip(ranges_to_replace.iter())
                    .find_map(|(selection, range)| {
                        if selection.id == newest_selection_id {
                            Some(
                                (range.start.0.0 as isize - selection.head().0.0 as isize)
                                    ..(range.end.0.0 as isize - selection.head().0.0 as isize),
                            )
                        } else {
                            None
                        }
                    })
            });

            cx.emit(EditorEvent::InputHandled {
                utf16_range_to_replace: range_to_replace,
                text: text.into(),
            });

            if let Some(ranges) = ranges_to_replace {
                this.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges(ranges)
                });
            }

            let marked_ranges = {
                let snapshot = this.buffer.read(cx).read(cx);
                this.selections
                    .disjoint_anchors_arc()
                    .iter()
                    .map(|selection| {
                        selection.start.bias_left(&snapshot)..selection.end.bias_right(&snapshot)
                    })
                    .collect::<Vec<_>>()
            };

            if text.is_empty() {
                this.unmark_text(window, cx);
            } else {
                this.highlight_text(
                    HighlightKey::InputComposition,
                    marked_ranges.clone(),
                    HighlightStyle {
                        underline: Some(UnderlineStyle {
                            thickness: px(1.),
                            color: None,
                            wavy: false,
                        }),
                        ..Default::default()
                    },
                    cx,
                );
            }

            // Disable auto-closing when composing text (i.e. typing a `"` on a Brazilian keyboard)
            let use_autoclose = this.use_autoclose;
            let use_auto_surround = this.use_auto_surround;
            this.set_use_autoclose(false);
            this.set_use_auto_surround(false);
            this.handle_input(text, window, cx);
            this.set_use_autoclose(use_autoclose);
            this.set_use_auto_surround(use_auto_surround);

            if let Some(new_selected_range) = new_selected_range_utf16 {
                let snapshot = this.buffer.read(cx).read(cx);
                let new_selected_ranges = marked_ranges
                    .into_iter()
                    .map(|marked_range| {
                        let insertion_start = marked_range.start.to_offset_utf16(&snapshot).0;
                        let new_start = MultiBufferOffsetUtf16(OffsetUtf16(
                            insertion_start.0 + new_selected_range.start,
                        ));
                        let new_end = MultiBufferOffsetUtf16(OffsetUtf16(
                            insertion_start.0 + new_selected_range.end,
                        ));
                        snapshot.clip_offset_utf16(new_start, Bias::Left)
                            ..snapshot.clip_offset_utf16(new_end, Bias::Right)
                    })
                    .collect::<Vec<_>>();

                drop(snapshot);
                this.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select_ranges(new_selected_ranges)
                });
            }
        });

        self.ime_transaction = self.ime_transaction.or(transaction);
        if let Some(transaction) = self.ime_transaction {
            self.buffer.update(cx, |buffer, cx| {
                buffer.group_until_transaction(transaction, cx);
            });
        }

        if self
            .text_highlights(HighlightKey::InputComposition, cx)
            .is_none()
        {
            self.ime_transaction.take();
        }
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        element_bounds: gpui::Bounds<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<gpui::Bounds<Pixels>> {
        let text_layout_details = self.text_layout_details(window, cx);
        let CharacterDimensions {
            em_width,
            em_advance,
            line_height,
        } = self.character_dimensions(window, cx);

        let snapshot = self.snapshot(window, cx);
        let scroll_position = snapshot.scroll_position();
        let scroll_left = scroll_position.x * ScrollOffset::from(em_advance);

        let start =
            MultiBufferOffsetUtf16(OffsetUtf16(range_utf16.start)).to_display_point(&snapshot);
        let x = Pixels::from(
            ScrollOffset::from(
                snapshot.x_for_display_point(start, &text_layout_details)
                    + self.gutter_dimensions.full_width(),
            ) - scroll_left,
        );
        let y = line_height * (start.row().as_f64() - scroll_position.y) as f32;

        Some(Bounds {
            origin: element_bounds.origin + point(x, y),
            size: size(em_width, line_height),
        })
    }

    fn character_index_for_point(
        &mut self,
        point: gpui::Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let position_map = self.last_position_map.as_ref()?;
        if !position_map.text_hitbox.contains(&point) {
            return None;
        }
        let display_point = position_map.point_for_position(point).previous_valid;
        let anchor = position_map
            .snapshot
            .display_point_to_anchor(display_point, Bias::Left);
        let utf16_offset = anchor.to_offset_utf16(&position_map.snapshot.buffer_snapshot());
        Some(utf16_offset.0.0)
    }

    fn accepts_text_input(&self, _window: &mut Window, _cx: &mut Context<Self>) -> bool {
        self.expects_character_input
    }
}
