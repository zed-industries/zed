use super::*;
use crate::deferred_input::CapturedInput;
use futures::future::{LocalBoxFuture, Shared};
use gpui::ClipboardReadError;
use std::future::Future;
use util::rel_path::RelPath;

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

#[derive(Clone)]
pub(super) struct CapturedClipboardAction(
    Shared<LocalBoxFuture<'static, Result<Option<Arc<ClipboardItem>>, ClipboardReadError>>>,
);

impl CapturedClipboardAction {
    pub(super) fn is_ready(&self) -> bool {
        self.0.clone().now_or_never().is_some()
    }

    pub(super) async fn wait(&self) -> Result<()> {
        self.0.clone().await?;
        Ok(())
    }

    pub(super) fn check_ready(&self) -> Result<()> {
        self.wait()
            .now_or_never()
            .context("Clipboard read is not ready")?
    }

    fn new(
        read: impl Future<Output = Result<Option<ClipboardItem>, ClipboardReadError>> + 'static,
    ) -> Self {
        Self(
            read.map(|result| result.map(|item| item.map(Arc::new)))
                .boxed_local()
                .shared(),
        )
    }

    fn read(&self) -> Result<Option<ClipboardItem>> {
        Ok(self
            .0
            .clone()
            .now_or_never()
            .context("Clipboard read is not ready")??
            .as_deref()
            .cloned())
    }
}

