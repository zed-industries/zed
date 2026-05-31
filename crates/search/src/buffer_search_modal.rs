use collections::HashMap;
use editor::{
    Anchor as MultiBufferAnchor, CurrentLineHighlight, Editor, EditorEvent, EditorSettings,
    HighlightKey, MultiBuffer, MultiBufferOffset, MultiBufferSnapshot, SelectionEffects,
    ToOffset, scroll::Autoscroll,
};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Global,
    HighlightStyle, KeyBinding, KeyContext, Render, SharedString, StyledText, Subscription, Task,
    UpdateGlobal, WeakEntity, Window, actions,
};
use language::language_settings::SoftWrap;
use language::{HighlightId, LanguageAwareStyling, Point};
use picker::{Picker, PickerDelegate};
use project::search::SearchQuery;
use settings::Settings;
use std::{
    ops::Range,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use text::Bias;
use ui::{
    Button, ButtonStyle, Color, CommonAnimationExt, Divider, Icon, IconName,
    KeyBinding as UiKeyBinding, Label, ListItem, Tooltip, prelude::*,
};
use ui_input::ErasedEditor;
use util::{ResultExt, paths::PathMatcher};
use vim_mode_setting::VimModeSetting;
use workspace::searchable::SearchableItem;
use workspace::{ModalView, Workspace};

use crate::{NextHistoryQuery, PreviousHistoryQuery};
use project::search_history::{SearchHistory, SearchHistoryCursor};

actions!(buffer_search_modal, [ToggleBufferSearch, ToggleLineMode]);

struct BufferSearchHistory(SearchHistory);
impl Global for BufferSearchHistory {}

const MAX_PREVIEW_BYTES: usize = 200;
const PREVIEW_DEBOUNCE_MS: u64 = 50;

type AnchorRange = Range<MultiBufferAnchor>;

pub fn init(cx: &mut App) {
    cx.set_global(BufferSearchHistory(SearchHistory::new(
        Some(50),
        project::search_history::QueryInsertionBehavior::ReplacePreviousIfContains,
    )));
    cx.bind_keys([
        KeyBinding::new("ctrl-c", NextHistoryQuery, Some("BufferSearchModal")),
        KeyBinding::new("ctrl-t", PreviousHistoryQuery, Some("BufferSearchModal")),
        KeyBinding::new("ctrl-r", ToggleLineMode, Some("BufferSearchModal")),
    ]);
    cx.observe_new(BufferSearchModal::register).detach();
}

#[derive(Clone)]
struct LineMatchData {
    line_label: SharedString,
    preview_text: SharedString,
    // Range in the preview text for the specific match this item represents (for list item highlighting)
    list_match_ranges: Arc<Vec<Range<usize>>>,
    active_match_index_in_list: Option<usize>,
    syntax_highlights: Option<Arc<Vec<(Range<usize>, HighlightId)>>>,
    // The offset of the match this item specifically represents (for sorting/selection)
    primary_match_offset: usize,
    // The range of match indices in all_matches that this line contains
    match_indices: Range<usize>,
}

// Helper to find safe char boundaries for highlighting
fn find_safe_char_boundaries(text: &str, start: usize, end: usize) -> Option<(usize, usize)> {
    let mut safe_start = start.min(text.len());
    while safe_start > 0 && !text.is_char_boundary(safe_start) {
        safe_start -= 1;
    }

    let mut safe_end = end.min(text.len());
    while safe_end > 0 && !text.is_char_boundary(safe_end) {
        safe_end -= 1;
    }

    if safe_start < safe_end {
        Some((safe_start, safe_end))
    } else {
        None
    }
}

// Truncate preview text with ellipsis
fn truncate_preview(text: &str, max_bytes: usize) -> SharedString {
    let trimmed = text.trim();
    if trimmed.len() <= max_bytes {
        return trimmed.to_string().into();
    }

    let mut end = max_bytes;
    while end > 0 && !trimmed.is_char_boundary(end) {
        end -= 1;
    }

    let mut result = trimmed[..end].to_string();
    result.push('…');
    result.into()
}

fn merge_highlights(
    syntax: &[(Range<usize>, HighlightStyle)],
    matches: &[(Range<usize>, HighlightStyle)],
) -> Vec<(Range<usize>, HighlightStyle)> {
    let mut endpoints = Vec::new();
    for (r, _) in syntax {
        endpoints.push(r.start);
        endpoints.push(r.end);
    }
    for (r, _) in matches {
        endpoints.push(r.start);
        endpoints.push(r.end);
    }
    endpoints.sort_unstable();
    endpoints.dedup();

    let mut result = Vec::new();
    for i in 0..endpoints.len().saturating_sub(1) {
        let start = endpoints[i];
        let end = endpoints[i + 1];
        if start >= end {
            continue;
        }
        let range = start..end;

        // Syntax style
        let syn_style = syntax
            .iter()
            .find(|(r, _)| r.start <= start && r.end >= end)
            .map(|(_, s)| s);
        // Match style
        let mat_style = matches
            .iter()
            .find(|(r, _)| r.start <= start && r.end >= end)
            .map(|(_, s)| s);

        if syn_style.is_none() && mat_style.is_none() {
            continue;
        }

        let mut style = syn_style.cloned().unwrap_or_default();
        if let Some(mat) = mat_style {
            if let Some(bg) = mat.background_color {
                style.background_color = Some(bg);
            }
            if let Some(fw) = mat.font_weight {
                style.font_weight = Some(fw);
            }
            if let Some(c) = mat.color {
                style.color = Some(c);
            }
            if let Some(u) = mat.underline {
                style.underline = Some(u);
            }
        }
        result.push((range, style));
    }
    result
}

#[inline]
fn preview_content_len(preview_text: &str) -> usize {
    preview_text
        .len()
        .saturating_sub(if preview_text.ends_with('…') {
            '…'.len_utf8()
        } else {
            0
        })
}

pub struct BufferSearchDelegate {
    target_editor: Entity<Editor>,
    target_buffer: Entity<MultiBuffer>,
    line_mode: bool,
    items: Vec<LineMatchData>,
    selected_index: usize,
    initial_cursor_offset: usize,
    search_cancelled: Option<Arc<AtomicBool>>,
    buffer_search_modal: WeakEntity<BufferSearchModal>,
    match_count: usize,
    is_searching: bool,
    current_query: String,
    focus_handle: Option<FocusHandle>,
    regex_error: Option<String>,
    all_matches: Arc<Vec<AnchorRange>>,
    search_history_cursor: SearchHistoryCursor,
}

pub struct BufferSearchModal {
    picker: Entity<Picker<BufferSearchDelegate>>,
    preview_editor: Option<Entity<Editor>>,
    target_buffer: Entity<MultiBuffer>,
    _picker_subscription: Subscription,
    _preview_editor_subscription: Option<Subscription>,
    _preview_debounce_task: Option<Task<()>>,
}

impl ModalView for BufferSearchModal {}

impl EventEmitter<DismissEvent> for BufferSearchModal {}

impl Focusable for BufferSearchModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for BufferSearchModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let preview_editor = self.preview_editor.clone();
        let picker = self.picker.clone();

        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("BufferSearchModal");

        let viewport_size = window.viewport_size();

        let modal_width = (viewport_size.width * 0.95).min(viewport_size.width);
        // needs to be a bit lower than the viewport height to avoid the dialog going off screen at the bottom
        let modal_height = (viewport_size.height * 0.8).min(viewport_size.height);

        let border_color = cx.theme().colors().border;

        let results_panel = v_flex()
            .flex_shrink_0()
            .min_h_0()
            .overflow_hidden()
            .h(rems(12.))
            .border_b_1()
            .border_color(border_color)
            .child(self.picker.clone());

        let preview_panel = v_flex()
            .id("buffer-search-preview")
            .relative()
            .flex_1()
            .overflow_hidden()
            .bg(cx.theme().colors().elevated_surface_background)
            .on_click(move |_, window, cx| {
                window.focus(&picker.focus_handle(cx), cx);
            })
            .when_some(preview_editor, |this, editor| this.child(editor))
            .when(self.preview_editor.is_none(), |this| {
                this.child(
                    div()
                        .size_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(Label::new("Select a result to preview").color(Color::Muted)),
                )
            });

        div()
            .id("buffer-search-modal")
            .key_context(key_context)
            .relative()
            .h(modal_height)
            .w(modal_width)
            .child(
                v_flex()
                    .elevation_3(cx)
                    .size_full()
                    .overflow_hidden()
                    .border_1()
                    .border_color(border_color)
                    .child(results_panel)
                    .child(preview_panel),
            )
            .on_action(cx.listener(Self::next_history_query))
            .on_action(cx.listener(Self::previous_history_query))
    }
}

