use super::*;
use crate::deferred_input::CapturedInput;
use gpui::{ClipboardReadMode, ClipboardRevision, ClipboardSnapshot};
use std::rc::Weak;
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

#[derive(Default)]
pub(super) struct ClipboardChannels {
    clipboard: RefCell<Weak<RefCell<ClipboardState>>>,
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    primary: RefCell<Weak<RefCell<ClipboardState>>>,
}

impl Global for ClipboardChannels {}

#[derive(Default)]
pub(super) struct DeferredClipboard {
    state: Rc<RefCell<ClipboardState>>,
}

#[derive(Clone)]
pub(super) struct CapturedClipboardAction {
    state: Rc<RefCell<ClipboardState>>,
    preceding_write: Option<Rc<ClipboardWrite>>,
    item: Option<Rc<ClipboardItem>>,
    provenance: Option<ClipboardProvenance>,
}

#[derive(Clone, Default)]
struct ClipboardProvenance {
    segment: Rc<()>,
    epoch: Rc<()>,
}

#[derive(Default)]
struct ClipboardState {
    revision: Option<ClipboardRevision>,
    provenance: ClipboardProvenance,
    last_write: Option<Rc<ClipboardWrite>>,
}

struct ClipboardWrite {
    item: Result<ClipboardItem, Arc<str>>,
    provenance: Option<ClipboardProvenance>,
}

impl ClipboardChannels {
    fn clipboard(&self) -> DeferredClipboard {
        DeferredClipboard::for_channel(&self.clipboard)
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    fn primary(&self) -> DeferredClipboard {
        DeferredClipboard::for_channel(&self.primary)
    }
}

impl ClipboardProvenance {
    fn same_epoch(&self, other: &Self) -> Option<bool> {
        Rc::ptr_eq(&self.segment, &other.segment).then(|| Rc::ptr_eq(&self.epoch, &other.epoch))
    }
}

impl ClipboardState {
    fn observe(&mut self, revision: Option<ClipboardRevision>) -> Option<ClipboardProvenance> {
        let revision = revision?;
        if self.revision.as_ref() != Some(&revision) {
            self.provenance.epoch = Rc::new(());
            self.revision = Some(revision);
        }
        Some(self.provenance.clone())
    }

    fn lose_continuity(&mut self) {
        self.revision = None;
        self.provenance = ClipboardProvenance::default();
    }
}

impl DeferredClipboard {
    fn for_channel(channel: &RefCell<Weak<RefCell<ClipboardState>>>) -> Self {
        let mut channel = channel.borrow_mut();
        let state = channel.upgrade().unwrap_or_else(|| {
            let state = Rc::new(RefCell::new(ClipboardState::default()));
            *channel = Rc::downgrade(&state);
            state
        });
        Self { state }
    }

    fn capture(
        &self,
        captured: Option<&CapturedClipboardAction>,
        read: impl FnOnce() -> ClipboardSnapshot,
    ) -> CapturedClipboardAction {
        captured
            .filter(|captured| Rc::ptr_eq(&captured.state, &self.state))
            .cloned()
            .unwrap_or_else(|| {
                let snapshot = read();
                let mut state = self.state.borrow_mut();
                let provenance = state.observe(snapshot.revision);
                CapturedClipboardAction {
                    state: self.state.clone(),
                    preceding_write: state.last_write.clone(),
                    item: snapshot.item.map(Rc::new),
                    provenance,
                }
            })
    }

    fn read(
        &self,
        captured: Option<&CapturedClipboardAction>,
        read: impl FnOnce() -> Option<ClipboardItem>,
    ) -> Result<Option<ClipboardItem>> {
        let Some(captured) = captured.filter(|captured| Rc::ptr_eq(&captured.state, &self.state))
        else {
            return Ok(read());
        };
        if let Some(last_write) = self.state.borrow().last_write.as_ref()
            && !captured
                .preceding_write
                .as_ref()
                .is_some_and(|preceding_write| Rc::ptr_eq(preceding_write, last_write))
        {
            match last_write
                .provenance
                .as_ref()
                .zip(captured.provenance.as_ref())
                .and_then(|(write, capture)| write.same_epoch(capture))
            {
                Some(true) => {
                    return last_write
                        .item
                        .as_ref()
                        .map(|item| Some(item.clone()))
                        .map_err(|error| anyhow::anyhow!(error.to_string()));
                }
                Some(false) => {}
                None => anyhow::bail!("Clipboard ordering is unavailable; paste was cancelled"),
            }
        }
        Ok(captured.item.as_deref().cloned())
    }