impl Editor {
    pub fn read_from_clipboard(&self, cx: &App) -> Result<Option<ClipboardItem>> {
        if let Some(captured) =
            CapturedInput::current(cx).and_then(|input| input.clipboard.as_ref())
        {
            captured.read()
        } else {
            Ok(cx.read_from_clipboard())
        }
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub fn read_from_primary(&self, cx: &App) -> Result<Option<ClipboardItem>> {
        if let Some(captured) = CapturedInput::current(cx).and_then(|input| input.primary.as_ref())
        {
            captured.read()
        } else {
            Ok(cx.read_from_primary())
        }
    }

    pub fn do_paste(
        &mut self,
        text: &String,
        clipboard_selections: Option<Vec<ClipboardSelection>>,
        handle_entire_lines: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.input_is_deferred(cx) {
            let text = text.clone();
            self.defer_input(
                move |editor, window, cx| {
                    editor.do_paste(&text, clipboard_selections, handle_entire_lines, window, cx);
                },
                window,
                cx,
            );
            return;
        }
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
                                        is_standalone_url(to_insert),
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
                let clipboard_is_url = is_standalone_url(&clipboard_text);

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
                            edit_for_markdown_paste(
                                &snapshot,
                                range,
                                text_for_cursor,
                                clipboard_is_url,
                            )
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

    pub fn paste(&mut self, action: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if self.defer_clipboard_action_with_access(
            ClipboardAccess::Read,
            action,
            Self::paste,
            window,
            cx,
        ) {
            return;
        }
        match self.read_from_clipboard(cx) {
            Ok(Some(item)) => self.paste_item(&item, window, cx),
            Ok(None) => {}
            Err(error) => self.notify_clipboard_error(error, cx),
        }
    }

    pub fn paste_item(
        &mut self,
        item: &ClipboardItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.input_is_deferred(cx) {
            let item = item.clone();
            self.defer_input(
                move |editor, window, cx| editor.paste_item(&item, window, cx),
                window,
                cx,
            );
            return;
        }
        if self.read_only(cx) {
            return;
        }

        let clipboard_image = item.entries().iter().find_map(|entry| match entry {
            ClipboardEntry::Image(image) if !image.bytes.is_empty() => Some(image),
            _ => None,
        });

        if let Some(image) = clipboard_image {
            let is_markdown = {
                let display_map = self.display_snapshot(cx);
                let selections = self.selections.all::<MultiBufferOffset>(&display_map);
                selections
                    .first()
                    .and_then(|s| display_map.buffer_snapshot().language_at(s.head()))
                    .map(|lang| lang.name() == "Markdown")
                    .unwrap_or(false)
            };

            if is_markdown {
                let handled = maybe!({
                    let buffer = self.buffer().read(cx).as_singleton()?;
                    let file = buffer.read(cx).file()?;
                    let worktree_id = file.worktree_id(cx);
                    let dir_rel_path = file.path().parent()?.into_arc();
                    let worktree = self
                        .project
                        .as_ref()?
                        .read(cx)
                        .worktree_for_id(worktree_id, cx)?;

                    let extension = image.format.extension();
                    let snapshot = worktree.read(cx).snapshot();
                    let (filename, file_path) =
                        unused_image_path(&dir_rel_path, extension, |path| {
                            snapshot.entry_for_path(path).is_some()
                        })?;

                    let create_task = worktree.update(cx, |worktree, cx| {
                        worktree.create_entry(file_path, false, Some(image.bytes.clone()), cx)
                    });

                    cx.spawn_in(window, async move |editor, cx| {
                        create_task.await?;
                        editor.update_in(cx, |editor, window, cx| {
                            editor.insert_image_snippet(&filename, window, cx)
                        })
                    })
                    .detach_and_log_err(cx);

                    Some(())
                });
                if handled.is_some() {
                    // stop clipboard handling when the snippet is inserted
                    return;
                }
            }
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

    fn insert_image_snippet(
        &mut self,
        filename: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.input_is_deferred(cx) {
            let filename = filename.to_owned();
            self.defer_input(
                move |editor, window, cx| editor.insert_image_snippet(&filename, window, cx),
                window,
                cx,
            );
            return;
        }
        let Some(snippet) = Snippet::parse(&format!("![$1]({filename})$0")).log_err() else {
            return;
        };
        let display_map = self.display_snapshot(cx);
        let insertion_ranges: Vec<Range<MultiBufferOffset>> = self
            .selections
            .all::<MultiBufferOffset>(&display_map)
            .into_iter()
            .map(|selection| selection.start..selection.end)
            .collect();
        self.insert_snippet(&insertion_ranges, snippet, window, cx)
            .log_err();
    }

    pub(super) fn publish_clipboard_item(
        &mut self,
        item: ClipboardItem,
        window: &Window,
        cx: &mut Context<Self>,
    ) {
        self.bind_input_window(window);
        if self.shared_input_is_deferred() {
            self.defer_clipboard_publication(move |_, _, cx| cx.write_to_clipboard(item), cx);
        } else {
            cx.write_to_clipboard(item);
        }
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub(super) fn publish_primary_item(
        &mut self,
        item: ClipboardItem,
        window: &Window,
        cx: &mut Context<Self>,
    ) {
        self.bind_input_window(window);
        if self.shared_input_is_deferred() {
            self.defer_clipboard_publication(move |_, _, cx| cx.write_to_primary(item), cx);
        } else {
            cx.write_to_primary(item);
        }
    }

    pub(super) fn capture_clipboard(&self, cx: &App) -> CapturedClipboardAction {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(capture) = cx.try_global::<crate::test::NativeClipboardCapture>() {
            return CapturedClipboardAction::new((capture.0)(false));
        }
        CapturedClipboardAction::new(cx.read_from_clipboard_async())
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub(super) fn capture_primary(&self, cx: &App) -> CapturedClipboardAction {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(capture) = cx.try_global::<crate::test::NativeClipboardCapture>() {
            return CapturedClipboardAction::new((capture.0)(true));
        }
        CapturedClipboardAction::new(futures::future::ready(Ok(cx.read_from_primary())))
    }

    pub(super) fn prepare_cut(
        &self,
        cut_no_selection_line: bool,
        selections: &SelectionsCollection,
        snapshot: &DisplaySnapshot,
        cx: &App,
    ) -> (ClipboardItem, Vec<Selection<Point>>) {
        let mut text = String::new();
        let buffer = snapshot.buffer_snapshot();
        let line_mode = selections.line_mode();
        let mut selections = selections.all::<Point>(snapshot);
        let mut clipboard_selections = Vec::with_capacity(selections.len());
        {
            let max_point = buffer.max_point();
            let mut is_first = true;
            let mut prev_selection_was_entire_line = false;
            for selection in &mut selections {
                let is_entire_line = (selection.is_empty() && cut_no_selection_line) || line_mode;
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
                    buffer,
                    self.project.as_ref(),
                    cx,
                ));
            }
        }

        (
            ClipboardItem::new_string_with_json_metadata(text, clipboard_selections),
            selections,
        )
    }

    pub(super) fn delete_cut_ranges(
        &mut self,
        selections: Vec<Selection<Point>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.transact(window, cx, |editor, window, cx| {
            editor.change_selections(
                SelectionEffects::default(),
                window,
                cx,
                |selections_to_change| {
                    selections_to_change.select(selections);
                },
            );
            editor.insert("", window, cx);
        });
    }

    pub(super) fn cut(&mut self, action: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if self.defer_clipboard_action(action, Self::cut, window, cx) {
            return;
        }
        if self.read_only(cx) {
            return;
        }
        let snapshot = self.display_snapshot(cx);
        let (item, selections) = self.prepare_cut(true, &self.selections, &snapshot, cx);
        self.publish_cut(item, selections, window, cx);
    }

    pub(super) fn publish_cut(
        &mut self,
        item: ClipboardItem,
        selections: Vec<Selection<Point>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.delete_cut_ranges(selections, window, cx);
        cx.write_to_clipboard(item);
    }

    pub(super) fn kill_ring_cut(
        &mut self,
        action: &KillRingCut,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.defer_action(action, Self::kill_ring_cut, window, cx) {
            return;
        }
        if self.read_only(cx) {
            return;
        }
        let selection_count = self.selections.count();
        let first_selection = self.selections.first_anchor();
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let first_selection_is_empty = first_selection.start == first_selection.end;
        let selection_start_point = first_selection.start.to_point(&snapshot);
        let selection_start_row = selection_start_point.row;
        let selection_start_column = selection_start_point.column;
        let Some((_, text_anchor)) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(first_selection.start, cx)
        else {
            return;
        };
        let buffer_id = text_anchor.buffer_id;

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
        let snapshot = self.display_snapshot(cx);
        let (item, selections) = self.prepare_cut(false, &self.selections, &snapshot, cx);
        self.delete_cut_ranges(selections, window, cx);

        let Some(item_text) = item.text() else {
            return;
        };

        let entry_metadata = item.entries().first().and_then(|entry| match entry {
            ClipboardEntry::String(entry) => entry.metadata_json::<Vec<ClipboardSelection>>(),
            _ => None,
        });

        let can_append = selection_count == 1 && first_selection_is_empty;
        let item = if can_append
            && let Some(previous_ring) = cx.try_global::<KillRing>()
            && previous_ring.can_append
            && previous_ring.buffer_id == buffer_id
            && previous_ring.row == selection_start_row
            && previous_ring.column == selection_start_column
        {
            let mut entries = previous_ring
                .metadata
                .as_ref()
                .map_or_else(Vec::new, Clone::clone);
            if let Some(metadata) = entry_metadata.as_ref() {
                entries.extend_from_slice(metadata);
            }

            let mut text = previous_ring.text.clone();
            text.push_str(&item_text);
            let text_len = text.len();

            KillRing {
                text,
                metadata: kill_ring_metadata_for_text(entries, text_len),
                row: previous_ring.row,
                column: previous_ring.column,
                buffer_id: previous_ring.buffer_id,
                can_append,
            }
        } else {
            KillRing {
                text: item_text,
                metadata: entry_metadata,
                row: selection_start_row,
                column: selection_start_column,
                buffer_id,
                can_append,
            }
        };

        cx.set_global(item)
    }

    pub(super) fn kill_ring_yank(
        &mut self,
        action: &KillRingYank,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.defer_action(action, Self::kill_ring_yank, window, cx) {
            return;
        }
        if !cx.has_global::<KillRing>() {
            return;
        }

        let (text, metadata) = cx.update_global::<KillRing, _>(|kill_ring, _| {
            kill_ring.can_append = false;
            (kill_ring.text.clone(), kill_ring.metadata.clone())
        });

        self.do_paste(&text, metadata, false, window, cx);
    }

    pub(super) fn copy_and_trim(
        &mut self,
        action: &CopyAndTrim,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.defer_clipboard_action(action, Self::copy_and_trim, window, cx) {
            return;
        }
        self.do_copy(true, cx);
    }

    pub(super) fn copy(&mut self, action: &Copy, window: &mut Window, cx: &mut Context<Self>) {
        if self.defer_clipboard_action(action, Self::copy, window, cx) {
            return;
        }
        self.do_copy(false, cx);
    }

    pub(super) fn diff_clipboard_with_selection(
        &mut self,
        action: &DiffClipboardWithSelection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.defer_clipboard_action_with_access(
            ClipboardAccess::Read,
            action,
            Self::diff_clipboard_with_selection,
            window,
            cx,
        ) {
            return;
        }
        let selections = self
            .selections
            .all::<MultiBufferOffset>(&self.display_snapshot(cx));

        if selections.is_empty() {
            log::warn!("There should always be at least one selection in Zed. This is a bug.");
            return;
        };

        let item = match self.read_from_clipboard(cx) {
            Ok(item) => item,
            Err(error) => {
                self.notify_clipboard_error(error, cx);
                return;
            }
        };
        let clipboard_text = item.and_then(|item| {
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

    fn do_copy(&mut self, strip_leading_indents: bool, cx: &mut Context<Self>) {
        let selections = self.selections.all::<Point>(&self.display_snapshot(cx));
        let buffer = self.buffer.read(cx).read(cx);
        let mut text = String::new();
        let mut clipboard_selections = Vec::with_capacity(selections.len());

        let max_point = buffer.max_point();
        let mut is_first = true;
        let mut prev_selection_was_entire_line = false;
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
            prev_selection_was_entire_line = is_entire_line && !is_multiline_trim;

            clipboard_selections.push(ClipboardSelection::for_buffer(
                selection_len,
                is_entire_line,
                start..end,
                &buffer,
                self.project.as_ref(),
                cx,
            ));
        }

        drop(buffer);
        cx.write_to_clipboard(ClipboardItem::new_string_with_json_metadata(
            text,
            clipboard_selections,
        ));
    }
}

struct KillRing {
    text: String,
    metadata: Option<Vec<ClipboardSelection>>,
    row: u32,
    column: u32,
    buffer_id: BufferId,
    can_append: bool,
}
impl Global for KillRing {}

fn kill_ring_metadata_for_text(
    mut metadata: Vec<ClipboardSelection>,
    text_len: usize,
) -> Option<Vec<ClipboardSelection>> {
    match metadata.len() {
        0 => None,
        1 => Some(metadata),
        _ => {
            let first_selection = metadata.remove(0);
            Some(vec![ClipboardSelection {
                len: text_len,
                is_entire_line: false,
                first_line_indent: first_selection.first_line_indent,
                file_path: None,
                line_range: None,
            }])
        }
    }
}

fn edit_for_markdown_paste<'a>(
    buffer: &MultiBufferSnapshot,
    range: Range<MultiBufferOffset>,
    to_insert: &'a str,
    to_insert_is_url: bool,
) -> (Range<MultiBufferOffset>, Cow<'a, str>) {
    if !to_insert_is_url {
        return (range, Cow::Borrowed(to_insert));
    };

    let old_text = buffer.text_for_range(range.clone()).collect::<String>();

    let new_text = if range.is_empty() || is_standalone_url(&old_text) {
        Cow::Borrowed(to_insert)
    } else {
        Cow::Owned(format!("[{old_text}]({to_insert})"))
    };
    (range, new_text)
}

/// Returns a filename of the form `image.{extension}` (or `image_{N}.{extension}`
/// if taken) that does not collide with an existing entry in `dir_rel_path`,
/// along with the full path of the candidate file.
fn unused_image_path(
    dir_rel_path: &RelPath,
    extension: &str,
    exists: impl Fn(&RelPath) -> bool,
) -> Option<(String, Arc<RelPath>)> {
    let mut filename = format!("image.{extension}");
    let mut counter = 1u32;
    loop {
        let candidate = dir_rel_path.join(RelPath::from_unix_str(&filename).ok()?);
        if !exists(&candidate) {
            return Some((filename, candidate.into()));
        }
        filename = format!("image_{counter}.{extension}");
        counter += 1;
    }
}

/// Whether `text` consists solely of a single URL, as opposed to merely
/// starting with a scheme-like prefix (e.g. a commit message like
/// `editor: Fix ...`, which `url::Url::parse` would accept).
fn is_standalone_url(text: &str) -> bool {
    let mut finder = linkify::LinkFinder::new();
    finder.kinds(&[linkify::LinkKind::Url]);
    finder
        .links(text)
        .next()
        .is_some_and(|link| link.start() == 0 && link.end() == text.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        editor_tests::init_test,
        test::{
            NativeClipboardCapture, editor_test_context::EditorTestContext, pending_clipboard_read,
        },
    };
    use fs::Fs as _;
    use futures::channel::oneshot;
    use gpui::{KeyBinding, Keystroke, TestAppContext};
    use language::language_settings::SoftWrap;
    use project::FakeFs;
    use std::cell::Cell;
    use util::{path, rel_path::rel_path};
    use workspace::{OpenOptions, SaveIntent, item::Item};

    #[gpui::test]
    async fn test_clipboard_read_failure_and_cancellation(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for outcome in ["success", "failure", "producer", "window"] {
            let mut editor_cx = EditorTestContext::new(cx).await;
            let (read, resolver) = pending_clipboard_read();
            let captures = Rc::new(Cell::new(0));
            let completion = editor_cx.update_editor(|editor, window, cx| {
                cx.set_global(NativeClipboardCapture(Rc::new({
                    let captures = captures.clone();
                    move |primary| {
                        assert!(!primary);
                        captures.set(captures.get() + 1);
                        read.clone()
                    }
                })));
                editor.paste(&Paste, window, cx);
                editor.replace_text_in_range(None, "q", window, cx);
                editor.input_completion(cx)
            });
            cx.run_until_parked();
            assert_eq!(captures.get(), 1);
            editor_cx.assert_editor_state("ˇ");
            editor_cx.update(|_, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(String::from("later")));
            });
            match outcome {
                "success" => resolver
                    .send(Ok(Some(ClipboardItem::new_string(String::from("started")))))
                    .expect("pending read"),
                "failure" => resolver
                    .send(Err(ClipboardReadError::Unavailable))
                    .expect("pending read"),
                "producer" => drop(resolver),
                "window" => {
                    editor_cx.update(|window, cx| {
                        cx.remove_global::<NativeClipboardCapture>();
                        window.remove_window();
                    });
                    editor_cx.run_until_parked();
                    assert!(resolver.is_canceled());
                }
                _ => unreachable!(),
            }
            assert_eq!(completion.await.is_err(), outcome != "success");
            assert_eq!(captures.get(), 1);
            assert_eq!(
                editor_cx.editor.read_with(cx, |editor, cx| editor.text(cx)),
                match outcome {
                    "success" => "startedq",
                    "window" => "",
                    _ => "q",
                },
            );
        }
    }

    #[gpui::test]
    async fn test_clipboard_geometry_retry_preserves_metadata(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut editor_cx = EditorTestContext::new(cx).await;
        let original = ClipboardItem::new_string_with_json_metadata(
            String::from("same text"),
            serde_json::json!({"source": "original"}),
        );
        let replacement = ClipboardItem::new_string_with_json_metadata(
            String::from("same text"),
            serde_json::json!({"source": "replacement"}),
        );
        let completion = editor_cx.update_editor(|editor, window, cx| {
            cx.write_to_clipboard(original.clone());
            let scope = editor
                .defer_shared_input(
                    NativeInput {
                        clipboard: Some(ClipboardAccess::Read),
                        ..NativeInput::default()
                    },
                    |_, _, _| panic!("native read must be synchronous"),
                    window,
                    cx,
                )
                .expect("native read")
                .expect("ready native read");
            scope.run(cx, |cx| {
                assert_eq!(
                    editor.read_from_clipboard(cx).expect("original read"),
                    Some(original.clone())
                );
                editor.set_placeholder_text(&"x".repeat(2_000), window, cx);
                let pending = editor
                    .placeholder_display_map
                    .as_ref()
                    .expect("placeholder")
                    .update(cx, |map, cx| {
                        map.snapshot(cx)
                            .long_row(DisplayRow(0))
                            .err()
                            .expect("cold geometry")
                    });
                editor.defer_geometry(
                    pending,
                    move |editor, _, cx| {
                        assert_eq!(
                            editor.read_from_clipboard(cx).expect("retried read"),
                            Some(original)
                        );
                    },
                    window,
                    cx,
                );
            });
            cx.write_to_clipboard(replacement.clone());
            editor.input_completion(cx)
        });
        completion.await.expect("geometry retry");
        editor_cx.update_editor(|editor, window, cx| {
            let scope = editor
                .defer_shared_input(
                    NativeInput {
                        clipboard: Some(ClipboardAccess::Read),
                        ..NativeInput::default()
                    },
                    |_, _, _| panic!("next command must not wait"),
                    window,
                    cx,
                )
                .expect("next command read")
                .expect("ready next command");
            scope.run(cx, |cx| {
                assert_eq!(
                    editor.read_from_clipboard(cx).expect("fresh read"),
                    Some(replacement)
                );
            });
        });
    }

    #[gpui::test]
    async fn test_clipboard_optional_read_failure_preserves_input(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for access in [ClipboardAccess::Read, ClipboardAccess::Observe] {
            let mut editor_cx = EditorTestContext::new(cx).await;
            let completion = editor_cx.update_editor(|editor, window, cx| {
                cx.set_global(NativeClipboardCapture(Rc::new(|_| {
                    futures::future::ready(Err(ClipboardReadError::Unavailable))
                        .boxed_local()
                        .shared()
                })));
                editor.schedule_input_continuation(window, cx, |_, _| {});
                let handle = cx.weak_entity();
                assert!(
                    editor
                        .defer_shared_input(
                            NativeInput {
                                clipboard: Some(access),
                                ..NativeInput::default()
                            },
                            move |result, window, cx| {
                                assert_eq!(result.is_err(), access == ClipboardAccess::Read);
                                if result.is_ok() {
                                    handle
                                        .update(cx, |editor, cx| {
                                            editor.handle_input("a", window, cx)
                                        })
                                        .expect("editor");
                                }
                            },
                            window,
                            cx,
                        )
                        .expect("queued read")
                        .is_none()
                );
                editor.handle_input("b", window, cx);
                editor.input_completion(cx)
            });
            assert_eq!(completion.await.is_err(), access == ClipboardAccess::Read);
            editor_cx.update_editor(|editor, window, cx| editor.handle_input("c", window, cx));
            editor_cx.assert_editor_state(if access == ClipboardAccess::Read {
                "bcˇ"
            } else {
                "abcˇ"
            });
        }
    }

    #[gpui::test]
    async fn test_clipboard_write_does_not_read_payload(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut editor_cx = EditorTestContext::new(cx).await;
        editor_cx.set_state("«abcˇ»");
        editor_cx.update_editor(|editor, window, cx| {
            cx.set_global(NativeClipboardCapture(Rc::new(|_| {
                panic!("write must not read the clipboard")
            })));
            editor.cut(&Cut, window, cx);
            assert_eq!(editor.text(cx), "");
            assert_eq!(
                cx.read_from_clipboard().and_then(|item| item.text()),
                Some(String::from("abc"))
            );
        });
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    #[gpui::test]
    async fn test_shared_input_reads_only_requested_native_channel(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for primary in [false, true] {
            let mut editor_cx = EditorTestContext::new(cx).await;
            let captures = Rc::new(RefCell::new(Vec::new()));
            let completion = editor_cx.update_editor(|editor, window, cx| {
                cx.set_global(NativeClipboardCapture(Rc::new({
                    let captures = captures.clone();
                    move |channel| {
                        captures.borrow_mut().push(channel);
                        futures::future::ready(Ok(Some(ClipboardItem::new_string(String::from(
                            "required",
                        )))))
                        .boxed_local()
                        .shared()
                    }
                })));
                editor.schedule_input_continuation(window, cx, |_, _| {});
                let handle = cx.weak_entity();
                assert!(
                    editor
                        .defer_shared_input(
                            NativeInput {
                                clipboard: (!primary).then_some(ClipboardAccess::Read),
                                primary: primary.then_some(ClipboardAccess::Read),
                            },
                            move |result, _, cx| {
                                result.expect("requested channel");
                                handle
                                    .read_with(cx, |editor, cx| {
                                        let item = if primary {
                                            editor.read_from_primary(cx)
                                        } else {
                                            editor.read_from_clipboard(cx)
                                        };
                                        assert_eq!(
                                            item.expect("clipboard").and_then(|item| item.text()),
                                            Some(String::from("required"))
                                        );
                                    })
                                    .expect("editor");
                            },
                            window,
                            cx,
                        )
                        .expect("queued read")
                        .is_none()
                );
                assert_eq!(*captures.borrow(), Vec::<bool>::new());
                editor.input_completion(cx)
            });
            completion.await.expect("requested channel completed");
            assert_eq!(*captures.borrow(), [primary]);
        }
    }

    #[gpui::test]
    async fn test_deferred_clipboard_fallback_reads_at_execution(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for already_pending in [false, true] {
            let mut cx = EditorTestContext::new(cx).await;
            cx.update(|window, cx| {
                cx.bind_keys([
                    KeyBinding::new("ctrl-v", Paste, Some("Editor")),
                    KeyBinding::new("ctrl-v", MoveUp, Some("Editor")),
                ]);
                window.draw(cx).clear(cx);
            });
            cx.background_executor.set_block_on_ticks(0..=0);
            let editor = cx.editor.clone();
            let text = "x".repeat(2_000);
            let completion = cx.update(|window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.set_soft_wrap_mode(SoftWrap::None, cx);
                    editor.set_text(text.clone(), window, cx);
                    editor.change_selections(
                        SelectionEffects::no_scroll(),
                        window,
                        cx,
                        |selections| {
                            selections.select_ranges([Point::zero()..Point::zero()]);
                        },
                    );
                    assert!(!editor.input_is_deferred(cx));
                    if already_pending {
                        editor.move_up(&MoveUp, window, cx);
                        assert!(editor.input_is_deferred(cx));
                    }
                });
                cx.write_to_clipboard(ClipboardItem::new_string(String::from("A")));
                window.dispatch_keystroke(Keystroke::parse("ctrl-v").expect("fallback key"), cx);
                cx.write_to_clipboard(ClipboardItem::new_string(String::from("B")));
                editor.update(cx, |editor, cx| {
                    assert!(editor.input_is_deferred(cx));
                    assert_eq!(editor.text(cx), text);
                    editor.input_completion(cx)
                })
            });
            cx.background_executor.set_block_on_ticks(0..=50);
            completion.await.expect("clipboard fallback completed");
            cx.assert_editor_state(&format!("Bˇ{text}"));
        }
    }

    #[gpui::test]
    async fn test_deferred_clipboard_reads_each_paste_at_execution(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for ingress in [
            PasteIngress::Direct,
            PasteIngress::Action,
            PasteIngress::RegisteredAction,
            PasteIngress::RegisteredKeyboard,
            PasteIngress::DeferredKeyboard,
        ] {
            let (mut cx, _subscription) = clipboard_test_context(ingress, cx).await;
            let editor = cx.editor.clone();
            let completion = cx.update(|window, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(String::from("A")));
                start_deferred_clipboard_input(&editor, window, cx);
                dispatch_paste(ingress, &editor, window, cx);
                cx.write_to_clipboard(ClipboardItem::new_string(String::from("B")));
                dispatch_paste(ingress, &editor, window, cx);
                editor.update(cx, |editor, cx| editor.input_completion(cx))
            });
            completion.await.expect("clipboard input completed");
            cx.assert_editor_state(&format!("ab\nxBBˇ{}\ncd", "x".repeat(1_999)));
        }
    }

    #[gpui::test]
    async fn test_deferred_clipboard_reads_after_empty_admission(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let (mut cx, _subscription) =
            clipboard_test_context(PasteIngress::RegisteredKeyboard, cx).await;
        let editor = cx.editor.clone();
        let completion = cx.update(|window, cx| {
            start_deferred_clipboard_input(&editor, window, cx);
            assert_eq!(cx.read_from_clipboard(), None);
            dispatch_paste(PasteIngress::RegisteredKeyboard, &editor, window, cx);
            cx.write_to_clipboard(ClipboardItem::new_string(String::from("late")));
            editor.update(cx, |editor, cx| editor.input_completion(cx))
        });
        completion.await.expect("clipboard input completed");
        cx.assert_editor_state(&format!("ab\nxlateˇ{}\ncd", "x".repeat(1_999)));
    }

    #[gpui::test]
    async fn test_deferred_clipboard_capture_after_replayed_write(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let (mut cx, _subscription) = clipboard_test_context(PasteIngress::Direct, cx).await;
        let editor = cx.editor.clone();
        let completion = cx.update(|window, cx| {
            start_deferred_clipboard_input(&editor, window, cx);
            editor.update(cx, |editor, cx| {
                editor.cut(&Cut, window, cx);
                assert!(editor.defer_input(
                    |editor, window, cx| {
                        editor.buffer.update(cx, |buffer, cx| {
                            let end = buffer.len(cx);
                            buffer.edit([(end..end, format!("\n{}", "y".repeat(2_000)))], None, cx);
                        });
                        editor.move_down(&MoveDown, window, cx);
                        assert!(editor.input_is_deferred(cx));
                        cx.write_to_clipboard(ClipboardItem::new_string(String::from("A")));
                        editor.paste(&Paste, window, cx);
                        cx.write_to_clipboard(ClipboardItem::new_string(String::from("B")));
                    },
                    window,
                    cx,
                ));
                editor.input_completion(cx)
            })
        });
        completion.await.expect("nested clipboard input completed");
        cx.assert_editor_state(&format!("ab\ncd\nBˇ{}", "y".repeat(2_000)));
    }

    #[gpui::test]
    async fn test_cut_to_end_of_line_with_pending_selection(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for (add, line_mode) in [(false, false), (true, false), (false, true), (true, true)] {
            let mut cx = EditorTestContext::new(cx).await;
            cx.set_state("ˇabc\ndef");
            cx.update_editor(|editor, window, cx| {
                editor.begin_selection(DisplayPoint::new(DisplayRow(1), 2), add, 1, window, cx);
                editor.update_selection(
                    DisplayPoint::new(DisplayRow(1), 1),
                    1,
                    gpui::Point::default(),
                    window,
                    cx,
                );
                editor.selections.set_line_mode(line_mode);
                editor.cut_to_end_of_line(&CutToEndOfLine::default(), window, cx);

                let (remaining_text, clipboard_text) = match (add, line_mode) {
                    (false, false) => ("abc\nd", "ef"),
                    (true, false) => ("\nd", "abc\nef"),
                    (false, true) => ("abc\n", "def"),
                    (true, true) => ("", "abc\ndef"),
                };
                assert_eq!(editor.text(cx), remaining_text);
                assert_eq!(editor.selections.pending_anchor(), None);
                assert_eq!(
                    cx.read_from_clipboard().and_then(|item| item.text()),
                    Some(String::from(clipboard_text))
                );
            });
        }
    }

    #[gpui::test]
    async fn test_cross_editor_clipboard_publication_order(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut results = Vec::new();
        for cold in [None, Some(false), Some(true)] {
            let mut source = EditorTestContext::new(cx).await;
            let mut target = EditorTestContext::new(cx).await;
            source.set_state("«alphaˇ»");
            target.set_state("Bˇ");
            let source_completion = source.update_editor(|editor, window, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(String::from("Z")));
                if let Some(first_cold) = cold {
                    editor.set_placeholder_text(&"x".repeat(2_000), window, cx);
                    let pending = editor
                        .placeholder_display_map
                        .as_ref()
                        .expect("placeholder display map")
                        .update(cx, |map, cx| {
                            map.snapshot(cx)
                                .long_row(DisplayRow(0))
                                .err()
                                .expect("cold geometry")
                        });
                    editor.defer_geometry(
                        pending,
                        move |editor, window, cx| {
                            if first_cold {
                                editor.copy(&Copy, window, cx);
                            }
                        },
                        window,
                        cx,
                    );
                    if !first_cold {
                        editor.copy(&Copy, window, cx);
                    }
                } else {
                    editor.copy(&Copy, window, cx);
                }
                editor.input_completion(cx)
            });
            let target_completion = target.update_editor(|editor, window, cx| {
                editor.paste(&Paste, window, cx);
                editor.handle_input("!", window, cx);
                editor.input_completion(cx)
            });
            source_completion.await.expect("source input completed");
            target_completion.await.expect("target input completed");
            results.push(target.editor(|editor, _, cx| editor.text(cx)));
        }
        assert_eq!(results, ["Balpha!", "Balpha!", "Balpha!"]);
    }

    #[gpui::test]
    async fn test_persistence_waits_for_pending_clipboard_input(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for operation in [
            PersistenceOperation::Save,
            PersistenceOperation::SaveAs,
            PersistenceOperation::Reload,
        ] {
            for failed in [false, true] {
                for shared in [false, true] {
                    let fs = FakeFs::new(cx.executor());
                    fs.insert_tree(path!("/root"), serde_json::json!({ "file": "abc" }))
                        .await;
                    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
                    let buffer = project
                        .update(cx, |project, cx| {
                            project.open_local_buffer(path!("/root/file"), cx)
                        })
                        .await
                        .expect("saved buffer");
                    let mut copy_path = buffer.read_with(cx, |buffer, cx| {
                        buffer.project_path(cx).expect("saved project path")
                    });
                    copy_path.path = rel_path("copy").into();
                    let editor = cx.add_window(|window, cx| {
                        Editor::for_buffer(buffer, Some(project.clone()), window, cx)
                    });
                    let mut editor_cx = EditorTestContext::for_editor(editor, cx).await;
                    editor_cx.set_selections_state("abcˇ");
                    assert!(!editor_cx.editor(|editor, _, cx| Item::is_dirty(editor, cx)));
                    let target = if shared {
                        editor_cx.update_editor(|editor, window, cx| {
                            cx.new(|cx| editor.clone(window, cx))
                        })
                    } else {
                        editor_cx.editor.clone()
                    };
                    let (resolver, completion) = queue_pending_clipboard_input(&mut editor_cx);
                    let mut persistence =
                        target.update_in(&mut editor_cx.cx, |editor, window, cx| match operation {
                            PersistenceOperation::Save => Item::save(
                                editor,
                                SaveOptions {
                                    format: false,
                                    force_format: false,
                                    autosave: false,
                                },
                                project.clone(),
                                window,
                                cx,
                            ),
                            PersistenceOperation::SaveAs => {
                                Item::save_as(editor, project.clone(), copy_path, window, cx)
                            }
                            PersistenceOperation::Reload => {
                                Item::reload(editor, project.clone(), window, cx)
                            }
                        });
                    cx.run_until_parked();
                    assert!(
                        (&mut persistence).now_or_never().is_none(),
                        "{operation:?}, failed={failed}, shared={shared} completed before input"
                    );
                    editor_cx.assert_editor_state("abcˇ");
                    resolver
                        .send(if failed {
                            Err(ClipboardReadError::Unavailable)
                        } else {
                            Ok(Some(ClipboardItem::new_string(String::from("P"))))
                        })
                        .ok();
                    assert_eq!(completion.await.is_err(), failed);
                    assert_eq!(
                        persistence.await.is_err(),
                        failed,
                        "{operation:?}, failed={failed}, shared={shared}"
                    );
                    assert_eq!(
                        editor_cx.buffer_text(),
                        if failed {
                            "abcq"
                        } else if operation == PersistenceOperation::Reload {
                            "abc"
                        } else {
                            "abcPq"
                        }
                    );
                    assert_eq!(
                        fs.load(path!("/root/file").as_ref())
                            .await
                            .expect("saved file"),
                        if !failed && operation == PersistenceOperation::Save {
                            "abcPq"
                        } else {
                            "abc"
                        }
                    );
                    if operation == PersistenceOperation::SaveAs {
                        if failed {
                            assert!(!fs.is_file(path!("/root/copy").as_ref()).await);
                        } else {
                            assert_eq!(
                                fs.load(path!("/root/copy").as_ref())
                                    .await
                                    .expect("saved copy"),
                                "abcPq"
                            );
                        }
                    }
                    editor_cx.update(|window, _| window.remove_window());
                }
            }
        }
    }

    #[gpui::test]
    async fn test_pane_close_waits_for_pending_clipboard_input(cx: &mut TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.ensure_final_newline_on_save = Some(false);
        });
        for (failed, shared, save_before_poll) in [
            (false, false, false),
            (false, true, false),
            (true, false, false),
            (true, true, false),
            (true, false, true),
        ] {
            let fs = FakeFs::new(cx.executor());
            fs.insert_tree(path!("/root"), serde_json::json!({ "file": "abc" }))
                .await;
            let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
            let (workspace, workspace_cx) =
                cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
            let editor = workspace
                .update_in(workspace_cx, |workspace, window, cx| {
                    workspace.open_abs_path(
                        PathBuf::from(path!("/root/file")),
                        OpenOptions::default(),
                        window,
                        cx,
                    )
                })
                .await
                .expect("open saved file")
                .downcast::<Editor>()
                .expect("workspace editor");
            let pane =
                workspace.read_with(workspace_cx, |workspace, _| workspace.active_pane().clone());
            let mut editor_cx = EditorTestContext::for_editor_in(editor, workspace_cx).await;
            editor_cx.set_selections_state("abcˇ");
            assert!(!editor_cx.editor(|editor, _, cx| Item::is_dirty(editor, cx)));
            let sibling = shared.then(|| {
                let editor = editor_cx
                    .update_editor(|editor, window, cx| cx.new(|cx| editor.clone(window, cx)));
                let pane = workspace.update_in(workspace_cx, |workspace, window, cx| {
                    workspace.split_pane(pane.clone(), SplitDirection::Right, window, cx)
                });
                pane.update_in(workspace_cx, |pane, window, cx| {
                    pane.add_item(Box::new(editor.clone()), true, true, None, window, cx);
                });
                (pane, editor)
            });
            let weak_editor = editor_cx.editor.downgrade();
            let item_id = editor_cx.editor.entity_id();
            let (resolver, completion) = queue_pending_clipboard_input(&mut editor_cx);
            if save_before_poll {
                let (sender, receiver) = oneshot::channel();
                editor_cx.update_editor(|editor, window, cx| {
                    let workspace = workspace.clone();
                    editor.schedule_input_continuation(window, cx, move |window, cx| {
                        let save = workspace.update(cx, |workspace, cx| {
                            workspace.save_active_item(SaveIntent::SaveWithoutFormat, window, cx)
                        });
                        assert!(sender.send(save).is_ok());
                    });
                });
                resolver.send(Err(ClipboardReadError::Unavailable)).ok();
                let input_error = completion.await.expect_err("failed capture");
                let save = receiver
                    .await
                    .expect("workspace save requested during input");
                assert_eq!(
                    save.await
                        .expect_err("workspace save must retain the preceding input failure")
                        .root_cause()
                        .to_string(),
                    input_error.root_cause().to_string()
                );
                editor_cx.assert_editor_state("abcqˇ");
                assert_eq!(
                    fs.load(path!("/root/file").as_ref())
                        .await
                        .expect("unsaved file"),
                    "abc"
                );
                workspace_cx.update(|window, _| window.remove_window());
                continue;
            }
            drop(editor_cx);
            let mut close = pane.update_in(workspace_cx, |pane, window, cx| {
                pane.close_item_by_id(item_id, SaveIntent::Close, window, cx)
            });
            workspace_cx.run_until_parked();
            workspace_cx.update(|window, cx| window.draw(cx).clear(cx));
            workspace_cx.run_until_parked();
            assert!(
                (&mut close).now_or_never().is_none(),
                "failed={failed}, shared={shared}: close completed before input"
            );
            assert_eq!(pane.read_with(workspace_cx, |pane, _| pane.items_len()), 1);
            assert!(weak_editor.upgrade().is_some());
            assert!(!workspace_cx.has_pending_prompt());
            resolver
                .send(if failed {
                    Err(ClipboardReadError::Unavailable)
                } else {
                    Ok(Some(ClipboardItem::new_string(String::from("P"))))
                })
                .ok();
            assert_eq!(completion.await.is_err(), failed);
            workspace_cx.run_until_parked();
            if failed {
                close.await.expect_err("failed capture must abort close");
                assert_eq!(pane.read_with(workspace_cx, |pane, _| pane.items_len()), 1);
                assert_eq!(
                    weak_editor
                        .read_with(workspace_cx, |editor, cx| editor.text(cx))
                        .expect("editor retained after failed close"),
                    "abcq"
                );
            } else {
                assert_eq!(workspace_cx.has_pending_prompt(), !shared);
                if !shared {
                    workspace_cx.simulate_prompt_answer("Save");
                }
                close.await.expect("close after completed input");
                assert_eq!(pane.read_with(workspace_cx, |pane, _| pane.items_len()), 0);
                workspace_cx.update(|window, cx| window.draw(cx).clear(cx));
                workspace_cx
                    .executor()
                    .advance_clock(workspace::SERIALIZATION_THROTTLE_TIME);
                workspace_cx.run_until_parked();
                assert!(
                    weak_editor.upgrade().is_none(),
                    "failed={failed}, shared={shared}: closed editor retained after serialization"
                );
                if let Some((pane, editor)) = sibling {
                    assert_eq!(
                        editor.read_with(workspace_cx, |editor, cx| editor.text(cx)),
                        "abcPq"
                    );
                    assert_eq!(
                        fs.load(path!("/root/file").as_ref())
                            .await
                            .expect("unsaved file"),
                        "abc"
                    );
                    let close = pane.update_in(workspace_cx, |pane, window, cx| {
                        pane.close_item_by_id(editor.entity_id(), SaveIntent::Close, window, cx)
                    });
                    workspace_cx.run_until_parked();
                    assert!(workspace_cx.has_pending_prompt());
                    workspace_cx.simulate_prompt_answer("Save");
                    close.await.expect("close last shared editor");
                }
            }
            assert_eq!(
                fs.load(path!("/root/file").as_ref())
                    .await
                    .expect("file after close"),
                if failed { "abc" } else { "abcPq" }
            );
            workspace_cx.update(|window, _| window.remove_window());
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum PersistenceOperation {
        Save,
        SaveAs,
        Reload,
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum PasteIngress {
        Direct,
        Action,
        RegisteredAction,
        RegisteredKeyboard,
        DeferredKeyboard,
    }

    async fn clipboard_test_context(
        ingress: PasteIngress,
        cx: &mut TestAppContext,
    ) -> (EditorTestContext, Option<Subscription>) {
        let mut cx = EditorTestContext::new(cx).await;
        let subscription = if ingress == PasteIngress::RegisteredAction
            || ingress == PasteIngress::RegisteredKeyboard
        {
            Some(cx.update_editor(|editor, _, cx| {
                let handle = cx.weak_entity();
                let subscription = editor.register_action(move |action: &Paste, window, cx| {
                    handle
                        .update(cx, |editor, cx| editor.paste(action, window, cx))
                        .expect("registered editor is alive");
                });
                cx.notify();
                subscription
            }))
        } else {
            None
        };
        cx.update(|window, cx| {
            cx.bind_keys([KeyBinding::new("ctrl-v", Paste, Some("Editor"))]);
            window.draw(cx).clear(cx);
        });
        (cx, subscription)
    }

    fn start_deferred_clipboard_input(editor: &Entity<Editor>, window: &mut Window, cx: &mut App) {
        editor.update(cx, |editor, cx| {
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor.set_text(format!("ab\n{}\ncd", "x".repeat(2_000)), window, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
            });
            editor.move_down(&MoveDown, window, cx);
            assert!(editor.input_is_deferred(cx));
        });
    }

    fn dispatch_paste(
        ingress: PasteIngress,
        editor: &Entity<Editor>,
        window: &mut Window,
        cx: &mut App,
    ) {
        match ingress {
            PasteIngress::Direct => {
                editor.update(cx, |editor, cx| editor.paste(&Paste, window, cx));
            }
            PasteIngress::Action | PasteIngress::RegisteredAction => {
                editor
                    .read(cx)
                    .focus_handle
                    .clone()
                    .dispatch_action(&Paste, window, cx);
            }
            PasteIngress::RegisteredKeyboard => {
                window.dispatch_keystroke(Keystroke::parse("ctrl-v").expect("paste keystroke"), cx);
            }
            PasteIngress::DeferredKeyboard => {
                editor.update(cx, |editor, cx| {
                    let origin = cx.weak_entity();
                    assert!(editor.defer_keystroke_input(
                        |window, _| {
                            window.keystroke_replay_with_context(
                                Keystroke::parse("ctrl-v").expect("paste keystroke"),
                                gpui::InputPreference::KeyBindings,
                                move |window, cx| {
                                    origin
                                        .update(cx, |editor, cx| editor.key_context(window, cx))
                                        .ok()
                                },
                            )
                        },
                        window,
                        cx,
                    ));
                });
            }
        }
    }

    fn queue_pending_clipboard_input(
        editor_cx: &mut EditorTestContext,
    ) -> (
        oneshot::Sender<Result<Option<ClipboardItem>, ClipboardReadError>>,
        Task<Result<()>>,
    ) {
        let (read, resolver) = pending_clipboard_read();
        let completion = editor_cx.update_editor(|editor, window, cx| {
            cx.set_global(NativeClipboardCapture(Rc::new(move |_| read.clone())));
            editor.paste(&Paste, window, cx);
            editor.replace_text_in_range(None, "q", window, cx);
            assert!(editor.shared_input_is_deferred());
            editor.input_completion(cx)
        });
        (resolver, completion)
    }
}