impl BufferSearchModal {
    pub fn toggle_for_editor(
        workspace: &mut Workspace,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let (selected_text, buffer, cursor_offset) = editor.update(cx, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let selection = editor.selections.newest_anchor();
            let range = selection.range();
            let head = selection.head();

            let selected_text = if range.start.cmp(&range.end, &snapshot).is_ne() {
                let start = range.start.to_offset(&snapshot).0;
                let end = range.end.to_offset(&snapshot).0;
                let mut text = snapshot
                    .text_for_range(MultiBufferOffset(start)..MultiBufferOffset(end))
                    .collect::<String>();
                if text.ends_with('\n') {
                    text.pop();
                }
                Some(text)
            } else if !VimModeSetting::get_global(cx).0 {
                let query = editor.query_suggestion(None, window, cx);
                if query.is_empty() { None } else { Some(query) }
            } else {
                None
            };

            let buffer = editor.buffer().clone();
            let cursor_offset = head.to_offset(&snapshot).0;

            (selected_text, buffer, cursor_offset)
        });

        let weak_workspace = cx.entity().downgrade();
        workspace.toggle_modal(window, cx, |window, cx| {
            BufferSearchModal::new(
                weak_workspace,
                editor,
                buffer,
                cursor_offset,
                selected_text,
                window,
                cx,
            )
        });
    }

    fn next_history_query(
        &mut self,
        _: &NextHistoryQuery,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut cursor = self.picker.read(cx).delegate.search_history_cursor.clone();

        let next_query = BufferSearchHistory::update_global(cx, |history, _| {
            history.0.next(&mut cursor).map(|s| s.to_string())
        });

        self.picker.update(cx, |picker, cx| {
            picker.delegate.search_history_cursor = cursor;
            if let Some(query) = next_query {
                picker.set_query(&query, window, cx);
            } else {
                picker.delegate.search_history_cursor.reset();
                picker.set_query("", window, cx);
            }
        });
    }

    fn previous_history_query(
        &mut self,
        _: &PreviousHistoryQuery,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let (current_query_empty, current_query, cursor_snapshot) = {
            let picker = self.picker.read(cx);
            (
                picker.query(cx).is_empty(),
                picker.query(cx).to_string(),
                picker.delegate.search_history_cursor.clone(),
            )
        };

        if current_query_empty {
            if let Some(query) = cx
                .global::<BufferSearchHistory>()
                .0
                .current(&cursor_snapshot)
                .map(|s| s.to_string())
            {
                self.picker
                    .update(cx, |picker, cx| picker.set_query(&query, window, cx));
                return;
            }
        }

        let mut cursor_mut = cursor_snapshot;
        let prev_query = BufferSearchHistory::update_global(cx, |history, _| {
            history
                .0
                .previous(&mut cursor_mut, &current_query)
                .map(|s| s.to_string())
        });

        if let Some(query) = prev_query {
            self.picker.update(cx, |picker, cx| {
                picker.delegate.search_history_cursor = cursor_mut;
                picker.set_query(&query, window, cx);
            });
        }
    }

    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _cx: &mut Context<Workspace>,
    ) {
        workspace.register_action(|workspace, _: &ToggleBufferSearch, window, cx| {
            let Some(editor) = workspace
                .active_item(cx)
                .and_then(|item| item.act_as::<Editor>(cx))
            else {
                return;
            };

            Self::toggle_for_editor(workspace, editor, window, cx);
        });

        workspace.register_action(Self::toggle_line_mode);
    }

    fn toggle_line_mode(
        workspace: &mut Workspace,
        _: &ToggleLineMode,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if let Some(modal) = workspace.active_modal::<Self>(cx) {
            modal.update(cx, |modal, cx| {
                modal.picker.update(cx, |picker, cx| {
                    picker.delegate.line_mode = !picker.delegate.line_mode;
                    let query = picker.delegate.current_query.clone();
                    picker.set_query(&query, window, cx);
                });
            });
        }
    }

    fn new(
        _workspace: WeakEntity<Workspace>,
        target_editor: Entity<Editor>,
        target_buffer: Entity<MultiBuffer>,
        initial_cursor_offset: usize,
        initial_query: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let weak_self = cx.entity().downgrade();

        let delegate = BufferSearchDelegate {
            target_editor,
            target_buffer: target_buffer.clone(),
            line_mode: true,
            items: Vec::new(),
            selected_index: 0,
            initial_cursor_offset,
            search_cancelled: None,
            buffer_search_modal: weak_self,
            match_count: 0,
            is_searching: false,
            current_query: initial_query.clone().unwrap_or_default(),
            focus_handle: None,
            regex_error: None,
            all_matches: Arc::new(Vec::new()),
            search_history_cursor: SearchHistoryCursor::default(),
        };

        let picker = cx.new(|cx| {
            let mut picker = Picker::uniform_list(delegate, window, cx)
                .modal(false)
                .max_height(None)
                .show_scrollbar(true);
            picker.delegate.focus_handle = Some(picker.focus_handle(cx));
            if let Some(q) = initial_query {
                picker.set_query(&q, window, cx);
            }
            picker
        });

        let picker_subscription = cx.subscribe_in(&picker, window, Self::on_picker_event);

        Self {
            picker,
            preview_editor: None,
            target_buffer,
            _picker_subscription: picker_subscription,
            _preview_editor_subscription: None,
            _preview_debounce_task: None,
        }
    }

    fn on_picker_event(
        &mut self,
        _picker: &Entity<Picker<BufferSearchDelegate>>,
        _event: &DismissEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.emit(DismissEvent);
    }

    fn on_preview_editor_event(
        &mut self,
        _editor: &Entity<Editor>,
        event: &EditorEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        if !matches!(event, EditorEvent::Edited { .. }) {
            return;
        }
    }

    fn navigate_and_highlight_matches(
        editor: &mut Editor,
        match_offset: usize,
        active_match_indices: Range<usize>,
        match_ranges: &[AnchorRange],
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
        let offset = MultiBufferOffset(match_offset);
        let anchor = buffer_snapshot.anchor_before(offset);
        editor.change_selections(Default::default(), window, cx, |s| {
            s.select_anchor_ranges([anchor..anchor])
        });
        editor.request_autoscroll(Autoscroll::center(), cx);

        editor.highlight_background(
            HighlightKey::BufferSearchHighlights,
            match_ranges,
            move |index, theme| {
                if active_match_indices.contains(index) {
                    theme.colors().search_active_match_background
                } else {
                    theme.colors().search_match_background
                }
            },
            cx,
        );
    }

    fn schedule_preview_update(
        &mut self,
        data: Option<(usize, Range<usize>, Arc<Vec<AnchorRange>>)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self._preview_debounce_task = Some(cx.spawn_in(window, async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(PREVIEW_DEBOUNCE_MS))
                .await;

            this.update_in(cx, |this, window, cx| {
                this.update_preview(data, window, cx);
            })
            .log_err();
        }));
    }

    fn update_preview(
        &mut self,
        data: Option<(usize, Range<usize>, Arc<Vec<AnchorRange>>)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((match_offset, active_indices, match_ranges)) = data else {
            self.preview_editor = None;
            self._preview_editor_subscription = None;
            cx.notify();
            return;
        };

        if let Some(editor) = &self.preview_editor {
            editor.update(cx, |editor, cx| {
                editor.set_current_line_highlight(Some(CurrentLineHighlight::Line));
                Self::navigate_and_highlight_matches(
                    editor,
                    match_offset,
                    active_indices.clone(),
                    &match_ranges,
                    window,
                    cx,
                );
            });
            cx.notify();
            return;
        }

        let buffer = self.target_buffer.clone();

        let editor = cx.new(|cx| {
            let mut editor = Editor::for_multibuffer(buffer.clone(), None, window, cx);
            editor.set_show_gutter(true, cx);
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_current_line_highlight(Some(CurrentLineHighlight::Line));
            editor
        });

        editor.update(cx, |editor, cx| {
            Self::navigate_and_highlight_matches(
                editor,
                match_offset,
                active_indices,
                &match_ranges,
                window,
                cx,
            );
        });

        self._preview_editor_subscription =
            Some(cx.subscribe_in(&editor, window, Self::on_preview_editor_event));
        self.preview_editor = Some(editor);
        cx.notify();
    }
}