    fn write(
        &self,
        captured: Option<&CapturedClipboardAction>,
        item: ClipboardItem,
        revision: impl FnOnce() -> Option<ClipboardRevision>,
        publish: impl FnOnce(ClipboardItem) -> Result<Option<ClipboardRevision>>,
    ) -> Result<()> {
        let revision = revision();
        let mut state = self.state.borrow_mut();
        let current = state.observe(revision);
        let captured = captured.filter(|captured| Rc::ptr_eq(&captured.state, &self.state));
        let provenance =
            captured.map_or_else(|| current.clone(), |captured| captured.provenance.clone());
        let superseded = provenance
            .as_ref()
            .is_some_and(|origin| origin.same_epoch(&state.provenance) == Some(false));
        let can_publish = captured.is_none()
            || provenance
                .as_ref()
                .zip(current.as_ref())
                .is_some_and(|(origin, current)| origin.same_epoch(current) == Some(true));
        drop(state);

        let result = if superseded {
            Ok(())
        } else if can_publish {
            let result = publish(item.clone());
            let mut state = self.state.borrow_mut();
            match result {
                Ok(Some(revision)) => {
                    state.revision = Some(revision);
                    Ok(())
                }
                Ok(None) => {
                    state.lose_continuity();
                    Ok(())
                }
                Err(error) => {
                    state.lose_continuity();
                    Err(error)
                }
            }
        } else {
            Err(anyhow::anyhow!(
                "Clipboard ordering is unavailable; copy was cancelled"
            ))
        };
        self.state.borrow_mut().last_write = Some(Rc::new(ClipboardWrite {
            item: result
                .as_ref()
                .map(|()| item)
                .map_err(|error| Arc::from(format!("{error:#}"))),
            provenance,
        }));
        result
    }
}

impl Editor {
    pub fn read_from_clipboard(&self, cx: &App) -> Result<Option<ClipboardItem>> {
        cx.global::<ClipboardChannels>().clipboard().read(
            CapturedInput::current(cx).and_then(|input| input.clipboard.as_ref()),
            || cx.read_from_clipboard(),
        )
    }

