use super::*;

impl Editor {
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
                                let target = bracket_pair.start.chars().next().unwrap();
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

    #[cfg(any(test, feature = "test-support"))]
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
}