fn build_search_query(query: &str) -> Result<SearchQuery, String> {
    SearchQuery::text(
        query,
        false, // whole_word: always false
        true,  // case_sensitive: always true
        false, // include_ignored: always false
        PathMatcher::default(),
        PathMatcher::default(),
        false,
        None,
    )
    .map_err(|e| e.to_string())
}

impl BufferSearchDelegate {
    fn spawn_line_search(
        &self,
        query: String,
        initial_cursor: usize,
        buffer_snapshot: MultiBufferSnapshot,
        cancelled: Arc<AtomicBool>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let window_handle = window.window_handle();
        cx.spawn(async move |picker, cx| {
            if cancelled.load(Ordering::Relaxed) {
                return;
            }

            let cancelled_clone = cancelled.clone();

            let (new_items, all_match_ranges) = cx
                .background_executor()
                .spawn(async move {
                    let terms: Vec<String> = query.split_whitespace().map(str::to_string).collect();

                    let line_count = buffer_snapshot.max_point().row + 1;
                    let mut new_items = Vec::new();
                    let mut all_match_ranges = Vec::new();

                    for line in 0..line_count {
                        if cancelled_clone.load(Ordering::Relaxed) {
                            return (new_items, all_match_ranges);
                        }

                        let line_start_offset =
                            buffer_snapshot.point_to_offset(Point::new(line, 0));
                        let line_end_offset = if line < buffer_snapshot.max_point().row {
                            buffer_snapshot.point_to_offset(Point::new(line + 1, 0))
                        } else {
                            buffer_snapshot.len()
                        };

                        let line_text: String = buffer_snapshot
                            .text_for_range(line_start_offset..line_end_offset)
                            .collect();

                        let mut line_match_ranges = Vec::new();
                        let mut matches = true;

                        // Case insensitive search for line mode
                        for term in &terms {
                            let mut start = 0;
                            let mut term_matches = false;
                            let term_lower = term.to_lowercase();
                            let line_text_lower = line_text.to_lowercase();
                            while let Some(relative_idx) =
                                line_text_lower[start..].find(&term_lower)
                            {
                                let idx = start + relative_idx;
                                line_match_ranges.push(idx..idx + term.len());
                                start = idx + term.len();
                                term_matches = true;
                            }

                            if !term_matches {
                                matches = false;
                                break;
                            }
                        }

                        if matches {
                            line_match_ranges.sort_by_key(|r| r.start);

                            let start_match_index = all_match_ranges.len();
                            for range in &line_match_ranges {
                                let start = line_start_offset + range.start;
                                let end = line_start_offset + range.end;
                                all_match_ranges.push(
                                    buffer_snapshot.anchor_at(start, Bias::Left)
                                        ..buffer_snapshot.anchor_at(end, Bias::Right),
                                );
                            }
                            let end_match_index = all_match_ranges.len();

                            let preview_text = truncate_preview(&line_text, MAX_PREVIEW_BYTES);
                            let trimmed_line = line_text.trim();
                            let left_trimmed_len = line_text.len() - line_text.trim_start().len();
                            let preview_len = preview_content_len(&preview_text);

                            let mut list_match_ranges = Vec::new();
                            for range in &line_match_ranges {
                                let start_in_trimmed = range.start.saturating_sub(left_trimmed_len);
                                let end_in_trimmed = range.end.saturating_sub(left_trimmed_len);

                                if start_in_trimmed < trimmed_line.len() {
                                    let p_start = start_in_trimmed;
                                    let p_end = end_in_trimmed;

                                    if p_start < preview_len {
                                        let valid_p_end = p_end.min(preview_len);
                                        if p_start < valid_p_end {
                                            list_match_ranges.push(p_start..valid_p_end);
                                        }
                                    }
                                }
                            }

                            // Re-calculate syntax highlights as in text search
                            let syntax_highlights = {
                                let mut highlights = Vec::new();
                                let trimmed_line = line_text.trim();
                                let left_trimmed_len =
                                    line_text.len() - line_text.trim_start().len();
                                let mut current_offset: usize = 0;
                                for chunk in buffer_snapshot.chunks(
                                    line_start_offset..line_end_offset,
                                    LanguageAwareStyling {
                                        tree_sitter: true,
                                        diagnostics: false,
                                    },
                                ) {
                                    let chunk_len = chunk.text.len();
                                    if let Some(highlight_id) = chunk.syntax_highlight_id {
                                        let chunk_in_trimmed_start =
                                            current_offset.saturating_sub(left_trimmed_len);
                                        let chunk_in_trimmed_end = (current_offset + chunk_len)
                                            .saturating_sub(left_trimmed_len);
                                        if chunk_in_trimmed_start < trimmed_line.len() {
                                            let start =
                                                chunk_in_trimmed_start.max(0).min(preview_len);
                                            let end = chunk_in_trimmed_end
                                                .min(trimmed_line.len())
                                                .min(preview_len);
                                            if start < end {
                                                highlights.push((start..end, highlight_id));
                                            }
                                        }
                                    }
                                    current_offset += chunk_len;
                                }
                                if highlights.is_empty() {
                                    None
                                } else {
                                    Some(Arc::new(highlights))
                                }
                            };

                            let line_label = if let Some((_, buffer_point)) =
                                buffer_snapshot.point_to_buffer_point(Point::new(line, 0))
                            {
                                (buffer_point.row + 1).to_string().into()
                            } else {
                                (line + 1).to_string().into()
                            };

                            new_items.push(LineMatchData {
                                line_label,
                                preview_text,
                                list_match_ranges: Arc::new(list_match_ranges),
                                active_match_index_in_list: None,
                                syntax_highlights,
                                primary_match_offset: (line_start_offset
                                    + line_match_ranges.first().map(|r| r.start).unwrap_or(0))
                                .0,
                                match_indices: start_match_index..end_match_index,
                            });
                        }
                    }
                    (new_items, all_match_ranges)
                })
                .await;

            if cancelled.load(Ordering::Relaxed) {
                return;
            }

            let all_match_ranges = Arc::new(all_match_ranges);

            picker
                .update(cx, |picker, cx| {
                    picker.delegate.match_count = new_items.len();
                    picker.delegate.items = new_items;
                    picker.delegate.is_searching = false;
                    picker.delegate.all_matches = all_match_ranges.clone();

                    // Find closest match to initial cursor
                    let mut best_index = 0;
                    let mut min_distance = usize::MAX;

                    for (idx, item) in picker.delegate.items.iter().enumerate() {
                        let dist = item.primary_match_offset.abs_diff(initial_cursor);
                        if dist < min_distance {
                            min_distance = dist;
                            best_index = idx;
                        }
                    }

                    picker.delegate.selected_index = best_index;
                    let preview_data = picker.delegate.items.get(best_index).map(|item| {
                        (
                            item.primary_match_offset,
                            item.match_indices.clone(),
                            all_match_ranges,
                        )
                    });
                    if let Some(modal) = picker.delegate.buffer_search_modal.upgrade() {
                        window_handle
                            .update(cx, |_, window, cx| {
                                modal.update(cx, |modal, cx| {
                                    modal.update_preview(preview_data, window, cx)
                                })
                            })
                            .log_err();
                    }
                    cx.notify();
                })
                .log_err();
        })
    }