    pub fn write_to_clipboard(&mut self, item: ClipboardItem, cx: &mut App) -> Result<()> {
        if self.input_is_replaying() {
            let clipboard = cx.global::<ClipboardChannels>().clipboard();
            clipboard.write(
                CapturedInput::current(cx).and_then(|input| input.clipboard.as_ref()),
                item,
                || {
                    cx.read_clipboard_snapshot(ClipboardReadMode::RevisionOnly)
                        .revision
                },
                |item| cx.write_to_clipboard_with_revision(item),
            )
        } else {
            cx.write_to_clipboard_with_revision(item).map(|_| ())
        }
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub fn read_from_primary(&self, cx: &App) -> Result<Option<ClipboardItem>> {
        cx.global::<ClipboardChannels>().primary().read(
            CapturedInput::current(cx).and_then(|input| input.primary.as_ref()),
            || cx.read_from_primary(),
        )
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub fn write_to_primary(&mut self, item: ClipboardItem, cx: &mut App) -> Result<()> {
        if self.input_is_replaying() {
            let primary = cx.global::<ClipboardChannels>().primary();
            primary.write(
                CapturedInput::current(cx).and_then(|input| input.primary.as_ref()),
                item,
                || {
                    cx.read_primary_snapshot(ClipboardReadMode::RevisionOnly)
                        .revision
                },
                |item| cx.write_to_primary_with_revision(item),
            )
        } else {
            cx.write_to_primary_with_revision(item).map(|_| ())
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
        if self.input_is_deferred() {
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
        if self.defer_clipboard_action(action, Self::paste, window, cx) {
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
        if self.input_is_deferred() {
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
        if self.input_is_deferred() {
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
        if self.clipboard_input_is_deferred() {
            let captured = cx.global::<ClipboardChannels>().clipboard().capture(
                CapturedInput::current(cx).and_then(|input| input.clipboard.as_ref()),
                || cx.read_clipboard_snapshot(ClipboardReadMode::RevisionOnly),
            );
            self.defer_clipboard_publication(
                CapturedInput {
                    clipboard: Some(captured),
                    ..CapturedInput::default()
                },
                move |editor, _, cx| {
                    if let Err(error) = editor.write_to_clipboard(item, cx) {
                        editor.notify_clipboard_error(error, cx);
                    }
                },
                cx,
            );
        } else if let Err(error) = self.write_to_clipboard(item, cx) {
            self.notify_clipboard_error(error, cx);
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
        if self.clipboard_input_is_deferred() {
            let captured = cx.global::<ClipboardChannels>().primary().capture(
                CapturedInput::current(cx).and_then(|input| input.primary.as_ref()),
                || cx.read_primary_snapshot(ClipboardReadMode::RevisionOnly),
            );
            self.defer_clipboard_publication(
                CapturedInput {
                    primary: Some(captured),
                    ..CapturedInput::default()
                },
                move |editor, _, cx| {
                    if let Err(error) = editor.write_to_primary(item, cx) {
                        editor.notify_clipboard_error(error, cx);
                    }
                },
                cx,
            );
        } else if let Err(error) = self.write_to_primary(item, cx) {
            self.notify_clipboard_error(error, cx);
        }
    }

    pub(super) fn capture_clipboard(&self, cx: &App) -> CapturedClipboardAction {
        cx.global::<ClipboardChannels>().clipboard().capture(
            CapturedInput::current(cx).and_then(|input| input.clipboard.as_ref()),
            || cx.read_clipboard_snapshot(ClipboardReadMode::Contents),
        )
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub(super) fn capture_primary(&self, cx: &App) -> CapturedClipboardAction {
        cx.global::<ClipboardChannels>().primary().capture(
            CapturedInput::current(cx).and_then(|input| input.primary.as_ref()),
            || cx.read_primary_snapshot(ClipboardReadMode::Contents),
        )
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub(super) fn read_captured_primary(
        &self,
        captured: &CapturedClipboardAction,
        cx: &App,
    ) -> Result<Option<ClipboardItem>> {
        cx.global::<ClipboardChannels>()
            .primary()
            .read(Some(captured), || cx.read_from_primary())
    }

    pub(super) fn notify_clipboard_error(&self, error: anyhow::Error, cx: &mut App) {
        let message = format!("Clipboard operation failed: {error:#}");
        log::error!("{message}");
        if let Some(workspace) = self.workspace() {
            cx.defer(move |cx| {
                workspace.update(cx, |workspace, cx| {
                    workspace.show_toast(
                        Toast::new(NotificationId::unique::<DeferredClipboard>(), message),
                        cx,
                    );
                });
            });
        }
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
        if let Err(error) = self.write_to_clipboard(item, cx) {
            self.notify_clipboard_error(error, cx);
            return;
        }
        self.delete_cut_ranges(selections, window, cx);
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
        if self.defer_clipboard_action(action, Self::diff_clipboard_with_selection, window, cx) {
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
        if let Err(error) = self.write_to_clipboard(
            ClipboardItem::new_string_with_json_metadata(text, clipboard_selections),
            cx,
        ) {
            self.notify_clipboard_error(error, cx);
        }
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
    use crate::{editor_tests::init_test, test::editor_test_context::EditorTestContext};
    use gpui::{KeyBinding, Keystroke, TestAppContext};
    use language::language_settings::SoftWrap;

    #[gpui::test]
    async fn test_deferred_clipboard_overlay_lives_only_with_captures(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut editor_cx = EditorTestContext::new(cx).await;
        let (capture, completion) = editor_cx.update_editor(|editor, window, cx| {
            cx.write_to_clipboard(ClipboardItem::new_string(String::from("Z")));
            let capture = editor.capture_clipboard(cx);
            let editor_handle = cx.weak_entity();
            editor.schedule_input_continuation(window, cx, move |_, cx| {
                editor_handle
                    .update(cx, |editor, cx| {
                        editor
                            .write_to_clipboard(
                                ClipboardItem::new_string(String::from("outgoing")),
                                cx,
                            )
                            .expect("publication");
                    })
                    .expect("editor");
            });
            (capture, editor.input_completion(cx))
        });
        cx.run_until_parked();
        completion
            .now_or_never()
            .expect("publication completed")
            .expect("publication succeeded");
        let state = Rc::downgrade(&capture.state);
        let write = Rc::downgrade(
            capture
                .state
                .borrow()
                .last_write
                .as_ref()
                .expect("last write"),
        );
        editor_cx.update_editor(|editor, _, cx| {
            cx.write_to_clipboard(ClipboardItem::new_string(String::from("external")));
            CapturedInput {
                clipboard: Some(capture.clone()),
                ..CapturedInput::default()
            }
            .run(cx, |cx| {
                assert_eq!(
                    editor
                        .read_from_clipboard(cx)
                        .expect("captured read")
                        .and_then(|item| item.text()),
                    Some(String::from("outgoing")),
                );
            });
        });
        drop(capture);
        assert!(state.upgrade().is_none());
        assert!(write.upgrade().is_none());
        editor_cx.update_editor(|editor, _, cx| {
            let captured = editor.capture_clipboard(cx);
            assert_eq!(
                captured.item.as_ref().and_then(|item| item.text()),
                Some(String::from("external"))
            );
        });
    }

    #[test]
    fn test_deferred_clipboard_observes_external_writes() {
        for (external, after_paste, expected) in [
            (&[][..], false, "A"),
            (&["B"][..], false, "B"),
            (&["Z"][..], false, "Z"),
            (&["Y", "Z"][..], false, "Z"),
            (&[][..], true, "A"),
        ] {
            let clipboard = DeferredClipboard::default();
            let native = RefCell::new(clipboard_snapshot(Some("Z")));
            let copy = clipboard.capture(None, || native.borrow().clone());
            for text in external {
                publish_clipboard(&native, ClipboardItem::new_string((*text).to_owned()))
                    .expect("external publication");
            }
            let paste = clipboard.capture(None, || native.borrow().clone());
            if after_paste {
                publish_clipboard(&native, ClipboardItem::new_string(String::from("late")))
                    .expect("late publication");
            }
            clipboard
                .write(
                    Some(&copy),
                    ClipboardItem::new_string(String::from("A")),
                    || native.borrow().revision.clone(),
                    |item| publish_clipboard(&native, item),
                )
                .expect("deferred publication");
            assert_eq!(
                clipboard
                    .read(Some(&paste), || panic!("paste reread clipboard"))
                    .expect("captured clipboard read")
                    .and_then(|item| item.text())
                    .as_deref(),
                Some(expected),
                "external={external:?}, after_paste={after_paste}",
            );
            assert_eq!(
                native
                    .borrow()
                    .item
                    .as_ref()
                    .and_then(|item| item.text())
                    .as_deref(),
                Some(if after_paste { "late" } else { expected }),
            );
        }
    }

    #[test]
    fn test_deferred_clipboard_channels_have_independent_writes() {
        let clipboard = DeferredClipboard::default();
        let primary = DeferredClipboard::default();
        let native_clipboard = RefCell::new(clipboard_snapshot(None));
        let native_primary = RefCell::new(clipboard_snapshot(Some("primary")));
        let captured_clipboard = clipboard.capture(None, || native_clipboard.borrow().clone());
        let captured_primary = primary.capture(None, || native_primary.borrow().clone());
        clipboard
            .write(
                Some(&captured_clipboard),
                ClipboardItem::new_string(String::from("copy")),
                || native_clipboard.borrow().revision.clone(),
                |item| publish_clipboard(&native_clipboard, item),
            )
            .expect("clipboard publication");
        let consumed = primary
            .read(Some(&captured_primary), || None)
            .expect("captured primary read");
        primary
            .write(
                Some(&captured_primary),
                ClipboardItem::new_string(String::from("selection")),
                || native_primary.borrow().revision.clone(),
                |item| publish_clipboard(&native_primary, item),
            )
            .expect("primary publication");
        assert_eq!(
            consumed.and_then(|item| item.text()).as_deref(),
            Some("primary")
        );
        assert_eq!(
            clipboard
                .read(Some(&captured_clipboard), || None)
                .expect("captured clipboard read")
                .and_then(|item| item.text())
                .as_deref(),
            Some("copy")
        );
        assert_eq!(
            primary
                .read(Some(&captured_primary), || None)
                .expect("captured primary read")
                .and_then(|item| item.text())
                .as_deref(),
            Some("selection")
        );
        let retried = primary.capture(Some(&captured_primary), || panic!("retry reread primary"));
        assert!(Rc::ptr_eq(
            retried.item.as_ref().expect("retried item"),
            captured_primary.item.as_ref().expect("captured item")
        ));
        let failed_primary = primary.capture(None, || ClipboardSnapshot {
            item: native_primary.borrow().item.clone(),
            revision: None,
        });
        primary
            .write(
                Some(&failed_primary),
                ClipboardItem::new_string(String::from("unpublished")),
                || None,
                |_| panic!("ambiguous primary publication"),
            )
            .expect_err("primary ordering is unavailable");
        assert_eq!(
            clipboard
                .read(Some(&captured_clipboard), || panic!("clipboard reread"))
                .expect("primary failure leaves clipboard independent")
                .and_then(|item| item.text())
                .as_deref(),
            Some("copy"),
        );
        assert_eq!(
            primary
                .read(Some(&captured_clipboard), || None)
                .expect("other channel read"),
            None
        );
        assert_eq!(
            DeferredClipboard::default()
                .read(Some(&retried), || None)
                .expect("new state read"),
            None
        );
    }

    #[test]
    fn test_deferred_clipboard_partial_drain() {
        let clipboard = DeferredClipboard::default();
        let native = RefCell::new(clipboard_snapshot(Some("Z")));
        let first = clipboard.capture(None, || native.borrow().clone());
        let second = clipboard.capture(None, || native.borrow().clone());
        clipboard
            .write(
                Some(&first),
                ClipboardItem::new_string(String::from("A")),
                || native.borrow().revision.clone(),
                |item| publish_clipboard(&native, item),
            )
            .expect("first publication");
        let paste = clipboard.capture(None, || native.borrow().clone());
        clipboard
            .write(
                Some(&second),
                ClipboardItem::new_string(String::from("C")),
                || native.borrow().revision.clone(),
                |item| publish_clipboard(&native, item),
            )
            .expect("second publication");
        assert_eq!(
            clipboard
                .read(Some(&paste), || None)
                .expect("partial drain read")
                .and_then(|item| item.text())
                .as_deref(),
            Some("C")
        );
        *native.borrow_mut() = clipboard_snapshot(Some("B"));
        clipboard
            .write(
                Some(&second),
                ClipboardItem::new_string(String::from("stale")),
                || native.borrow().revision.clone(),
                |_| panic!("stale native publication"),
            )
            .expect("superseded publication");
        let paste = clipboard.capture(None, || native.borrow().clone());
        assert_eq!(
            clipboard
                .read(Some(&paste), || None)
                .expect("external snapshot read")
                .and_then(|item| item.text())
                .as_deref(),
            Some("B")
        );
        clipboard
            .write(
                Some(&second),
                ClipboardItem::new_string(String::from("stale")),
                || None,
                |_| panic!("lost a known external observation"),
            )
            .expect("superseded publication without a current observation");
    }

    #[test]
    fn test_deferred_clipboard_unavailable_identity() {
        for text in [None, Some("B")] {
            let clipboard = DeferredClipboard::default();
            let native = RefCell::new(ClipboardSnapshot {
                item: text.map(|text| ClipboardItem::new_string(text.to_owned())),
                revision: None,
            });
            let copy = clipboard.capture(None, || native.borrow().clone());
            let paste = clipboard.capture(None, || native.borrow().clone());
            native.borrow_mut().item = Some(ClipboardItem::new_string(String::from("late")));
            assert_eq!(
                clipboard
                    .read(Some(&paste), || panic!("captured clipboard reread"))
                    .expect("unconflicted snapshot")
                    .and_then(|item| item.text())
                    .as_deref(),
                text,
            );
            assert_eq!(
                clipboard
                    .write(
                        Some(&copy),
                        ClipboardItem::new_string(String::from("A")),
                        || None,
                        |_| panic!("ambiguous native publication"),
                    )
                    .expect_err("ambiguous publication must fail")
                    .to_string(),
                "Clipboard ordering is unavailable; copy was cancelled",
            );
            assert_eq!(
                clipboard
                    .read(Some(&paste), || panic!("failed dependency reread"))
                    .expect_err("ambiguous dependency must fail")
                    .to_string(),
                "Clipboard ordering is unavailable; paste was cancelled",
            );
            let fresh = clipboard.capture(None, || native.borrow().clone());
            assert_eq!(
                clipboard
                    .read(Some(&fresh), || panic!("fresh snapshot reread"))
                    .expect("snapshot after the failed operation")
                    .and_then(|item| item.text())
                    .as_deref(),
                Some("late"),
            );
            clipboard
                .write(
                    None,
                    ClipboardItem::new_string(String::from("direct")),
                    || None,
                    |item| {
                        native.borrow_mut().item = Some(item);
                        Ok(None)
                    },
                )
                .expect("direct unversioned publication");
            assert_eq!(
                native
                    .borrow()
                    .item
                    .as_ref()
                    .and_then(|item| item.text())
                    .as_deref(),
                Some("direct")
            );
        }
    }

    #[test]
    fn test_deferred_clipboard_publication_loses_continuity() {
        for fails in [false, true] {
            let clipboard = DeferredClipboard::default();
            let native = RefCell::new(clipboard_snapshot(Some("Z")));
            let first = clipboard.capture(None, || native.borrow().clone());
            let second = clipboard.capture(None, || native.borrow().clone());
            let early_paste = clipboard.capture(None, || native.borrow().clone());
            let result = clipboard.write(
                Some(&first),
                ClipboardItem::new_string(String::from("A")),
                || native.borrow().revision.clone(),
                |item| {
                    publish_clipboard(&native, item)?;
                    if fails {
                        anyhow::bail!("injected publication failure");
                    }
                    Ok(None)
                },
            );
            let early_read = clipboard.read(Some(&early_paste), || panic!("early paste reread"));
            if fails {
                assert_eq!(
                    result.expect_err("publication failed").to_string(),
                    "injected publication failure"
                );
                assert_eq!(
                    early_read
                        .expect_err("failed publication dependency")
                        .to_string(),
                    "injected publication failure"
                );
            } else {
                result.expect("unversioned publication succeeded");
                assert_eq!(
                    early_read
                        .expect("logical write before receipt loss")
                        .and_then(|item| item.text())
                        .as_deref(),
                    Some("A")
                );
            }
            let paste = clipboard.capture(None, || native.borrow().clone());
            assert_eq!(
                clipboard
                    .write(
                        Some(&second),
                        ClipboardItem::new_string(String::from("C")),
                        || native.borrow().revision.clone(),
                        |_| panic!("publication across an uncertain segment"),
                    )
                    .expect_err("uncertain publication must fail")
                    .to_string(),
                "Clipboard ordering is unavailable; copy was cancelled"
            );
            assert_eq!(
                clipboard
                    .read(Some(&paste), || panic!("partial drain reread"))
                    .expect_err("partial drain lost continuity")
                    .to_string(),
                "Clipboard ordering is unavailable; paste was cancelled"
            );
            let fresh = clipboard.capture(None, || native.borrow().clone());
            clipboard
                .write(
                    Some(&fresh),
                    ClipboardItem::new_string(String::from("D")),
                    || native.borrow().revision.clone(),
                    |item| publish_clipboard(&native, item),
                )
                .expect("publication in the new segment");
            assert_eq!(
                clipboard
                    .read(Some(&fresh), || panic!("new segment reread"))
                    .expect("new segment dependency")
                    .and_then(|item| item.text())
                    .as_deref(),
                Some("D")
            );
        }
    }

    #[gpui::test]
    async fn test_deferred_clipboard_keystroke_factory_is_lazy(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        cx.update_editor(|editor, window, cx| {
            assert!(!editor.defer_keystroke_input(
                |_, _| panic!("ready input constructed a keystroke replay"),
                window,
                cx,
            ));
        });
    }

    #[gpui::test]
    async fn test_deferred_clipboard_fallback_captures_payload(cx: &mut TestAppContext) {
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
                    assert!(!editor.input_is_deferred());
                    if already_pending {
                        editor.move_up(&MoveUp, window, cx);
                        assert!(editor.input_is_deferred());
                    }
                });
                cx.write_to_clipboard(ClipboardItem::new_string(String::from("A")));
                window.dispatch_keystroke(Keystroke::parse("ctrl-v").expect("fallback key"), cx);
                cx.write_to_clipboard(ClipboardItem::new_string(String::from("B")));
                editor.update(cx, |editor, cx| {
                    assert!(editor.input_is_deferred());
                    assert_eq!(editor.text(cx), text);
                    editor.input_completion(cx)
                })
            });
            cx.background_executor.set_block_on_ticks(0..=50);
            completion.await.expect("clipboard fallback completed");
            cx.assert_editor_state(&format!("Aˇ{text}"));
        }
    }

    #[gpui::test]
    async fn test_deferred_clipboard_captures_each_paste(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for ingress in [
            PasteIngress::Direct,
            PasteIngress::Action,
            PasteIngress::RegisteredAction,
            PasteIngress::RegisteredKeyboard,
        ] {
            let (mut cx, _subscription) = clipboard_test_context(ingress, cx).await;
            let editor = cx.editor.clone();
            let completion = cx.update(|window, cx| {
                start_deferred_clipboard_input(&editor, window, cx);
                for text in ["A", "B"] {
                    cx.write_to_clipboard(ClipboardItem::new_string(text.to_owned()));
                    dispatch_paste(ingress, &editor, window, cx);
                }
                cx.write_to_clipboard(ClipboardItem::new_string(String::from("C")));
                editor.update(cx, |editor, cx| editor.input_completion(cx))
            });
            completion.await.expect("clipboard input completed");
            cx.assert_editor_state(&format!("ab\nxABˇ{}\ncd", "x".repeat(1_999)));
        }
    }

    #[gpui::test]
    async fn test_deferred_clipboard_replayed_writes_are_dependencies(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for ingress in [
            PasteIngress::Direct,
            PasteIngress::Action,
            PasteIngress::RegisteredAction,
            PasteIngress::RegisteredKeyboard,
        ] {
            for (external, after_paste, expected) in [
                (&[][..], false, "A"),
                (&["B"][..], false, "B"),
                (&["Z"][..], false, "Z"),
                (&["Y", "Z"][..], false, "Z"),
                (&[][..], true, "A"),
            ] {
                for cold in [false, true] {
                    let (mut cx, _subscription) = clipboard_test_context(ingress, cx).await;
                    let editor = cx.editor.clone();
                    let line = "x".repeat(2_000);
                    let completion = cx.update_editor(|editor, window, cx| {
                        editor.set_soft_wrap_mode(SoftWrap::None, cx);
                        editor.set_text(format!("A\n{line}"), window, cx);
                        editor.change_selections(
                            SelectionEffects::no_scroll(),
                            window,
                            cx,
                            |selections| {
                                selections.select_ranges([Point::new(0, 0)..Point::new(0, 0)]);
                            },
                        );
                        editor.move_down(&MoveDown, window, cx);
                        assert!(editor.input_is_deferred());
                        editor.input_completion(cx)
                    });
                    if !cold {
                        completion.await.expect("warm clipboard geometry");
                    }
                    let completion = cx.update(|window, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(String::from("Z")));
                        editor.update(cx, |editor, cx| {
                            let select =
                                |editor: &mut Editor,
                                 window: &mut Window,
                                 cx: &mut Context<Editor>| {
                                    editor.change_selections(
                                        SelectionEffects::no_scroll(),
                                        window,
                                        cx,
                                        |selections| {
                                            selections.select_ranges([
                                                Point::new(0, 0)..Point::new(0, 1)
                                            ]);
                                        },
                                    );
                                };
                            if !editor.defer_input(select, window, cx) {
                                select(editor, window, cx);
                            }
                            editor.copy(&Copy, window, cx);
                        });
                        for text in external {
                            cx.write_to_clipboard(ClipboardItem::new_string((*text).to_owned()));
                        }
                        dispatch_paste(ingress, &editor, window, cx);
                        if after_paste {
                            cx.write_to_clipboard(ClipboardItem::new_string(String::from("late")));
                        }
                        editor.update(cx, |editor, cx| editor.input_completion(cx))
                    });
                    completion.await.expect("clipboard provenance completed");
                    cx.assert_editor_state(&format!("{expected}ˇ\n{line}"));
                    cx.update(|_, cx| {
                        assert_eq!(
                            cx.read_from_clipboard()
                                .and_then(|item| item.text())
                                .as_deref(),
                            Some(if after_paste { "late" } else { expected })
                        );
                    });
                }
            }
        }
    }

    #[gpui::test]
    async fn test_deferred_clipboard_captures_empty_clipboard(cx: &mut TestAppContext) {
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
        completion.await.expect("empty clipboard input completed");
        cx.assert_editor_state(&format!("ab\nxˇ{}\ncd", "x".repeat(1_999)));
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
                        assert!(editor.input_is_deferred());
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
        cx.assert_editor_state(&format!("ab\ncd\nAˇ{}", "y".repeat(2_000)));
    }

    #[gpui::test]
    async fn test_deferred_clipboard_failed_cut_preserves_text(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for (cold, unavailable_provenance) in [(false, false), (true, false), (true, true)] {
            for to_end_of_line in [false, true] {
                let (mut cx, _subscription) =
                    clipboard_test_context(PasteIngress::Direct, cx).await;
                let editor = cx.editor.clone();
                let completion = cx.update(|window, cx| {
                    cx.write_to_clipboard(ClipboardItem::new_string(String::from("Z")));
                    if cold {
                        start_deferred_clipboard_input(&editor, window, cx);
                    } else {
                        editor.update(cx, |editor, cx| {
                            editor.set_text(String::from("ab"), window, cx);
                            editor.change_selections(
                                SelectionEffects::no_scroll(),
                                window,
                                cx,
                                |selections| {
                                    selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
                                },
                            );
                        });
                    }
                    editor.update(cx, |editor, cx| {
                        let mut captured = editor.capture_clipboard(cx);
                        if unavailable_provenance {
                            captured.provenance = None;
                        } else {
                            cx.fail_next_checked_clipboard_write();
                        }
                        CapturedInput {
                            clipboard: Some(captured),
                            ..CapturedInput::default()
                        }
                        .run(cx, |cx| {
                            if to_end_of_line {
                                editor.cut_to_end_of_line(
                                    &CutToEndOfLine {
                                        stop_at_newlines: false,
                                    },
                                    window,
                                    cx,
                                );
                            } else {
                                editor.cut(&Cut, window, cx);
                            }
                            if cold {
                                editor.paste(&Paste, window, cx);
                            }
                        });
                        editor.input_completion(cx)
                    })
                });
                completion
                    .await
                    .expect("failed clipboard operations completed");
                if cold {
                    cx.assert_editor_state(&format!("ab\nxˇ{}\ncd", "x".repeat(1_999)));
                } else {
                    cx.assert_editor_state("aˇb");
                }
                cx.update(|_, cx| {
                    assert_eq!(
                        cx.read_from_clipboard()
                            .and_then(|item| item.text())
                            .as_deref(),
                        Some("Z"),
                    );
                });
            }
        }
    }

    #[gpui::test]
    async fn test_cut_to_end_of_line_preserves_pending_selection_on_failure(
        cx: &mut TestAppContext,
    ) {
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
                let pending = editor.selections.pending_anchor().copied();
                let disjoint = editor.selections.disjoint_anchors_arc();
                assert!(pending.is_some());
                cx.write_to_clipboard(ClipboardItem::new_string(String::from("Z")));
                cx.fail_next_checked_clipboard_write();

                editor.cut_to_end_of_line(&CutToEndOfLine::default(), window, cx);

                assert_eq!(editor.text(cx), "abc\ndef");
                assert_eq!(editor.selections.pending_anchor().copied(), pending);
                assert_eq!(editor.selections.disjoint_anchors_arc(), disjoint);
                assert_eq!(
                    cx.read_from_clipboard().and_then(|item| item.text()),
                    Some(String::from("Z"))
                );

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

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum PasteIngress {
        Direct,
        Action,
        RegisteredAction,
        RegisteredKeyboard,
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
            assert!(editor.input_is_deferred());
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
        }
    }

    fn clipboard_snapshot(text: Option<&str>) -> ClipboardSnapshot {
        ClipboardSnapshot {
            item: text.map(|text| ClipboardItem::new_string(text.to_owned())),
            revision: Some(ClipboardRevision::default()),
        }
    }

    fn publish_clipboard(
        native: &RefCell<ClipboardSnapshot>,
        item: ClipboardItem,
    ) -> Result<Option<ClipboardRevision>> {
        let revision = ClipboardRevision::default();
        *native.borrow_mut() = ClipboardSnapshot {
            item: Some(item),
            revision: Some(revision.clone()),
        };
        Ok(Some(revision))
    }
}
