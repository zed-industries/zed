use super::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClipboardSelection {
    /// The number of bytes in this selection.
    pub len: usize,
    /// Whether this was a full-line selection.
    pub is_entire_line: bool,
    /// The indentation of the first line when this content was originally copied.
    pub first_line_indent: u32,
    #[serde(default)]
    pub file_path: Option<PathBuf>,
    #[serde(default)]
    pub line_range: Option<RangeInclusive<u32>>,
}

impl ClipboardSelection {
    pub fn for_buffer(
        len: usize,
        is_entire_line: bool,
        range: Range<Point>,
        buffer: &MultiBufferSnapshot,
        project: Option<&Entity<Project>>,
        cx: &App,
    ) -> Self {
        let first_line_indent = buffer
            .indent_size_for_line(MultiBufferRow(range.start.row))
            .len;

        let file_path = util::maybe!({
            let project = project?.read(cx);
            let file = buffer.file_at(range.start)?;
            let project_path = ProjectPath {
                worktree_id: file.worktree_id(cx),
                path: file.path().clone(),
            };
            project.absolute_path(&project_path, cx)
        });

        let line_range = if file_path.is_some() {
            buffer
                .range_to_buffer_range(range)
                .map(|(_, buffer_range)| buffer_range.start.row..=buffer_range.end.row)
        } else {
            None
        };

        Self {
            len,
            is_entire_line,
            first_line_indent,
            file_path,
            line_range,
        }
    }
}

impl Editor {
    pub fn do_paste(
        &mut self,
        text: &String,
        clipboard_selections: Option<Vec<ClipboardSelection>>,
        handle_entire_lines: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }

        self.finalize_last_transaction(cx);

        let clipboard_text = Cow::Borrowed(text.as_str());