    fn render_match(&self, ix: usize, selected: bool, cx: &App) -> ListItem {
        let item = &self.items[ix];
        let preview_text = &item.preview_text;
        let line_label = &item.line_label;
        let list_match_ranges = &item.list_match_ranges;
        let syntax_highlights = &item.syntax_highlights;

        let preview_str: &str = preview_text.as_ref();

        let is_valid_range = |range: &Range<usize>| -> bool {
            range.start < range.end
                && range.end <= preview_str.len()
                && preview_str.is_char_boundary(range.start)
                && preview_str.is_char_boundary(range.end)
        };

        let syntax_theme = cx.theme().syntax();
        let mut match_highlights = Vec::new();

        if selected || self.line_mode {
            if self.line_mode {
                for range in list_match_ranges.iter() {
                    if is_valid_range(range) {
                        let match_style = HighlightStyle {
                            font_weight: Some(gpui::FontWeight::BOLD),
                            color: Some(cx.theme().colors().text_accent),
                            ..Default::default()
                        };
                        match_highlights.push((range.clone(), match_style));
                    }
                }
            } else if let Some(i) = item.active_match_index_in_list {
                if let Some(range) = list_match_ranges.get(i) {
                    if is_valid_range(range) {
                        let color = cx.theme().colors().search_active_match_background;
                        let match_style = HighlightStyle {
                            font_weight: Some(gpui::FontWeight::BOLD),
                            background_color: Some(color),
                            ..Default::default()
                        };
                        match_highlights.push((range.clone(), match_style));
                    }
                }
            }
        }

        let mut highlights: Vec<(Range<usize>, HighlightStyle)> = syntax_highlights
            .as_ref()
            .map(|sh| {
                sh.iter()
                    .filter_map(|(range, id)| {
                        if !is_valid_range(range) {
                            return None;
                        }
                        syntax_theme.get(*id).cloned().map(|style| (range.clone(), style))
                    })
                    .collect()
            })
            .unwrap_or_default();

        if !match_highlights.is_empty() {
            highlights = merge_highlights(&highlights, &match_highlights);
        }

        let label_color = if item.primary_match_offset < self.initial_cursor_offset {
            Color::Accent
        } else {
            Color::Muted
        };

        ListItem::new(ix).inset(true).toggle_state(selected).child(
            h_flex()
                .items_center()
                .w_full()
                .pl(px(8.))
                .justify_between()
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .text_ellipsis()
                        .text_ui_sm(cx)
                        .child(StyledText::new(preview_text).with_highlights(highlights)),
                )
                .child(
                    Label::new(line_label.clone())
                        .size(ui::LabelSize::Small)
                        .color(label_color),
                ),
        )
    }
}