        self.transact(window, cx, |this, window, cx| {
            let had_active_edit_prediction = this.has_active_edit_prediction();
            let display_map = this.display_snapshot(cx);
            let old_selections = this.selections.all::<MultiBufferOffset>(&display_map);
            let cursor_offset = this
                .selections
                .last::<MultiBufferOffset>(&display_map)
                .head();

            if let Some(mut clipboard_selections) = clipboard_selections {
                let all_selections_were_entire_line =
                    clipboard_selections.iter().all(|s| s.is_entire_line);
                let first_selection_indent_column =
                    clipboard_selections.first().map(|s| s.first_line_indent);
                if clipboard_selections.len() != old_selections.len() {
                    clipboard_selections.drain(..);
                }
                let mut auto_indent_on_paste = true;

                this.buffer.update(cx, |buffer, cx| {
                    let snapshot = buffer.read(cx);
                    auto_indent_on_paste = snapshot
                        .language_settings_at(cursor_offset, cx)
                        .auto_indent_on_paste;

                    let mut start_offset = 0;
                    let mut edits = Vec::new();
                    let mut original_indent_columns = Vec::new();
                    for (ix, selection) in old_selections.iter().enumerate() {
                        let to_insert;
                        let entire_line;
                        let original_indent_column;
                        if let Some(clipboard_selection) = clipboard_selections.get(ix) {
                            let end_offset = start_offset + clipboard_selection.len;
                            to_insert = &clipboard_text[start_offset..end_offset];
                            entire_line = clipboard_selection.is_entire_line;
                            start_offset = if entire_line {
                                end_offset
                            } else {
                                end_offset + 1
                            };
                            original_indent_column = Some(clipboard_selection.first_line_indent);
                        } else {
                            to_insert = &*clipboard_text;
                            entire_line = all_selections_were_entire_line;
                            original_indent_column = first_selection_indent_column
                        }

                        let (range, to_insert) =
                            if selection.is_empty() && handle_entire_lines && entire_line {
                                // If the corresponding selection was empty when this slice of the
                                // clipboard text was written, then the entire line containing the
                                // selection was copied. If this selection is also currently empty,
                                // then paste the line before the current line of the buffer.
                                let column = selection.start.to_point(&snapshot).column as usize;
                                let line_start = selection.start - column;
                                (line_start..line_start, Cow::Borrowed(to_insert))
                            } else {
                                let language = snapshot.language_at(selection.head());
                                let range = selection.range();
                                if let Some(language) = language
                                    && language.name() == "Markdown"
                                {
                                    edit_for_markdown_paste(
                                        &snapshot,
                                        range,
                                        to_insert,
                                        url::Url::parse(to_insert).ok(),
                                    )
                                } else {
                                    (range, Cow::Borrowed(to_insert))
                                }
                            };

                        edits.push((range, to_insert));
                        original_indent_columns.push(original_indent_column);
                    }
                    drop(snapshot);

                    buffer.edit(
                        edits,
                        if auto_indent_on_paste {
                            Some(AutoindentMode::Block {
                                original_indent_columns,
                            })
                        } else {
                            None
                        },
                        cx,
                    );
                });

                let selections = this
                    .selections
                    .all::<MultiBufferOffset>(&this.display_snapshot(cx));
                this.change_selections(Default::default(), window, cx, |s| s.select(selections));
            } else {
                let url = url::Url::parse(&clipboard_text).ok();

                let auto_indent_mode = if !clipboard_text.is_empty() {
                    Some(AutoindentMode::Block {
                        original_indent_columns: Vec::new(),
                    })
                } else {
                    None
                };

                let selection_anchors = this.buffer.update(cx, |buffer, cx| {
                    let snapshot = buffer.snapshot(cx);

                    let anchors = old_selections
                        .iter()
                        .map(|s| {
                            let anchor = snapshot.anchor_after(s.head());
                            s.map(|_| anchor)
                        })
                        .collect::<Vec<_>>();

                    let mut edits = Vec::new();

                    // When pasting text without metadata (e.g. copied from an
                    // external editor using multiple cursors) and the number of
                    // lines matches the number of selections, distribute one
                    // line per cursor instead of pasting the whole text at each.
                    let lines: Vec<&str> = clipboard_text.split('\n').collect();
                    let distribute_lines =
                        old_selections.len() > 1 && lines.len() == old_selections.len();

                    for (ix, selection) in old_selections.iter().enumerate() {
                        let language = snapshot.language_at(selection.head());
                        let range = selection.range();

                        let text_for_cursor: &str = if distribute_lines {
                            lines[ix]
                        } else {
                            &clipboard_text
                        };

                        let (edit_range, edit_text) = if let Some(language) = language
                            && language.name() == "Markdown"
                        {
                            edit_for_markdown_paste(&snapshot, range, text_for_cursor, url.clone())
                        } else {
                            (range, Cow::Borrowed(text_for_cursor))
                        };

                        edits.push((edit_range, edit_text));
                    }

                    drop(snapshot);
                    buffer.edit(edits, auto_indent_mode, cx);

                    anchors
                });

                this.change_selections(Default::default(), window, cx, |s| {
                    s.select_anchors(selection_anchors);
                });
            }

            //   🤔                 |    ..     | show_in_menu |
            // | ..                  |   true        true
            // | had_edit_prediction |   false       true

            let trigger_in_words =
                this.show_edit_predictions_in_menu() || !had_active_edit_prediction;

            this.trigger_completion_on_input(text, trigger_in_words, window, cx);
        });
    }

    pub fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(item) = cx.read_from_clipboard() {
            self.paste_item(&item, window, cx);
        }
    }

    pub fn paste_item(
        &mut self,
        item: &ClipboardItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }
        let clipboard_string = item.entries().iter().find_map(|entry| match entry {
            ClipboardEntry::String(s) => Some(s),
            _ => None,
        });
        match clipboard_string {
            Some(clipboard_string) => self.do_paste(
                clipboard_string.text(),
                clipboard_string.metadata_json::<Vec<ClipboardSelection>>(),
                true,
                window,
                cx,
            ),
            _ => self.do_paste(&item.text().unwrap_or_default(), None, true, window, cx),
        }
    }

    pub(super) fn cut_common(
        &mut self,
        cut_no_selection_line: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ClipboardItem {
        let mut text = String::new();
        let buffer = self.buffer.read(cx).snapshot(cx);
        let mut selections = self.selections.all::<Point>(&self.display_snapshot(cx));
        let mut clipboard_selections = Vec::with_capacity(selections.len());
        {
            let max_point = buffer.max_point();
            let mut is_first = true;
            let mut prev_selection_was_entire_line = false;
            for selection in &mut selections {
                let is_entire_line =
                    (selection.is_empty() && cut_no_selection_line) || self.selections.line_mode();
                if is_entire_line {
                    selection.start = Point::new(selection.start.row, 0);
                    if !selection.is_empty() && selection.end.column == 0 {
                        selection.end = cmp::min(max_point, selection.end);
                    } else {
                        selection.end = cmp::min(max_point, Point::new(selection.end.row + 1, 0));
                    }
                    selection.goal = SelectionGoal::None;
                }
                if is_first {
                    is_first = false;
                } else if !prev_selection_was_entire_line {
                    text += "\n";
                }
                prev_selection_was_entire_line = is_entire_line;
                let mut len = 0;
                for chunk in buffer.text_for_range(selection.start..selection.end) {
                    text.push_str(chunk);
                    len += chunk.len();
                }

                clipboard_selections.push(ClipboardSelection::for_buffer(
                    len,
                    is_entire_line,
                    selection.range(),
                    &buffer,
                    self.project.as_ref(),
                    cx,
                ));
            }
        }

        self.transact(window, cx, |this, window, cx| {
            this.change_selections(Default::default(), window, cx, |s| {
                s.select(selections);
            });
            this.insert("", window, cx);
        });
        ClipboardItem::new_string_with_json_metadata(text, clipboard_selections)
    }

    pub(super) fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if self.read_only(cx) {
            return;
        }
        let item = self.cut_common(true, window, cx);
        cx.write_to_clipboard(item);
    }

    pub(super) fn kill_ring_cut(
        &mut self,
        _: &KillRingCut,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }
        self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.move_with(&mut |snapshot, sel| {
                if sel.is_empty() {
                    sel.end = DisplayPoint::new(sel.end.row(), snapshot.line_len(sel.end.row()));
                }
                if sel.is_empty() {
                    sel.end = DisplayPoint::new(sel.end.row() + 1_u32, 0);
                }
            });
        });
        let item = self.cut_common(false, window, cx);
        cx.set_global(KillRing(item))
    }

    pub(super) fn kill_ring_yank(
        &mut self,
        _: &KillRingYank,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let (text, metadata) = if let Some(KillRing(item)) = cx.try_global() {
            if let Some(ClipboardEntry::String(kill_ring)) = item.entries().first() {
                (kill_ring.text().to_string(), kill_ring.metadata_json())
            } else {
                return;
            }
        } else {
            return;
        };
        self.do_paste(&text, metadata, false, window, cx);
    }

    pub(super) fn copy_and_trim(
        &mut self,
        _: &CopyAndTrim,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.do_copy(true, cx);
    }

    pub(super) fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        self.do_copy(false, cx);
    }

    pub(super) fn diff_clipboard_with_selection(
        &mut self,
        _: &DiffClipboardWithSelection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selections = self
            .selections
            .all::<MultiBufferOffset>(&self.display_snapshot(cx));

        if selections.is_empty() {
            log::warn!("There should always be at least one selection in Zed. This is a bug.");
            return;
        };

        let clipboard_text = cx.read_from_clipboard().and_then(|item| {
            item.entries().iter().find_map(|entry| match entry {
                ClipboardEntry::String(text) => Some(text.text().to_string()),
                _ => None,
            })
        });

        let Some(clipboard_text) = clipboard_text else {
            log::warn!("Clipboard doesn't contain text.");
            return;
        };

        window.dispatch_action(
            Box::new(DiffClipboardWithSelectionData {
                clipboard_text,
                editor: cx.entity(),
            }),
            cx,
        );
    }

    fn do_copy(&self, strip_leading_indents: bool, cx: &mut Context<Self>) {
        let selections = self.selections.all::<Point>(&self.display_snapshot(cx));
        let buffer = self.buffer.read(cx).read(cx);
        let mut text = String::new();
        let mut clipboard_selections = Vec::with_capacity(selections.len());

        let max_point = buffer.max_point();
        let mut is_first = true;
        for selection in &selections {
            let mut start = selection.start;
            let mut end = selection.end;
            let is_entire_line = selection.is_empty() || self.selections.line_mode();
            let mut add_trailing_newline = false;
            if is_entire_line {
                start = Point::new(start.row, 0);
                let next_line_start = Point::new(end.row + 1, 0);
                if next_line_start <= max_point {
                    end = next_line_start;
                } else {
                    // We're on the last line without a trailing newline.
                    // Copy to the end of the line and add a newline afterwards.
                    end = Point::new(end.row, buffer.line_len(MultiBufferRow(end.row)));
                    add_trailing_newline = true;
                }
            }

            let mut trimmed_selections = Vec::new();
            if strip_leading_indents && end.row.saturating_sub(start.row) > 0 {
                let row = MultiBufferRow(start.row);
                let first_indent = buffer.indent_size_for_line(row);
                if first_indent.len == 0 || start.column > first_indent.len {
                    trimmed_selections.push(start..end);
                } else {
                    trimmed_selections.push(
                        Point::new(row.0, first_indent.len)
                            ..Point::new(row.0, buffer.line_len(row)),
                    );
                    for row in start.row + 1..=end.row {
                        let mut line_len = buffer.line_len(MultiBufferRow(row));
                        if row == end.row {
                            line_len = end.column;
                        }
                        if line_len == 0 {
                            trimmed_selections.push(Point::new(row, 0)..Point::new(row, line_len));
                            continue;
                        }
                        let row_indent_size = buffer.indent_size_for_line(MultiBufferRow(row));
                        if row_indent_size.len >= first_indent.len {
                            trimmed_selections
                                .push(Point::new(row, first_indent.len)..Point::new(row, line_len));
                        } else {
                            trimmed_selections.clear();
                            trimmed_selections.push(start..end);
                            break;
                        }
                    }
                }
            } else {
                trimmed_selections.push(start..end);
            }

            let is_multiline_trim = trimmed_selections.len() > 1;
            let mut selection_len: usize = 0;
            let prev_selection_was_entire_line = is_entire_line && !is_multiline_trim;

            for trimmed_range in trimmed_selections {
                if is_first {
                    is_first = false;
                } else if is_multiline_trim || !prev_selection_was_entire_line {
                    text.push('\n');
                    if is_multiline_trim {
                        selection_len += 1;
                    }
                }
                for chunk in buffer.text_for_range(trimmed_range.start..trimmed_range.end) {
                    text.push_str(chunk);
                    selection_len += chunk.len();
                }
                if add_trailing_newline {
                    text.push('\n');
                    selection_len += 1;
                }
            }

            clipboard_selections.push(ClipboardSelection::for_buffer(
                selection_len,
                is_entire_line,
                start..end,
                &buffer,
                self.project.as_ref(),
                cx,
            ));
        }

        cx.write_to_clipboard(ClipboardItem::new_string_with_json_metadata(
            text,
            clipboard_selections,
        ));
    }
}

struct KillRing(ClipboardItem);
impl Global for KillRing {}

fn edit_for_markdown_paste<'a>(
    buffer: &MultiBufferSnapshot,
    range: Range<MultiBufferOffset>,
    to_insert: &'a str,
    url: Option<url::Url>,
) -> (Range<MultiBufferOffset>, Cow<'a, str>) {
    if url.is_none() {
        return (range, Cow::Borrowed(to_insert));
    };

    let old_text = buffer.text_for_range(range.clone()).collect::<String>();

    let new_text = if range.is_empty() || url::Url::parse(&old_text).is_ok() {
        Cow::Borrowed(to_insert)
    } else {
        Cow::Owned(format!("[{old_text}]({to_insert})"))
    };
    (range, new_text)
}