impl PickerDelegate for BufferSearchDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.items.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn selected_index_changed(
        &self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Box<dyn Fn(&mut Window, &mut App) + 'static>> {
        let buffer_search_modal = self.buffer_search_modal.clone();

        let preview_data = if let Some(item) = self.items.get(ix) {
            Some((
                item.primary_match_offset,
                item.match_indices.clone(),
                self.all_matches.clone(),
            ))
        } else {
            None
        };

        Some(Box::new(move |window, cx| {
            let preview_data_clone = preview_data.clone();
            if let Some(modal) = buffer_search_modal.upgrade() {
                modal.update(cx, |modal, cx| {
                    modal.schedule_preview_update(preview_data_clone, window, cx);
                });
            }
        }))
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search in buffer...".into()
    }

    fn render_editor(
        &self,
        editor: &Arc<dyn ErasedEditor>,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        v_flex()
            .bg(cx.theme().colors().elevated_surface_background)
            .child(
                h_flex()
                    .overflow_hidden()
                    .flex_none()
                    .py_1()
                    .px_2()
                    .gap_1()
                    .child(
                        h_flex()
                            .flex_1()
                            .min_w_32()
                            .h_6()
                            .rounded_md()
                            .bg(cx.theme().colors().elevated_surface_background)
                            .child(editor.render(_window, cx))
                            .child(
                                h_flex()
                                    .gap_1()
                                    .when(self.is_searching, |this| {
                                        this.child(
                                            Icon::new(IconName::ArrowCircle)
                                                .size(IconSize::Small)
                                                .color(Color::Muted)
                                                .with_rotate_animation(2),
                                        )
                                    })
                                    .when(self.match_count > 0 && !self.is_searching, |this| {
                                        this.child(
                                            Label::new(format!(
                                                "{} / {}",
                                                self.selected_index + 1,
                                                self.match_count
                                            ))
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                        )
                                    })
                                    .child(
                                        Button::new("line-mode", "")
                                            .start_icon(
                                                Icon::new(IconName::ListFilter)
                                                    .size(IconSize::Small),
                                            )
                                            .style(ButtonStyle::Subtle)
                                            .toggle_state(self.line_mode)
                                            .key_binding(
                                                UiKeyBinding::for_action_in(
                                                    &ToggleLineMode,
                                                    self.focus_handle.as_ref().unwrap(),
                                                    cx,
                                                )
                                                .map(|kb| kb.size(rems_from_px(12.))),
                                            )
                                            .on_click(cx.listener(move |picker, _, window, cx| {
                                                picker.delegate.line_mode =
                                                    !picker.delegate.line_mode;
                                                let query = picker.delegate.current_query.clone();
                                                picker.set_query(&query, window, cx);
                                            }))
                                            .tooltip(|window, cx| {
                                                Tooltip::text("Toggle Line Mode")(window, cx)
                                            }),
                                    ),
                            ),
                    ),
            )
            .child(Divider::horizontal())
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.current_query = query.clone();
        let window_handle = window.window_handle();

        if let Some(prev_cancelled) = self.search_cancelled.take() {
            prev_cancelled.store(true, Ordering::Relaxed);
        }
        let cancelled = Arc::new(AtomicBool::new(false));
        self.search_cancelled = Some(cancelled.clone());

        let initial_cursor = self.initial_cursor_offset;
        let buffer_snapshot = self.target_buffer.read(cx).snapshot(cx);

        self.is_searching = true;

        if self.line_mode && !query.is_empty() {
            return self.spawn_line_search(
                query,
                initial_cursor,
                buffer_snapshot,
                cancelled,
                window,
                cx,
            );
        }

        if query.is_empty() {
            // Populate with all lines
            return cx.spawn(async move |picker, cx| {
                if cancelled.load(Ordering::Relaxed) {
                    return;
                }

                let line_count = buffer_snapshot.max_point().row + 1;
                let cancelled_clone = cancelled.clone();
                let buffer_snapshot_clone = buffer_snapshot.clone();

                let new_items = cx
                    .background_executor()
                    .spawn(async move {
                        let mut new_items = Vec::with_capacity(line_count as usize);

                        for line in 0..line_count {
                            if cancelled_clone.load(Ordering::Relaxed) {
                                return new_items;
                            }

                            let line_start =
                                buffer_snapshot_clone.point_to_offset(Point::new(line, 0));
                            let line_end = if line < buffer_snapshot_clone.max_point().row {
                                buffer_snapshot_clone.point_to_offset(Point::new(line + 1, 0))
                            } else {
                                buffer_snapshot_clone.len()
                            };

                            let line_text: String = buffer_snapshot_clone
                                .text_for_range(line_start..line_end)
                                .collect();

                            let preview_text = truncate_preview(&line_text, MAX_PREVIEW_BYTES);
                            let preview_len = preview_content_len(&preview_text);
                            let preview_str: &str = preview_text.as_ref();

                            // For syntax highlighting, map original line chunks to preview text coordinates
                            let syntax_highlights = {
                                let mut highlights = Vec::new();

                                // Since truncate_preview operates on trimmed text (line_text.trim()),
                                // we need to map the syntax highlighting chunks accordingly
                                let trimmed_line = line_text.trim();
                                let left_trimmed_len =
                                    line_text.len() - line_text.trim_start().len();

                                let mut current_offset = 0;
                                for chunk in buffer_snapshot_clone.chunks(
                                    line_start..line_end,
                                    LanguageAwareStyling {
                                        tree_sitter: true,
                                        diagnostics: false,
                                    },
                                ) {
                                    let chunk_len = chunk.text.len();
                                    if let Some(highlight_id) = chunk.syntax_highlight_id {
                                        let chunk_absolute_start = current_offset;
                                        let chunk_absolute_end = current_offset + chunk_len;

                                        // Map to trimmed line coordinates
                                        let chunk_in_trimmed_start =
                                            chunk_absolute_start.saturating_sub(left_trimmed_len);
                                        let chunk_in_trimmed_end =
                                            chunk_absolute_end.saturating_sub(left_trimmed_len);

                                        // Only highlight if within the trimmed content range
                                        if chunk_in_trimmed_start < trimmed_line.len() {
                                            let start_in_preview = chunk_in_trimmed_start.max(0);
                                            let end_in_preview =
                                                chunk_in_trimmed_end.min(trimmed_line.len());

                                            if start_in_preview < end_in_preview
                                                && start_in_preview < preview_len
                                            {
                                                let clamped_start =
                                                    start_in_preview.min(preview_len);
                                                let clamped_end = end_in_preview.min(preview_len);

                                                if let Some((safe_start, safe_end)) =
                                                    find_safe_char_boundaries(
                                                        preview_str,
                                                        clamped_start,
                                                        clamped_end,
                                                    )
                                                {
                                                    highlights
                                                        .push((safe_start..safe_end, highlight_id));
                                                }
                                            }
                                        }
                                    }
                                    current_offset += chunk_len;
                                }

                                if highlights.is_empty() {
                                    None
                                } else {
                                    Some(Arc::new(highlights))
                                }
                            };

                            let line_label = if let Some((_, buffer_point)) =
                                buffer_snapshot_clone.point_to_buffer_point(Point::new(line, 0))
                            {
                                (buffer_point.row + 1).to_string().into()
                            } else {
                                (line + 1).to_string().into()
                            };

                            new_items.push(LineMatchData {
                                line_label,
                                preview_text,
                                list_match_ranges: Arc::new(Vec::new()),
                                active_match_index_in_list: None,
                                syntax_highlights,
                                primary_match_offset: line_start.0,
                                match_indices: 0..0,
                            });
                        }
                        new_items
                    })
                    .await;

                if cancelled.load(Ordering::Relaxed) {
                    return;
                }

                picker
                    .update(cx, |picker, cx| {
                        if cancelled.load(Ordering::Relaxed) {
                            return;
                        }

                        picker.delegate.match_count = new_items.len();
                        picker.delegate.items = new_items;
                        picker.delegate.is_searching = false;
                        picker.delegate.all_matches = Arc::new(Vec::new());

                        // Set selected index to cursor line
                        let cursor_line = buffer_snapshot
                            .offset_to_point(MultiBufferOffset(initial_cursor))
                            .row;
                        picker.delegate.selected_index =
                            cursor_line.min(line_count.saturating_sub(1)) as usize;

                        let selected_index = picker.delegate.selected_index;
                        let buffer_search_modal = picker.delegate.buffer_search_modal.clone();
                        let preview_data = picker.delegate.items.get(selected_index).map(|item| {
                            (
                                item.primary_match_offset,
                                item.match_indices.clone(),
                                picker.delegate.all_matches.clone(),
                            )
                        });

                        if let Some(modal) = buffer_search_modal.upgrade() {
                            window_handle
                                .update(cx, |_, window, cx| {
                                    modal.update(cx, |modal, cx| {
                                        modal.update_preview(preview_data, window, cx);
                                    });
                                })
                                .log_err();
                        }

                        cx.notify();
                    })
                    .log_err();
            });
        }

        cx.spawn(async move |picker, cx| {
            if cancelled.load(Ordering::Relaxed) {
                return;
            }

            let search_query = match build_search_query(&query) {
                Ok(q) => {
                    picker
                        .update(cx, |picker, cx| {
                            picker.delegate.regex_error = None;
                            cx.notify();
                        })
                        .log_err();
                    q
                }
                Err(error_message) => {
                    picker
                        .update(cx, |picker, cx| {
                            picker.delegate.regex_error = Some(error_message);
                            picker.delegate.items.clear();
                            picker.delegate.match_count = 0;
                            picker.delegate.is_searching = false;
                            cx.notify();
                        })
                        .log_err();
                    return;
                }
            };

            let cancelled_clone = cancelled.clone();

            let (all_match_ranges, new_items) = cx
                .background_executor()
                .spawn(async move {
                    let mut ranges = Vec::new();

                    // Search the entire multi-buffer range
                    let full_range = buffer_snapshot.anchor_before(MultiBufferOffset(0))
                        ..buffer_snapshot.anchor_after(buffer_snapshot.len());

                    // Break down multi-buffer into individual buffer ranges, following Editor::find_matches pattern
                    for (search_buffer, search_range, deleted_hunk_anchor) in
                        buffer_snapshot.range_to_buffer_ranges_with_deleted_hunks(full_range)
                    {
                        // Perform search on this buffer segment
                        let buffer_matches = search_query
                            .search(
                                search_buffer,
                                Some(search_range.start.0..search_range.end.0),
                            )
                            .await;

                        // Convert buffer-relative matches to multi-buffer anchor ranges
                        ranges.extend(buffer_matches.into_iter().filter_map(|match_range| {
                            if let Some(deleted_hunk_anchor) = deleted_hunk_anchor {
                                let start = search_buffer
                                    .anchor_after(search_range.start + match_range.start);
                                let end = search_buffer
                                    .anchor_before(search_range.start + match_range.end);
                                Some(
                                    deleted_hunk_anchor.with_diff_base_anchor(start)
                                        ..deleted_hunk_anchor.with_diff_base_anchor(end),
                                )
                            } else {
                                let start = search_buffer
                                    .anchor_after(search_range.start + match_range.start);
                                let end = search_buffer
                                    .anchor_before(search_range.start + match_range.end);
                                Some(
                                    buffer_snapshot.anchor_in_buffer(start)?
                                        ..buffer_snapshot.anchor_in_buffer(end)?,
                                )
                            }
                        }));
                    }

                    if cancelled_clone.load(Ordering::Relaxed) {
                        return (Vec::new(), Vec::new());
                    }

                    // Group matches by line to compute preview text once per line
                    let mut lines_data: HashMap<u32, Vec<Range<MultiBufferOffset>>> =
                        HashMap::default();
                    for range in ranges.iter() {
                        let start_offset = range.start.to_offset(&buffer_snapshot);
                        let start_point = buffer_snapshot.offset_to_point(start_offset);
                        let end_offset = range.end.to_offset(&buffer_snapshot);
                        lines_data
                            .entry(start_point.row)
                            .or_default()
                            .push(start_offset..end_offset);
                    }

                    let mut new_items: Vec<LineMatchData> = Vec::with_capacity(ranges.len());
                    let mut sorted_lines: Vec<u32> = lines_data.keys().cloned().collect();
                    sorted_lines.sort();

                    let mut match_indices_counter = 0;

                    for line in sorted_lines {
                        if cancelled_clone.load(Ordering::Relaxed) {
                            return (Vec::new(), Vec::new());
                        }

                        if let Some(ranges) = lines_data.remove(&line) {
                            let line_start = buffer_snapshot.point_to_offset(Point::new(line, 0));
                            let line_end = if line < buffer_snapshot.max_point().row {
                                buffer_snapshot.point_to_offset(Point::new(line + 1, 0))
                            } else {
                                buffer_snapshot.len()
                            };
                            let line_text: String = buffer_snapshot
                                .text_for_range(line_start..line_end)
                                .collect();

                            let trim_start = line_text.len() - line_text.trim_start().len();

                            // Create an item for each match with its own preview text centered around the match
                            for (i, range) in ranges.iter().enumerate() {
                                let rel_match_start = range.start.0.saturating_sub(line_start.0);

                                let (p_start, p_end) = {
                                    let match_len = range.end.0 - range.start.0;
                                    let context = (MAX_PREVIEW_BYTES.saturating_sub(match_len)) / 2;
                                    let mut start = rel_match_start.saturating_sub(context);

                                    if start < trim_start {
                                        start = trim_start;
                                    }

                                    if rel_match_start < start {
                                        start = rel_match_start;
                                    }

                                    let end = (start + MAX_PREVIEW_BYTES).min(line_text.len());
                                    (start, end)
                                };
                                let (p_start, p_end) =
                                    find_safe_char_boundaries(&line_text, p_start, p_end)
                                        .unwrap_or((p_start, p_end));

                                let mut preview_string = String::new();
                                if p_start > trim_start {
                                    preview_string.push('…');
                                }
                                preview_string.push_str(line_text[p_start..p_end].trim_end());
                                if p_end < line_text.trim_end().len() {
                                    preview_string.push('…');
                                }
                                let preview_text: SharedString = preview_string.into();
                                let prefix_len = if p_start > trim_start {
                                    '…'.len_utf8()
                                } else {
                                    0
                                };

                                let mut list_match_ranges = Vec::new();
                                let mut active_match_index_in_list = None;

                                for (j, other_range) in ranges.iter().enumerate() {
                                    let other_rel_start =
                                        other_range.start.0.saturating_sub(line_start.0);
                                    let other_rel_end = other_range
                                        .end
                                        .0
                                        .saturating_sub(line_start.0)
                                        .min(line_text.len());

                                    let start = other_rel_start.max(p_start);
                                    let end = other_rel_end.min(p_end);
                                    if start < end {
                                        let rel_start = (start - p_start) + prefix_len;
                                        let rel_end = (end - p_start) + prefix_len;
                                        list_match_ranges.push(rel_start..rel_end);
                                        if i == j {
                                            active_match_index_in_list =
                                                Some(list_match_ranges.len() - 1);
                                        }
                                    }
                                }

                                let mut item_syntax = Vec::new();
                                let chunk_offset_start = line_start.0 + p_start;
                                let chunk_offset_end = line_start.0 + p_end;
                                let mut current_rel_offset = prefix_len;

                                for chunk in buffer_snapshot.chunks(
                                    MultiBufferOffset(chunk_offset_start)
                                        ..MultiBufferOffset(chunk_offset_end),
                                    LanguageAwareStyling {
                                        tree_sitter: true,
                                        diagnostics: false,
                                    },
                                ) {
                                    let len = chunk.text.len();
                                    if let Some(id) = chunk.syntax_highlight_id {
                                        item_syntax.push((
                                            current_rel_offset..current_rel_offset + len,
                                            id,
                                        ));
                                    }
                                    current_rel_offset += len;
                                }
                                let syntax_highlights = if item_syntax.is_empty() {
                                    None
                                } else {
                                    Some(Arc::new(item_syntax))
                                };

                                let line_label = if let Some((_, buffer_point)) =
                                    buffer_snapshot.point_to_buffer_point(Point::new(line, 0))
                                {
                                    (buffer_point.row + 1).to_string().into()
                                } else {
                                    (line + 1).to_string().into()
                                };

                                new_items.push(LineMatchData {
                                    line_label,
                                    preview_text,
                                    list_match_ranges: Arc::new(list_match_ranges),
                                    active_match_index_in_list,
                                    syntax_highlights,
                                    primary_match_offset: range.start.0,
                                    match_indices: match_indices_counter
                                        ..(match_indices_counter + 1),
                                });
                                match_indices_counter += 1;
                            }
                        }
                    }
                    (ranges, new_items)
                })
                .await;

            if cancelled.load(Ordering::Relaxed) {
                return;
            }

            let all_match_ranges = Arc::new(all_match_ranges);

            picker
                .update(cx, |picker, cx| {
                    if cancelled.load(Ordering::Relaxed) {
                        return;
                    }

                    picker.delegate.match_count = all_match_ranges.len();
                    picker.delegate.items = new_items;
                    picker.delegate.is_searching = false;
                    picker.delegate.all_matches = all_match_ranges.clone();

                    // Find closest match to initial cursor
                    // For search results, we look at primary_match_offset (absolute offset in buffer)
                    let mut best_index = 0;
                    let mut min_distance = usize::MAX;

                    for (idx, item) in picker.delegate.items.iter().enumerate() {
                        let dist = item.primary_match_offset.abs_diff(initial_cursor);

                        if dist < min_distance {
                            min_distance = dist;
                            best_index = idx;
                        }
                    }

                    picker.delegate.selected_index = best_index;

                    let buffer_search_modal = picker.delegate.buffer_search_modal.clone();
                    let preview_data = picker.delegate.items.get(best_index).map(|item| {
                        (
                            item.primary_match_offset,
                            item.match_indices.clone(),
                            picker.delegate.all_matches.clone(),
                        )
                    });

                    if let Some(modal) = buffer_search_modal.upgrade() {
                        window_handle
                            .update(cx, |_, window, cx| {
                                modal.update(cx, |modal, cx| {
                                    modal.update_preview(preview_data, window, cx);
                                });
                            })
                            .log_err();
                    }

                    cx.notify();
                })
                .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let query = self.current_query.clone();
        if !query.is_empty() {
            BufferSearchHistory::update_global(cx, |history, _| {
                history.0.add(&mut self.search_history_cursor, query);
            });
        }
        if let Some(item) = self.items.get(self.selected_index) {
            let target_editor = self.target_editor.clone();
            let match_offset = item.primary_match_offset;
            let match_indices = item.match_indices.clone();
            let all_matches = self.all_matches.clone();

            // Dismiss modal
            cx.emit(DismissEvent);

            // Move cursor in actual editor using the same approach as Editor::activate_match
            target_editor.update(cx, |editor, cx| {
                let buffer_snapshot = editor.buffer().read(cx).snapshot(cx);

                // Get the specific match range for this item
                let match_range = if !all_matches.is_empty() && !match_indices.is_empty() {
                    // Use the first match in the range for this line item
                    let match_index = match_indices.start;
                    if match_index < all_matches.len() {
                        Some(all_matches[match_index].clone())
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some(range) = match_range {
                    // Unfold the range if it's folded
                    editor.unfold_ranges(std::slice::from_ref(&range), false, true, cx);

                    // Convert anchor range to selection range
                    let selection_range = editor.range_for_match(&range);

                    // Use proper autoscroll behavior
                    let autoscroll = if EditorSettings::get_global(cx).search.center_on_match {
                        Autoscroll::center()
                    } else {
                        Autoscroll::fit()
                    };

                    // Change selections with proper effects
                    editor.change_selections(
                        SelectionEffects::scroll(autoscroll).nav_history(true),
                        window,
                        cx,
                        |s| s.select_ranges([selection_range]),
                    );
                } else {
                    // Fallback to offset-based positioning if no proper range is available
                    let offset = MultiBufferOffset(match_offset);
                    let anchor = buffer_snapshot.anchor_before(offset);
                    let autoscroll = if EditorSettings::get_global(cx).search.center_on_match {
                        Autoscroll::center()
                    } else {
                        Autoscroll::fit()
                    };

                    editor.change_selections(
                        SelectionEffects::scroll(autoscroll).nav_history(true),
                        window,
                        cx,
                        |s| s.select_anchor_ranges([anchor..anchor]),
                    );
                }
            });
        } else {
            cx.emit(DismissEvent);
        }
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        Some(self.render_match(ix, selected, cx))
    }

    fn render_header(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        if let Some(error) = &self.regex_error {
            return Some(
                h_flex()
                    .w_full()
                    .px_3()
                    .py_1()
                    .child(
                        Label::new(format!("Invalid regex: {}", error))
                            .size(LabelSize::Small)
                            .color(Color::Error),
                    )
                    .into_any(),
            );
        }
        None
    }
}
