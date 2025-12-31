use collections::{HashMap, HashSet};
use editor::{Anchor as MultiBufferAnchor, Editor, EditorEvent};
use file_icons::FileIcons;
use futures::StreamExt;
use gpui::{
    Action, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    HighlightStyle, Pixels, Render, SharedString, StyledText, Subscription, Task, WeakEntity,
    Window, actions, prelude::*,
};
use language::{Buffer, BufferEvent, HighlightId};
use picker::{Picker, PickerDelegate};
use project::{Project, ProjectPath, search::SearchQuery};
use std::{path::Path, pin::pin, sync::Arc, time::Duration};
use text::{ToOffset as _, ToPoint as _};
use ui::{
    Button, ButtonStyle, Color, Divider, Icon, IconButton, IconButtonShape, IconName, KeyBinding,
    Label, ListItem, ListItemSpacing, Tooltip, prelude::*, rems_from_px,
};
use util::{ResultExt, paths::PathMatcher};
use workspace::{
    Item, ModalView, Save, Workspace, item::SaveOptions, searchable::SearchableItemHandle,
};

use crate::{
    SearchOption, SearchOptions, ToggleCaseSensitive, ToggleIncludeIgnored, ToggleRegex,
    ToggleWholeWord,
};

type AnchorRange = std::ops::Range<text::Anchor>;

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

struct BufferExtractData {
    worktree_id: project::WorktreeId,
    path: Arc<util::rel_path::RelPath>,
    snapshot: language::BufferSnapshot,
    ranges: Vec<AnchorRange>,
}

fn extract_buffer_data(
    buf: &Buffer,
    ranges: Vec<AnchorRange>,
    cx: &App,
) -> Option<BufferExtractData> {
    let file = buf.file()?;
    Some(BufferExtractData {
        worktree_id: file.worktree_id(cx),
        path: file.path().clone(),
        snapshot: buf.snapshot(),
        ranges,
    })
}

struct LineInfo {
    line_start_offset: usize,
    line_text: String,
    trim_start: usize,
    match_ranges: Vec<AnchorRange>,
}

fn group_ranges_by_line(
    ranges: &[AnchorRange],
    snapshot: &language::BufferSnapshot,
) -> (HashMap<u32, LineInfo>, Vec<u32>) {
    let estimated_lines = ranges.len().min(MAX_LINES_PER_FILE);
    let mut lines_data: HashMap<u32, LineInfo> = HashMap::default();
    lines_data.reserve(estimated_lines);
    let mut line_order = Vec::with_capacity(estimated_lines);

    for range in ranges {
        let start_point = range.start.to_point(snapshot);
        let line = start_point.row;

        if let Some(line_data) = lines_data.get_mut(&line) {
            line_data.match_ranges.push(range.clone());
        } else {
            let line_start = snapshot.point_to_offset(text::Point::new(line, 0));
            let line_end_col = snapshot.line_len(line);
            let line_end = snapshot.point_to_offset(text::Point::new(line, line_end_col));

            let line_text: String = snapshot.text_for_range(line_start..line_end).collect();
            let trim_start = line_text.len() - line_text.trim_start().len();

            lines_data.insert(
                line,
                LineInfo {
                    line_start_offset: line_start,
                    line_text,
                    trim_start,
                    match_ranges: vec![range.clone()],
                },
            );
            line_order.push(line);
        }

        if line_order.len() >= MAX_LINES_PER_FILE {
            break;
        }
    }

    (lines_data, line_order)
}

fn create_line_match_data(
    line: u32,
    info: LineInfo,
    snapshot: &language::BufferSnapshot,
    project_path: &ProjectPath,
    file_key: &SharedString,
) -> LineMatchData {
    let preview_text = truncate_preview(&info.line_text, MAX_PREVIEW_BYTES);
    let preview_len = preview_content_len(&preview_text);
    let line_label: SharedString = (line + 1).to_string().into();

    let mut match_positions = Vec::new();
    let preview_str: &str = preview_text.as_ref();
    for range in &info.match_ranges {
        let match_start_offset = range.start.to_offset(snapshot);
        let match_end_offset = range.end.to_offset(snapshot);

        let start_in_line = match_start_offset.saturating_sub(info.line_start_offset);
        let end_in_line = match_end_offset.saturating_sub(info.line_start_offset);

        let start_in_preview = start_in_line.saturating_sub(info.trim_start);
        let end_in_preview = end_in_line.saturating_sub(info.trim_start);

        if start_in_preview < preview_len && end_in_preview > 0 {
            let clamped_start = start_in_preview.min(preview_len);
            let clamped_end = end_in_preview.min(preview_len);
            if let Some((safe_start, safe_end)) =
                find_safe_char_boundaries(preview_str, clamped_start, clamped_end)
            {
                match_positions.push(safe_start..safe_end);
            }
        }
    }

    LineMatchData {
        project_path: project_path.clone(),
        file_key: file_key.clone(),
        line,
        line_label,
        preview_text,
        match_ranges: Arc::new(info.match_ranges),
        match_positions: Arc::new(match_positions),
        trim_start: info.trim_start,
        syntax_highlights: None,
    }
}

fn process_file_matches(data: BufferExtractData) -> Option<FileMatchResult> {
    let project_path = ProjectPath {
        worktree_id: data.worktree_id,
        path: data.path.clone(),
    };

    let (file_name, parent_path) = extract_path_parts(&data.path);
    let file_key = format_file_key(&parent_path, &file_name);
    let snapshot = &data.snapshot;

    let (mut lines_data, line_order) = group_ranges_by_line(&data.ranges, snapshot);

    if line_order.is_empty() {
        return None;
    }

    let matches = line_order
        .into_iter()
        .filter_map(|line| {
            let info = lines_data.remove(&line)?;
            Some(create_line_match_data(
                line,
                info,
                snapshot,
                &project_path,
                &file_key,
            ))
        })
        .collect();

    Some(FileMatchResult {
        file_name,
        parent_path,
        file_key,
        matches,
    })
}

const MIN_WIDTH_FOR_HORIZONTAL_LAYOUT: Pixels = px(950.);
const LEFT_PANEL_RATIO: f32 = 0.30;
const VERTICAL_RESULTS_RATIO: f32 = 0.40;
const MAX_PREVIEW_BYTES: usize = 200;
const PREVIEW_DEBOUNCE_MS: u64 = 50;
const EDIT_OPEN_DELAY_MS: u64 = 200;
const STREAM_CHUNK_SIZE: usize = 64;
const FIRST_BATCH_THRESHOLD: usize = 16;
const BACKGROUND_BATCH_THRESHOLD: usize = 128;
const MAX_LINES_PER_FILE: usize = 800;
const MAX_SEARCH_RESULT_FILES: usize = 5_000;
const MAX_SEARCH_RESULT_RANGES: usize = 10_000;

fn compute_search_debounce_ms(file_count: usize) -> u64 {
    match file_count {
        0..100 => 0,
        100..1_000 => 50,
        1_000..10_000 => 100,
        10_000..50_000 => 150,
        _ => 200,
    }
}

fn get_project_file_count(project: &Project, cx: &App) -> usize {
    project
        .worktrees(cx)
        .map(|worktree| worktree.read(cx).snapshot().file_count())
        .sum()
}

actions!(search, [QuickSearch]);

struct LineMatchData {
    project_path: ProjectPath,
    file_key: SharedString,
    line: u32,
    line_label: SharedString,
    preview_text: SharedString,
    match_ranges: Arc<Vec<AnchorRange>>,
    match_positions: Arc<Vec<std::ops::Range<usize>>>,
    trim_start: usize,
    syntax_highlights: Option<Arc<Vec<(std::ops::Range<usize>, HighlightId)>>>,
}

enum QuickSearchHighlights {}

struct FileMatchResult {
    file_name: SharedString,
    parent_path: SharedString,
    file_key: SharedString,
    matches: Vec<LineMatchData>,
}

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

fn extract_syntax_highlights_for_line(
    snapshot: &language::BufferSnapshot,
    preview_text: &str,
    line: u32,
    trim_start: usize,
) -> Option<Vec<(std::ops::Range<usize>, HighlightId)>> {
    let preview_len = preview_content_len(preview_text);
    if preview_len == 0 {
        return None;
    }

    let line_start = snapshot.point_to_offset(text::Point::new(line, 0));
    let line_len = snapshot.line_len(line);
    let line_end = snapshot.point_to_offset(text::Point::new(line, line_len));

    let mut highlights = Vec::new();
    let mut current_offset = 0;

    for chunk in snapshot.chunks(line_start..line_end, true) {
        let chunk_len = chunk.text.len();

        if let Some(highlight_id) = chunk.syntax_highlight_id {
            let abs_start = current_offset;
            let abs_end = current_offset + chunk_len;

            let rel_start = abs_start.saturating_sub(trim_start);
            let rel_end = abs_end.saturating_sub(trim_start);

            if rel_end > 0 && rel_start < preview_len {
                let clamped_start = rel_start.min(preview_len);
                let clamped_end = rel_end.min(preview_len);

                if let Some((safe_start, safe_end)) =
                    find_safe_char_boundaries(preview_text, clamped_start, clamped_end)
                {
                    highlights.push((safe_start..safe_end, highlight_id));
                }
            }
        }

        current_offset += chunk_len;
    }

    if highlights.is_empty() {
        None
    } else {
        Some(highlights)
    }
}

fn format_file_key(parent_path: &str, file_name: &str) -> SharedString {
    if parent_path.is_empty() {
        file_name.to_string().into()
    } else {
        format!("{}/{}", parent_path, file_name).into()
    }
}

fn extract_path_parts(path: &Arc<util::rel_path::RelPath>) -> (SharedString, SharedString) {
    let file_name: SharedString = path
        .file_name()
        .map(|n| n.to_string())
        .unwrap_or_default()
        .into();
    let parent_path: SharedString = path
        .parent()
        .map(|p| p.as_unix_str().to_string())
        .unwrap_or_default()
        .into();
    (file_name, parent_path)
}

pub fn init(cx: &mut App) {
    cx.observe_new(QuickSearchModal::register).detach();
}

enum QuickSearchItem {
    FileHeader {
        file_name: SharedString,
        parent_path: SharedString,
        file_key: SharedString,
    },
    LineMatch(LineMatchData),
}

pub struct QuickSearchDelegate {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    search_options: SearchOptions,
    items: Vec<QuickSearchItem>,
    visible_indices: Vec<usize>,
    visible_line_match_indices: Vec<usize>,
    collapsed_files: HashSet<SharedString>,
    selected_index: usize,
    quick_search: WeakEntity<QuickSearchModal>,
    match_count: usize,
    file_count: usize,
    is_limited: bool,
    is_searching: bool,
    current_query: String,
    focus_handle: Option<FocusHandle>,
    regex_error: Option<String>,
    buffer_cache: HashMap<ProjectPath, Entity<Buffer>>,
    buffer_subscriptions: HashMap<ProjectPath, Subscription>,
}

pub struct QuickSearchModal {
    picker: Entity<Picker<QuickSearchDelegate>>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    preview_editor: Option<Entity<Editor>>,
    preview_buffer: Option<Entity<Buffer>>,
    preview_pending_path: Option<ProjectPath>,
    preview_opened_in_workspace: Option<ProjectPath>,
    pending_preview_data: Option<(ProjectPath, u32, Arc<Vec<AnchorRange>>)>,
    _picker_subscription: Subscription,
    _preview_editor_subscription: Option<Subscription>,
    _open_in_workspace_task: Option<Task<()>>,
    _preview_debounce_task: Option<Task<()>>,
}

impl ModalView for QuickSearchModal {}

impl EventEmitter<DismissEvent> for QuickSearchModal {}

impl Focusable for QuickSearchModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for QuickSearchModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let preview_editor = self.preview_editor.clone();
        let picker = self.picker.clone();
        let project = self.project.clone();

        let viewport_size = window.viewport_size();
        let use_vertical_layout = viewport_size.width < MIN_WIDTH_FOR_HORIZONTAL_LAYOUT;

        let modal_width = (viewport_size.width * 0.9).min(viewport_size.width);
        let modal_height = (viewport_size.height * 0.8).min(viewport_size.height);

        let border_color = cx.theme().colors().border;

        let results_panel = v_flex()
            .flex_shrink_0()
            .min_h_0()
            .overflow_hidden()
            .child(self.picker.clone());

        let save_preview_editor = preview_editor.clone();
        let preview_panel = v_flex()
            .id("quick-search-preview")
            .relative()
            .flex_1()
            .overflow_hidden()
            .bg(cx.theme().colors().editor_background)
            .on_click(move |_, window, cx| {
                window.focus(&picker.focus_handle(cx), cx);
            })
            .on_action({
                move |_: &Save, window, cx| {
                    if let Some(editor) = save_preview_editor.clone() {
                        editor.update(cx, |editor, cx| {
                            editor
                                .save(SaveOptions::default(), project.clone(), window, cx)
                                .detach_and_log_err(cx);
                        });
                    }
                }
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

        let content = if use_vertical_layout {
            let results_height = modal_height * VERTICAL_RESULTS_RATIO;
            v_flex()
                .w_full()
                .flex_1()
                .min_h_0()
                .overflow_hidden()
                .child(
                    results_panel
                        .h(results_height)
                        .w_full()
                        .border_b_1()
                        .border_color(border_color),
                )
                .child(preview_panel.w_full())
                .into_any_element()
        } else {
            let left_panel_width = modal_width * LEFT_PANEL_RATIO;
            h_flex()
                .w_full()
                .flex_1()
                .min_h_0()
                .overflow_hidden()
                .child(
                    results_panel
                        .w(left_panel_width)
                        .h_full()
                        .border_r_1()
                        .border_color(border_color),
                )
                .child(preview_panel.h_full())
                .into_any_element()
        };

        div()
            .id("quick-search-modal")
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
                    .child(content),
            )
    }
}

impl QuickSearchModal {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _cx: &mut Context<Workspace>,
    ) {
        workspace.register_action(|workspace, _: &QuickSearch, window, cx| {
            let selected_text = workspace
                .active_item(cx)
                .and_then(|item| item.act_as::<Editor>(cx))
                .map(|editor| editor.query_suggestion(window, cx))
                .filter(|query| !query.is_empty());

            let project = workspace.project().clone();
            let weak_workspace = cx.entity().downgrade();
            workspace.toggle_modal(window, cx, |window, cx| {
                QuickSearchModal::new(weak_workspace, project, selected_text, window, cx)
            });
        });
        workspace.register_action(Self::toggle_case_sensitive);
        workspace.register_action(Self::toggle_whole_word);
        workspace.register_action(Self::toggle_regex);
        workspace.register_action(Self::toggle_include_ignored);
    }

    fn toggle_search_option(
        workspace: &mut Workspace,
        option: SearchOptions,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if let Some(modal) = workspace.active_modal::<Self>(cx) {
            modal.update(cx, |modal, cx| {
                modal.picker.update(cx, |picker, cx| {
                    picker.delegate.toggle_search_option(option);
                    let query = picker.delegate.current_query.clone();
                    picker.set_query(query, window, cx);
                });
            });
        }
    }

    fn toggle_case_sensitive(
        workspace: &mut Workspace,
        _: &ToggleCaseSensitive,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        Self::toggle_search_option(workspace, SearchOptions::CASE_SENSITIVE, window, cx);
    }

    fn toggle_whole_word(
        workspace: &mut Workspace,
        _: &ToggleWholeWord,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        Self::toggle_search_option(workspace, SearchOptions::WHOLE_WORD, window, cx);
    }

    fn toggle_regex(
        workspace: &mut Workspace,
        _: &ToggleRegex,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        Self::toggle_search_option(workspace, SearchOptions::REGEX, window, cx);
    }

    fn toggle_include_ignored(
        workspace: &mut Workspace,
        _: &ToggleIncludeIgnored,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        Self::toggle_search_option(workspace, SearchOptions::INCLUDE_IGNORED, window, cx);
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        initial_query: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let weak_self = cx.entity().downgrade();

        let delegate = QuickSearchDelegate {
            workspace: workspace.clone(),
            project: project.clone(),
            search_options: SearchOptions::NONE,
            items: Vec::new(),
            visible_indices: Vec::new(),
            visible_line_match_indices: Vec::new(),
            collapsed_files: HashSet::default(),
            selected_index: 0,
            quick_search: weak_self,
            match_count: 0,
            file_count: 0,
            is_limited: false,
            is_searching: false,
            current_query: initial_query.clone().unwrap_or_default(),
            focus_handle: None,
            regex_error: None,
            buffer_cache: HashMap::default(),
            buffer_subscriptions: HashMap::default(),
        };

        let picker = cx.new(|cx| {
            let mut picker = Picker::uniform_list(delegate, window, cx)
                .modal(false)
                .max_height(None)
                .show_scrollbar(true);
            picker.delegate.focus_handle = Some(picker.focus_handle(cx));
            if let Some(q) = initial_query {
                picker.set_query(q, window, cx);
            }
            picker
        });

        let picker_subscription = cx.subscribe_in(&picker, window, Self::on_picker_event);

        Self {
            picker,
            workspace,
            project,
            preview_editor: None,
            preview_buffer: None,
            preview_pending_path: None,
            preview_opened_in_workspace: None,
            pending_preview_data: None,
            _picker_subscription: picker_subscription,
            _preview_editor_subscription: None,
            _open_in_workspace_task: None,
            _preview_debounce_task: None,
        }
    }

    fn on_picker_event(
        &mut self,
        _picker: &Entity<Picker<QuickSearchDelegate>>,
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !matches!(event, EditorEvent::Edited { .. }) {
            return;
        }

        if self.preview_opened_in_workspace.is_some() {
            return;
        }

        let Some(buffer) = &self.preview_buffer else {
            return;
        };

        let Some(file) = buffer.read(cx).file() else {
            return;
        };

        let project_path = ProjectPath {
            worktree_id: file.worktree_id(cx),
            path: file.path().clone(),
        };

        let Some(preview_editor) = self.preview_editor.clone() else {
            return;
        };

        // Delay before opening the file in the workspace to handle focus transitions:
        // when the file opens in the workspace, it steals focus from the Quick Search.
        // After opening, we restore focus to the preview editor so the user can continue editing.
        self._open_in_workspace_task = Some(cx.spawn_in(window, async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(EDIT_OPEN_DELAY_MS))
                .await;

            this.update_in(cx, |this, window, cx| {
                if this.preview_opened_in_workspace.is_some() {
                    return;
                }

                this.preview_opened_in_workspace = Some(project_path.clone());

                let Some(workspace) = this.workspace.upgrade() else {
                    return;
                };

                let open_task = workspace.update(cx, |workspace, cx| {
                    workspace.open_path_preview(project_path, None, false, false, false, window, cx)
                });

                cx.spawn_in(window, async move |_, cx| {
                    let _ = open_task.await;
                    cx.update(|window, cx| {
                        window.focus(&preview_editor.focus_handle(cx), cx);
                    })
                    .log_err();
                })
                .detach();
            })
            .log_err();
        }));
    }

    fn is_same_preview_path(&self, project_path: &ProjectPath, cx: &App) -> bool {
        self.preview_buffer
            .as_ref()
            .and_then(|b| b.read(cx).file())
            .map_or(false, |file| {
                file.worktree_id(cx) == project_path.worktree_id
                    && file.path() == &project_path.path
            })
    }

    fn navigate_and_highlight_matches(
        editor: &mut Editor,
        line: u32,
        match_ranges: &[AnchorRange],
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let point = text::Point::new(line, 0);
        editor.go_to_singleton_buffer_point(point, window, cx);

        let multi_buffer = editor.buffer().read(cx);
        if let Some(excerpt_id) = multi_buffer.excerpt_ids().first().copied() {
            let multi_buffer_ranges: Vec<_> = match_ranges
                .iter()
                .map(|range| MultiBufferAnchor::range_in_buffer(excerpt_id, range.clone()))
                .collect();
            editor.highlight_background::<QuickSearchHighlights>(
                &multi_buffer_ranges,
                |_, theme| theme.colors().search_match_background,
                cx,
            );
        }
    }

    fn schedule_preview_update(
        &mut self,
        data: Option<(ProjectPath, u32, Arc<Vec<AnchorRange>>)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.pending_preview_data = data.clone();

        if data.is_none() {
            self._preview_debounce_task = None;
            self.update_preview(None, window, cx);
            return;
        }

        if let Some((ref project_path, line, _)) = data {
            if self.is_same_preview_path(project_path, cx) {
                self._preview_debounce_task = None;
                if let Some(editor) = &self.preview_editor {
                    editor.update(cx, |editor, cx| {
                        let match_ranges = data
                            .as_ref()
                            .map(|(_, _, ranges)| ranges.as_slice())
                            .unwrap_or(&[]);
                        Self::navigate_and_highlight_matches(
                            editor,
                            line,
                            match_ranges,
                            window,
                            cx,
                        );
                    });
                }
                cx.notify();
                return;
            }
        }

        self._preview_debounce_task = Some(cx.spawn_in(window, async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(PREVIEW_DEBOUNCE_MS))
                .await;

            this.update_in(cx, |this, window, cx| {
                let data = this.pending_preview_data.take();
                this.update_preview(data, window, cx);
            })
            .log_err();
        }));
    }

    fn update_preview(
        &mut self,
        data: Option<(ProjectPath, u32, Arc<Vec<AnchorRange>>)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((project_path, line, match_ranges)) = data else {
            self.preview_editor = None;
            self.preview_buffer = None;
            self.preview_pending_path = None;
            self.preview_opened_in_workspace = None;
            self._preview_editor_subscription = None;
            cx.notify();
            return;
        };

        if self.is_same_preview_path(&project_path, cx) {
            self.preview_pending_path = None;

            if let Some(editor) = &self.preview_editor {
                editor.update(cx, |editor, cx| {
                    Self::navigate_and_highlight_matches(editor, line, &match_ranges, window, cx);
                });
            }
            cx.notify();
            return;
        }

        if self.preview_pending_path.as_ref() == Some(&project_path) {
            return;
        }

        self.preview_pending_path = Some(project_path.clone());

        let cached_buffer = self
            .picker
            .read(cx)
            .delegate
            .buffer_cache
            .get(&project_path)
            .cloned();

        if let Some(buffer) = cached_buffer {
            self.preview_pending_path = None;

            let project = self.project.clone();
            let editor = cx.new(|cx| {
                let mut editor = Editor::for_buffer(buffer.clone(), Some(project), window, cx);
                editor.set_show_gutter(true, cx);
                editor
            });

            editor.update(cx, |editor, cx| {
                Self::navigate_and_highlight_matches(editor, line, &match_ranges, window, cx);
            });

            self._preview_editor_subscription =
                Some(cx.subscribe_in(&editor, window, Self::on_preview_editor_event));
            self.preview_editor = Some(editor);
            self.preview_buffer = Some(buffer);
            self.preview_opened_in_workspace = None;
            cx.notify();
            return;
        }

        let project = self.project.clone();
        let open_buffer_task = project.update(cx, |project, cx| {
            project.open_buffer(project_path.clone(), cx)
        });

        cx.spawn_in(window, async move |this, cx| {
            let Ok(buffer) = open_buffer_task.await else {
                return;
            };

            this.update_in(cx, |this, window, cx| {
                if this.preview_pending_path.as_ref() != Some(&project_path) {
                    return;
                }
                this.preview_pending_path = None;

                let project = this.project.clone();
                let editor = cx.new(|cx| {
                    let mut editor = Editor::for_buffer(buffer.clone(), Some(project), window, cx);
                    editor.set_show_gutter(true, cx);
                    editor
                });

                editor.update(cx, |editor, cx| {
                    Self::navigate_and_highlight_matches(editor, line, &match_ranges, window, cx);
                });

                this._preview_editor_subscription =
                    Some(cx.subscribe_in(&editor, window, Self::on_preview_editor_event));
                this.preview_editor = Some(editor);
                this.preview_buffer = Some(buffer);
                this.preview_opened_in_workspace = None;
                cx.notify();
            })
            .log_err();
        })
        .detach();
    }
}

struct SearchResults {
    items: Vec<QuickSearchItem>,
    buffers: HashMap<ProjectPath, Entity<Buffer>>,
}

struct BatchCounters {
    total_files: usize,
    total_line_matches: usize,
    search_limited: bool,
}

fn process_buffer_into_batch(
    file_result: FileMatchResult,
    buffer: Entity<Buffer>,
    batch: &mut SearchResults,
    counters: &mut BatchCounters,
) {
    if let Some(first_match) = file_result.matches.first() {
        batch
            .buffers
            .insert(first_match.project_path.clone(), buffer);
    }

    batch.items.push(QuickSearchItem::FileHeader {
        file_name: file_result.file_name,
        parent_path: file_result.parent_path,
        file_key: file_result.file_key,
    });
    counters.total_files += 1;

    for match_data in file_result.matches {
        batch.items.push(QuickSearchItem::LineMatch(match_data));
        counters.total_line_matches += 1;
    }

    if counters.total_files > MAX_SEARCH_RESULT_FILES
        || counters.total_line_matches > MAX_SEARCH_RESULT_RANGES
    {
        counters.search_limited = true;
    }
}

fn process_results_in_background(
    buffer_data_list: Vec<(BufferExtractData, Entity<Buffer>)>,
) -> Vec<(FileMatchResult, Entity<Buffer>)> {
    buffer_data_list
        .into_iter()
        .filter_map(|(data, buffer)| process_file_matches(data).map(|result| (result, buffer)))
        .collect()
}

fn apply_batch_to_picker(
    delegate: &mut QuickSearchDelegate,
    batch: SearchResults,
    total_line_matches: usize,
    total_files: usize,
    is_first: bool,
    cx: &mut Context<Picker<QuickSearchDelegate>>,
) -> Option<(ProjectPath, u32, Arc<Vec<AnchorRange>>)> {
    let prev_items_len = delegate.items.len();
    delegate.items.extend(batch.items);

    for (project_path, buffer) in batch.buffers {
        if delegate.buffer_subscriptions.contains_key(&project_path) {
            delegate.buffer_cache.insert(project_path, buffer);
            continue;
        }

        let pp = project_path.clone();
        let subscription = cx.subscribe(&buffer, move |picker, _buffer, event, cx| {
            if matches!(event, BufferEvent::Reparsed) {
                picker.delegate.update_syntax_highlights_for_buffer(&pp, cx);
                cx.notify();
            }
        });
        delegate
            .buffer_subscriptions
            .insert(project_path.clone(), subscription);
        delegate.buffer_cache.insert(project_path.clone(), buffer);
        delegate.update_syntax_highlights_for_buffer(&project_path, cx);
    }

    delegate.update_visible_indices_from(prev_items_len);
    delegate.match_count = total_line_matches;
    delegate.file_count = total_files;

    if is_first {
        let first_selectable = delegate
            .visible_indices
            .iter()
            .position(|&actual_idx| {
                matches!(
                    delegate.items.get(actual_idx),
                    Some(QuickSearchItem::LineMatch(_))
                )
            })
            .unwrap_or(0);
        delegate.selected_index = first_selectable;
    }

    cx.notify();

    if is_first {
        delegate.items.iter().find_map(|item| {
            if let QuickSearchItem::LineMatch(data) = item {
                Some((
                    data.project_path.clone(),
                    data.line,
                    data.match_ranges.clone(),
                ))
            } else {
                None
            }
        })
    } else {
        None
    }
}

fn trigger_preview_update(
    quick_search: &WeakEntity<QuickSearchModal>,
    preview_data: Option<(ProjectPath, u32, Arc<Vec<AnchorRange>>)>,
    cx: &mut gpui::AsyncWindowContext,
) {
    if let Some(first_match) = preview_data {
        if let Some(quick_search) = quick_search.upgrade() {
            quick_search
                .update_in(cx, |qs, window, cx| {
                    qs.update_preview(Some(first_match), window, cx);
                })
                .log_err();
        }
    }
}

async fn process_and_apply_batch(
    buffer_data: Vec<(BufferExtractData, Entity<Buffer>)>,
    counters: &mut BatchCounters,
    is_first_batch: &mut bool,
    limit_reached: bool,
    picker: &WeakEntity<Picker<QuickSearchDelegate>>,
    quick_search: &WeakEntity<QuickSearchModal>,
    cx: &mut gpui::AsyncWindowContext,
) {
    let processed_results = cx
        .background_executor()
        .spawn(async move { process_results_in_background(buffer_data) })
        .await;

    let mut batch = SearchResults {
        items: Vec::with_capacity(processed_results.len() * 2),
        buffers: HashMap::default(),
    };

    for (file_result, buffer) in processed_results {
        process_buffer_into_batch(file_result, buffer, &mut batch, counters);
        if counters.search_limited {
            break;
        }
    }

    if limit_reached {
        counters.search_limited = true;
    }

    if !batch.items.is_empty() {
        let is_first = *is_first_batch;
        let total_line_matches = counters.total_line_matches;
        let total_files = counters.total_files;

        let preview_data = picker
            .update_in(cx, |picker, _window, cx| {
                apply_batch_to_picker(
                    &mut picker.delegate,
                    batch,
                    total_line_matches,
                    total_files,
                    is_first,
                    cx,
                )
            })
            .ok()
            .flatten();

        trigger_preview_update(quick_search, preview_data, cx);
        *is_first_batch = false;
    }
}

struct PendingBufferData {
    list: Vec<(BufferExtractData, Entity<Buffer>)>,
    limit_reached: bool,
    first_batch_sent: bool,
}

impl PendingBufferData {
    fn new() -> Self {
        Self {
            list: Vec::with_capacity(BACKGROUND_BATCH_THRESHOLD),
            limit_reached: false,
            first_batch_sent: false,
        }
    }

    fn len(&self) -> usize {
        self.list.len()
    }

    fn should_process(&self) -> bool {
        if self.limit_reached {
            return true;
        }
        let threshold = if self.first_batch_sent {
            BACKGROUND_BATCH_THRESHOLD
        } else {
            FIRST_BATCH_THRESHOLD
        };
        self.len() >= threshold
    }

    fn take(&mut self) -> Vec<(BufferExtractData, Entity<Buffer>)> {
        self.first_batch_sent = true;
        std::mem::take(&mut self.list)
    }

    fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

fn build_search_query(query: &str, search_options: SearchOptions) -> Result<SearchQuery, String> {
    if search_options.contains(SearchOptions::REGEX) {
        SearchQuery::regex(
            query,
            search_options.contains(SearchOptions::WHOLE_WORD),
            search_options.contains(SearchOptions::CASE_SENSITIVE),
            search_options.contains(SearchOptions::INCLUDE_IGNORED),
            false,
            PathMatcher::default(),
            PathMatcher::default(),
            false,
            None,
        )
    } else {
        SearchQuery::text(
            query,
            search_options.contains(SearchOptions::WHOLE_WORD),
            search_options.contains(SearchOptions::CASE_SENSITIVE),
            search_options.contains(SearchOptions::INCLUDE_IGNORED),
            PathMatcher::default(),
            PathMatcher::default(),
            false,
            None,
        )
    }
    .map_err(|e| e.to_string())
}

impl QuickSearchDelegate {
    fn clear_search_state(&mut self) {
        self.items.clear();
        self.visible_indices.clear();
        self.visible_line_match_indices.clear();
        self.match_count = 0;
        self.file_count = 0;
        self.is_limited = false;
        self.is_searching = false;
        self.regex_error = None;
        self.buffer_cache.clear();
        self.buffer_subscriptions.clear();
    }

    fn reset_for_new_search(&mut self) {
        self.items.clear();
        self.buffer_cache.clear();
        self.buffer_subscriptions.clear();
        self.visible_indices.clear();
        self.visible_line_match_indices.clear();
        self.match_count = 0;
        self.file_count = 0;
        self.is_limited = false;
    }

    fn update_visible_indices(&mut self) {
        self.update_visible_indices_from(0);
    }

    fn update_visible_indices_from(&mut self, start_index: usize) {
        if start_index == 0 {
            self.visible_indices.clear();
            self.visible_indices.reserve(self.items.len());
            self.visible_line_match_indices.clear();
            self.visible_line_match_indices.reserve(self.items.len());
        }

        for (idx, item) in self.items.iter().enumerate().skip(start_index) {
            match item {
                QuickSearchItem::FileHeader { .. } => {
                    self.visible_indices.push(idx);
                }
                QuickSearchItem::LineMatch(data) => {
                    if !self.collapsed_files.contains(&data.file_key) {
                        let visible_idx = self.visible_indices.len();
                        self.visible_indices.push(idx);
                        self.visible_line_match_indices.push(visible_idx);
                    }
                }
            }
        }
    }

    fn toggle_file_collapsed(&mut self, file_key: &SharedString) {
        let is_expanding = self.collapsed_files.contains(file_key);

        if is_expanding {
            self.collapsed_files.remove(file_key);
            self.expand_file_indices(file_key);
        } else {
            self.collapsed_files.insert(file_key.clone());
            self.collapse_file_indices(file_key);
        }
    }

    fn toggle_all_files_collapsed(&mut self, clicked_file_key: &SharedString) {
        let is_clicked_collapsed = self.collapsed_files.contains(clicked_file_key);

        if is_clicked_collapsed {
            self.collapsed_files.clear();
        } else {
            for item in &self.items {
                if let QuickSearchItem::FileHeader { file_key, .. } = item {
                    self.collapsed_files.insert(file_key.clone());
                }
            }
        }
        self.update_visible_indices();
    }

    fn collapse_file_indices(&mut self, file_key: &SharedString) {
        let mut indices_to_remove = Vec::new();

        for (visible_idx, &actual_idx) in self.visible_indices.iter().enumerate() {
            if let Some(QuickSearchItem::LineMatch(data)) = self.items.get(actual_idx) {
                if &data.file_key == file_key {
                    indices_to_remove.push(visible_idx);
                }
            }
        }

        for &visible_idx in indices_to_remove.iter().rev() {
            self.visible_indices.remove(visible_idx);
        }

        self.rebuild_visible_line_match_indices();
    }

    fn expand_file_indices(&mut self, file_key: &SharedString) {
        let header_visible_pos = self.visible_indices.iter().position(|&idx| {
            matches!(
                self.items.get(idx),
                Some(QuickSearchItem::FileHeader { file_key: fk, .. }) if fk == file_key
            )
        });

        let Some(header_visible_pos) = header_visible_pos else {
            return;
        };

        let header_actual_idx = self.visible_indices[header_visible_pos];

        let line_indices: Vec<usize> = self
            .items
            .iter()
            .enumerate()
            .skip(header_actual_idx + 1)
            .take_while(|(_, item)| {
                matches!(item, QuickSearchItem::LineMatch(data) if &data.file_key == file_key)
            })
            .map(|(idx, _)| idx)
            .collect();

        let insert_pos = header_visible_pos + 1;
        self.visible_indices
            .splice(insert_pos..insert_pos, line_indices);

        self.rebuild_visible_line_match_indices();
    }

    fn rebuild_visible_line_match_indices(&mut self) {
        self.visible_line_match_indices.clear();
        self.visible_line_match_indices
            .reserve(self.visible_indices.len());
        for (visible_idx, &actual_idx) in self.visible_indices.iter().enumerate() {
            if matches!(
                self.items.get(actual_idx),
                Some(QuickSearchItem::LineMatch(_))
            ) {
                self.visible_line_match_indices.push(visible_idx);
            }
        }
    }

    fn toggle_search_option(&mut self, option: SearchOptions) {
        self.search_options.toggle(option);
    }

    fn update_syntax_highlights_for_buffer(&mut self, project_path: &ProjectPath, cx: &App) {
        let Some(buffer) = self.buffer_cache.get(project_path) else {
            return;
        };

        let snapshot = buffer.read(cx).snapshot();

        for item in &mut self.items {
            if let QuickSearchItem::LineMatch(data) = item {
                if &data.project_path == project_path && data.syntax_highlights.is_none() {
                    data.syntax_highlights = extract_syntax_highlights_for_line(
                        &snapshot,
                        &data.preview_text,
                        data.line,
                        data.trim_start,
                    )
                    .map(Arc::new);
                }
            }
        }
    }

    #[inline]
    fn actual_index(&self, visible_index: usize) -> Option<usize> {
        self.visible_indices.get(visible_index).copied()
    }

    #[inline]
    fn is_line_match_at_visible_index(&self, visible_index: usize) -> bool {
        self.visible_indices
            .get(visible_index)
            .and_then(|&actual_idx| self.items.get(actual_idx))
            .map_or(false, |item| matches!(item, QuickSearchItem::LineMatch(_)))
    }

    #[inline]
    fn find_nearest_line_match(
        &self,
        from_visible_index: usize,
        going_down: bool,
    ) -> Option<usize> {
        if self.visible_line_match_indices.is_empty() {
            return None;
        }

        let search_result = self
            .visible_line_match_indices
            .binary_search(&from_visible_index);

        match search_result {
            Ok(pos) => Some(self.visible_line_match_indices[pos]),
            Err(insert_pos) => {
                if going_down {
                    if insert_pos < self.visible_line_match_indices.len() {
                        Some(self.visible_line_match_indices[insert_pos])
                    } else {
                        None
                    }
                } else if insert_pos > 0 {
                    Some(self.visible_line_match_indices[insert_pos - 1])
                } else {
                    None
                }
            }
        }
    }

    fn expand_and_select(
        &mut self,
        file_key: &SharedString,
        header_visible_idx: usize,
        going_down: bool,
    ) {
        self.toggle_file_collapsed(file_key);

        let target_idx = if going_down {
            self.find_first_match_of_file(file_key, header_visible_idx)
        } else {
            self.find_last_match_of_file(file_key, header_visible_idx)
        };

        if let Some(idx) = target_idx {
            self.selected_index = idx;
        }
    }

    fn find_first_match_of_file(
        &self,
        file_key: &SharedString,
        header_visible_idx: usize,
    ) -> Option<usize> {
        let next_idx = header_visible_idx + 1;
        if next_idx < self.visible_indices.len() {
            let actual_idx = *self.visible_indices.get(next_idx)?;
            if let Some(QuickSearchItem::LineMatch(data)) = self.items.get(actual_idx) {
                if &data.file_key == file_key {
                    return Some(next_idx);
                }
            }
        }
        None
    }

    fn find_last_match_of_file(
        &self,
        file_key: &SharedString,
        header_visible_idx: usize,
    ) -> Option<usize> {
        let mut last_match_idx = None;
        for check_idx in (header_visible_idx + 1)..self.visible_indices.len() {
            let actual_idx = *self.visible_indices.get(check_idx)?;
            match self.items.get(actual_idx) {
                Some(QuickSearchItem::LineMatch(data)) if &data.file_key == file_key => {
                    last_match_idx = Some(check_idx);
                }
                Some(QuickSearchItem::FileHeader { .. }) => break,
                _ => {}
            }
        }
        last_match_idx.or_else(|| self.find_first_match_of_file(file_key, header_visible_idx))
    }

    fn find_collapsed_file_in_direction(
        &self,
        from_visible_idx: usize,
        going_down: bool,
    ) -> Option<(usize, SharedString)> {
        let range: Box<dyn Iterator<Item = usize>> = if going_down {
            Box::new((from_visible_idx + 1)..self.visible_indices.len())
        } else {
            Box::new((0..from_visible_idx).rev())
        };

        for scan_idx in range {
            if let Some(&actual_idx) = self.visible_indices.get(scan_idx) {
                if let Some(QuickSearchItem::FileHeader { file_key, .. }) =
                    self.items.get(actual_idx)
                {
                    if self.collapsed_files.contains(file_key) {
                        return Some((scan_idx, file_key.clone()));
                    }
                }
            }
        }
        None
    }

    fn render_file_header(
        &self,
        ix: usize,
        file_name: &SharedString,
        parent_path: &SharedString,
        file_key: &SharedString,
        cx: &App,
    ) -> ListItem {
        let is_collapsed = self.collapsed_files.contains(file_key.as_ref());

        let chevron_icon = if is_collapsed {
            IconName::ChevronRight
        } else {
            IconName::ChevronDown
        };

        let file_icon = FileIcons::get_icon(Path::new(file_name.as_ref()), cx)
            .map(Icon::from_path)
            .unwrap_or_else(|| Icon::new(IconName::File));

        let quick_search = self.quick_search.clone();
        let file_key = file_key.clone();
        let file_name = file_name.clone();
        let parent_path = parent_path.clone();

        ListItem::new(ix)
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .child(
                h_flex()
                    .id(("file-header", ix))
                    .w_full()
                    .gap_1()
                    .cursor_pointer()
                    .on_click(move |event, window, cx| {
                        cx.stop_propagation();
                        if let Some(qs) = quick_search.upgrade() {
                            qs.update(cx, |qs, cx| {
                                window.focus(&qs.picker.focus_handle(cx), cx);
                                qs.picker.update(cx, |picker, cx| {
                                    if event.modifiers().alt {
                                        picker.delegate.toggle_all_files_collapsed(&file_key);
                                    } else {
                                        picker.delegate.toggle_file_collapsed(&file_key);
                                    }
                                    cx.notify();
                                });
                            });
                        }
                    })
                    .child(
                        Icon::new(chevron_icon)
                            .color(Color::Muted)
                            .size(ui::IconSize::Small),
                    )
                    .child(file_icon.color(Color::Muted).size(ui::IconSize::Small))
                    .child(Label::new(file_name).size(ui::LabelSize::Small))
                    .when(!parent_path.is_empty(), |this| {
                        this.child(
                            Label::new(parent_path)
                                .size(ui::LabelSize::Small)
                                .color(Color::Muted),
                        )
                    }),
            )
    }

    fn render_line_match(
        &self,
        ix: usize,
        selected: bool,
        line_label: &SharedString,
        preview_text: &SharedString,
        match_positions: &Arc<Vec<std::ops::Range<usize>>>,
        syntax_highlights: &Option<Arc<Vec<(std::ops::Range<usize>, HighlightId)>>>,
        cx: &App,
    ) -> ListItem {
        let quick_search = self.quick_search.clone();
        let preview_str: &str = preview_text.as_ref();

        let is_valid_range = |range: &std::ops::Range<usize>| -> bool {
            range.start < range.end
                && range.end <= preview_str.len()
                && preview_str.is_char_boundary(range.start)
                && preview_str.is_char_boundary(range.end)
        };

        let syntax_theme = cx.theme().syntax();
        let mut highlights: Vec<(std::ops::Range<usize>, HighlightStyle)> = syntax_highlights
            .as_ref()
            .map(|sh| {
                sh.iter()
                    .filter_map(|(range, id)| {
                        if !is_valid_range(range) {
                            return None;
                        }
                        id.style(&syntax_theme).map(|style| (range.clone(), style))
                    })
                    .collect()
            })
            .unwrap_or_default();

        for range in match_positions.iter() {
            if !is_valid_range(range) {
                continue;
            }
            let match_style = HighlightStyle {
                font_weight: Some(gpui::FontWeight::BOLD),
                ..Default::default()
            };
            highlights.push((range.clone(), match_style));
        }

        ListItem::new(ix)
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(selected)
            .on_click({
                move |event, window, cx| {
                    cx.stop_propagation();
                    let Some(qs) = quick_search.upgrade() else {
                        return;
                    };
                    if event.click_count() >= 2 {
                        qs.update(cx, |modal, cx| {
                            modal.picker.update(cx, |picker, cx| {
                                picker.delegate.selected_index = ix;
                                picker.delegate.confirm(false, window, cx);
                            });
                        });
                    } else {
                        let preview_data = {
                            let modal = qs.read(cx);
                            let delegate = &modal.picker.read(cx).delegate;
                            delegate.actual_index(ix).and_then(|idx| {
                                match delegate.items.get(idx) {
                                    Some(QuickSearchItem::LineMatch(data)) => Some((
                                        data.project_path.clone(),
                                        data.line,
                                        data.match_ranges.clone(),
                                    )),
                                    _ => None,
                                }
                            })
                        };

                        qs.update(cx, |modal, cx| {
                            window.focus(&modal.picker.focus_handle(cx), cx);
                            modal.picker.update(cx, |picker, cx| {
                                picker.delegate.selected_index = ix;
                                cx.notify();
                            });
                            modal.schedule_preview_update(preview_data, window, cx);
                        });
                    }
                }
            })
            .child(
                h_flex()
                    .w_full()
                    .gap_2()
                    .pl(px(20.))
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
                        Label::new(line_label)
                            .size(ui::LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
    }
}

impl PickerDelegate for QuickSearchDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.visible_indices.len()
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
        if self.visible_indices.is_empty() {
            self.selected_index = 0;
            return;
        }

        let ix = ix.min(self.visible_indices.len().saturating_sub(1));

        if self.is_line_match_at_visible_index(ix) {
            self.selected_index = ix;
            return;
        }

        let going_down = ix >= self.selected_index;

        let collapsed_file_at_ix = self.visible_indices.get(ix).and_then(|&actual_idx| {
            if let Some(QuickSearchItem::FileHeader { file_key, .. }) = self.items.get(actual_idx) {
                if self.collapsed_files.contains(file_key) {
                    Some(file_key.clone())
                } else {
                    None
                }
            } else {
                None
            }
        });

        if let Some(file_key) = collapsed_file_at_ix {
            self.expand_and_select(&file_key, ix, going_down);
            return;
        }

        if let Some(found) = self.find_nearest_line_match(ix, going_down) {
            let scan_range: Box<dyn Iterator<Item = usize>> = if going_down {
                Box::new((ix + 1)..found)
            } else {
                Box::new(((found + 1)..ix).rev())
            };

            for scan_idx in scan_range {
                if let Some(&actual_idx) = self.visible_indices.get(scan_idx) {
                    if let Some(QuickSearchItem::FileHeader { file_key, .. }) =
                        self.items.get(actual_idx)
                    {
                        if self.collapsed_files.contains(file_key) {
                            let file_key = file_key.clone();
                            self.expand_and_select(&file_key, scan_idx, going_down);
                            return;
                        }
                    }
                }
            }

            self.selected_index = found;
        } else {
            if let Some((collapsed_idx, file_key)) =
                self.find_collapsed_file_in_direction(ix, going_down)
            {
                self.expand_and_select(&file_key, collapsed_idx, going_down);
                return;
            }

            if let Some(found) = self.find_nearest_line_match(ix, !going_down) {
                self.selected_index = found;
            }
        }
    }

    fn selected_index_changed(
        &self,
        _ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Box<dyn Fn(&mut Window, &mut App) + 'static>> {
        let quick_search = self.quick_search.clone();
        let actual_index = self.actual_index(self.selected_index);
        let preview_data = actual_index.and_then(|idx| match self.items.get(idx) {
            Some(QuickSearchItem::LineMatch(data)) => Some((
                data.project_path.clone(),
                data.line,
                data.match_ranges.clone(),
            )),
            _ => None,
        });

        Some(Box::new(move |window, cx| {
            if let Some(quick_search) = quick_search.upgrade() {
                quick_search.update(cx, |qs, cx| {
                    qs.schedule_preview_update(preview_data.clone(), window, cx);
                });
            }
        }))
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search in project...".into()
    }

    fn render_editor(
        &self,
        editor: &Entity<Editor>,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        let search_options = self.search_options;
        let focus_handle = self.focus_handle.clone();

        let render_option_button_fn = |option: SearchOption, cx: &mut Context<Picker<Self>>| {
            let is_active = search_options.contains(option.as_options());
            let action = option.to_toggle_action();
            let label = option.label();
            let fh = focus_handle.clone();
            let options = option.as_options();

            IconButton::new(label, option.icon())
                .on_click(cx.listener(move |picker, _, window, cx| {
                    picker.delegate.toggle_search_option(options);
                    let query = picker.delegate.current_query.clone();
                    picker.set_query(query, window, cx);
                }))
                .style(ButtonStyle::Subtle)
                .shape(IconButtonShape::Square)
                .toggle_state(is_active)
                .when_some(fh, |this, fh| {
                    this.tooltip(move |_window, cx| Tooltip::for_action_in(label, action, &fh, cx))
                })
        };

        v_flex()
            .bg(cx.theme().colors().toolbar_background)
            .child(
                h_flex()
                    .overflow_hidden()
                    .flex_none()
                    .py_2()
                    .px_2()
                    .gap_2()
                    .child(
                        h_flex()
                            .flex_1()
                            .min_w_32()
                            .h_8()
                            .pl_2()
                            .pr_1()
                            .border_1()
                            .border_color(cx.theme().colors().border)
                            .rounded_md()
                            .child(editor.clone())
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(render_option_button_fn(SearchOption::CaseSensitive, cx))
                                    .child(render_option_button_fn(SearchOption::WholeWord, cx))
                                    .child(render_option_button_fn(SearchOption::Regex, cx))
                                    .child(render_option_button_fn(
                                        SearchOption::IncludeIgnored,
                                        cx,
                                    )),
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

        if query.is_empty() {
            self.clear_search_state();
            let quick_search = self.quick_search.clone();
            cx.defer_in(window, move |_, _window, cx| {
                if let Some(quick_search) = quick_search.upgrade() {
                    quick_search.update(cx, |qs, cx| {
                        qs.preview_editor = None;
                        qs.preview_buffer = None;
                        cx.notify();
                    });
                }
            });
            cx.notify();
            return Task::ready(());
        }

        self.is_searching = true;

        let file_count = get_project_file_count(self.project.read(cx), cx);
        let debounce_ms = compute_search_debounce_ms(file_count);

        let project = self.project.clone();
        let search_options = self.search_options;
        let quick_search = self.quick_search.clone();

        cx.spawn_in(window, async move |picker, cx| {
            if debounce_ms > 0 {
                cx.background_executor()
                    .timer(Duration::from_millis(debounce_ms))
                    .await;
            }

            let search_query = match build_search_query(&query, search_options) {
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
                            picker.delegate.visible_indices.clear();
                            picker.delegate.match_count = 0;
                            picker.delegate.file_count = 0;
                            picker.delegate.is_searching = false;
                            cx.notify();
                        })
                        .log_err();
                    return;
                }
            };

            let Some(project::SearchResults { rx: results_rx, _task_handle }) = project
                .update(cx, |project, cx| project.search(search_query, cx))
                .log_err()
            else {
                return;
            };

            picker
                .update(cx, |picker, cx| {
                    picker.delegate.reset_for_new_search();
                    cx.notify();
                })
                .log_err();

            let mut counters = BatchCounters {
                total_files: 0,
                total_line_matches: 0,
                search_limited: false,
            };
            let mut is_first_batch = true;
            let mut pending = PendingBufferData::new();

            let mut results_stream = pin!(results_rx.ready_chunks(STREAM_CHUNK_SIZE));
            while let Some(results) = results_stream.next().await {
                for result in results {
                    match result {
                        project::search::SearchResult::Buffer { buffer, ranges } => {
                            if ranges.is_empty() {
                                continue;
                            }

                            let extract_data = cx
                                .read_entity(&buffer, |buf, cx| {
                                    extract_buffer_data(buf, ranges, cx)
                                })
                                .log_err()
                                .flatten();

                            if let Some(data) = extract_data {
                                pending.list.push((data, buffer));
                            }
                        }
                        project::search::SearchResult::LimitReached => {
                            pending.limit_reached = true;
                            break;
                        }
                    }
                }

                if !pending.should_process() {
                    continue;
                }

                let buffer_data_to_process = pending.take();
                let limit_reached = pending.limit_reached;

                process_and_apply_batch(
                    buffer_data_to_process,
                    &mut counters,
                    &mut is_first_batch,
                    limit_reached,
                    &picker,
                    &quick_search,
                    cx,
                )
                .await;

                if counters.search_limited {
                    break;
                }
            }

            if !pending.is_empty() {
                let buffer_data_to_process = pending.take();
                let limit_reached = pending.limit_reached;

                process_and_apply_batch(
                    buffer_data_to_process,
                    &mut counters,
                    &mut is_first_batch,
                    limit_reached,
                    &picker,
                    &quick_search,
                    cx,
                )
                .await;
            }

            picker
                .update(cx, |picker, cx| {
                    picker.delegate.is_limited = counters.search_limited;
                    picker.delegate.is_searching = false;
                    cx.notify();
                })
                .log_err();
        })
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let actual_index = match self.actual_index(self.selected_index) {
            Some(idx) => idx,
            None => return,
        };

        let Some(QuickSearchItem::LineMatch(data)) = self.items.get(actual_index) else {
            return;
        };

        let project_path = data.project_path.clone();
        let line = data.line;

        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                let task = if secondary {
                    workspace.split_path_preview(project_path, false, None, window, cx)
                } else {
                    workspace.open_path(project_path, None, true, window, cx)
                };
                cx.spawn_in(window, async move |_, cx| {
                    if let Some(item) = task.await.log_err() {
                        if let Some(editor) = item.downcast::<Editor>() {
                            editor
                                .update_in(cx, |editor, window, cx| {
                                    let point = text::Point::new(line, 0);
                                    editor.go_to_singleton_buffer_point(point, window, cx);
                                })
                                .log_err();
                        }
                    }
                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);
            });
        }
        cx.emit(DismissEvent);
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
        let actual_ix = *self.visible_indices.get(ix)?;
        let item = self.items.get(actual_ix)?;

        match item {
            QuickSearchItem::FileHeader {
                file_name,
                parent_path,
                file_key,
            } => Some(self.render_file_header(ix, file_name, parent_path, file_key, cx)),
            QuickSearchItem::LineMatch(data) => Some(self.render_line_match(
                ix,
                selected,
                &data.line_label,
                &data.preview_text,
                &data.match_positions,
                &data.syntax_highlights,
                cx,
            )),
        }
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

        if self.match_count > 0 {
            let results_text = if self.is_limited {
                format!("{}+ results (limited)", self.match_count)
            } else {
                let result_word = if self.match_count == 1 {
                    "result"
                } else {
                    "results"
                };
                let file_word = if self.file_count == 1 {
                    "file"
                } else {
                    "files"
                };
                format!(
                    "{} {} in {} {}",
                    self.match_count, result_word, self.file_count, file_word
                )
            };
            return Some(
                h_flex()
                    .w_full()
                    .px_3()
                    .py_1()
                    .child(
                        Label::new(results_text)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .into_any(),
            );
        }

        Some(
            h_flex()
                .w_full()
                .px_3()
                .py_1()
                .child(
                    Label::new("0 results")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any(),
        )
    }

    fn render_footer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        let focus_handle = self.focus_handle.clone()?;

        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .gap_1()
                .justify_end()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Button::new("open-split", "Open in Split")
                        .key_binding(
                            KeyBinding::for_action_in(&menu::SecondaryConfirm, &focus_handle, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::SecondaryConfirm.boxed_clone(), cx);
                        }),
                )
                .child(
                    Button::new("open", "Open")
                        .key_binding(
                            KeyBinding::for_action_in(&menu::Confirm, &focus_handle, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::Confirm.boxed_clone(), cx);
                        }),
                )
                .into_any(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, VisualTestContext};
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use std::ops::Deref;
    use util::path;

    struct TestFixture {
        quick_search: Entity<QuickSearchModal>,
        cx: VisualTestContext,
    }

    impl TestFixture {
        async fn new(cx: &mut TestAppContext, files: serde_json::Value) -> Self {
            Self::new_with_query(cx, files, None).await
        }

        async fn new_with_query(
            cx: &mut TestAppContext,
            files: serde_json::Value,
            initial_query: Option<String>,
        ) -> Self {
            cx.update(|cx| {
                let settings = SettingsStore::test(cx);
                cx.set_global(settings);
                theme::init(theme::LoadThemes::JustBase, cx);
                editor::init(cx);
                crate::init(cx);
            });

            let fs = FakeFs::new(cx.background_executor.clone());
            fs.insert_tree(path!("/project"), files).await;

            let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
            let window =
                cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
            let workspace = window.root(cx).unwrap();
            let mut visual_cx = VisualTestContext::from_window(*window.deref(), cx);

            let quick_search = visual_cx.new_window_entity({
                let weak_workspace = workspace.downgrade();
                move |window, cx| {
                    QuickSearchModal::new(weak_workspace, project, initial_query, window, cx)
                }
            });

            Self {
                quick_search,
                cx: visual_cx,
            }
        }

        async fn search(&mut self, query: &str) {
            self.quick_search
                .update_in(&mut self.cx, |modal, window, cx| {
                    modal.picker.update(cx, |picker, cx| {
                        picker
                            .delegate
                            .update_matches(query.to_string(), window, cx)
                    })
                })
                .await;
        }

        fn set_query(&mut self, query: &str) {
            self.quick_search
                .update_in(&mut self.cx, |modal, window, cx| {
                    modal.picker.update(cx, |picker, cx| {
                        picker.set_query(query, window, cx);
                    });
                });
        }

        fn toggle_option(&mut self, option: SearchOptions) {
            self.quick_search.update(&mut self.cx, |modal, cx| {
                modal.picker.update(cx, |picker, _cx| {
                    picker.delegate.toggle_search_option(option);
                });
            });
        }

        fn set_items(&mut self, items: Vec<QuickSearchItem>) {
            self.quick_search.update(&mut self.cx, |modal, cx| {
                modal.picker.update(cx, |picker, _cx| {
                    picker.delegate.items = items;
                    picker.delegate.update_visible_indices();
                });
            });
        }

        fn toggle_file_collapsed(&mut self, file_key: &SharedString) {
            let file_key = file_key.clone();
            self.quick_search.update(&mut self.cx, |modal, cx| {
                modal.picker.update(cx, |picker, _cx| {
                    picker.delegate.toggle_file_collapsed(&file_key);
                });
            });
        }

        fn toggle_all_files_collapsed(&mut self, file_key: &SharedString) {
            let file_key = file_key.clone();
            self.quick_search.update(&mut self.cx, |modal, cx| {
                modal.picker.update(cx, |picker, _cx| {
                    picker.delegate.toggle_all_files_collapsed(&file_key);
                });
            });
        }

        fn delegate<T>(&mut self, read_fn: impl FnOnce(&QuickSearchDelegate) -> T) -> T {
            self.quick_search.update(&mut self.cx, |modal, cx| {
                read_fn(&modal.picker.read(cx).delegate)
            })
        }
    }

    fn file_header(file_name: &str, parent_path: &str) -> QuickSearchItem {
        let file_key = format_file_key(parent_path, file_name);
        QuickSearchItem::FileHeader {
            file_name: SharedString::from(file_name.to_string()),
            parent_path: SharedString::from(parent_path.to_string()),
            file_key,
        }
    }

    fn line_match(file_key: &str, line: u32, preview: &str) -> QuickSearchItem {
        QuickSearchItem::LineMatch(LineMatchData {
            project_path: ProjectPath {
                worktree_id: project::WorktreeId::from_usize(0),
                path: util::rel_path::rel_path(file_key).into(),
            },
            file_key: SharedString::from(file_key.to_string()),
            line,
            line_label: SharedString::from((line + 1).to_string()),
            preview_text: SharedString::from(preview.to_string()),
            match_ranges: Arc::new(Vec::new()),
            match_positions: Arc::new(Vec::new()),
            trim_start: 0,
            syntax_highlights: None,
        })
    }

    #[gpui::test]
    async fn test_quick_search_modal_creation(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(cx, json!({"file.rs": "fn main() {}\n"})).await;

        fixture.quick_search.update(&mut fixture.cx, |modal, cx| {
            assert!(modal.preview_editor.is_none());
            assert!(modal.preview_buffer.is_none());
            assert_eq!(modal.picker.read(cx).delegate.items.len(), 0);
        });
    }

    #[gpui::test]
    async fn test_quick_search_empty_query_clears_results(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(cx, json!({"file.rs": "fn test() {}\n"})).await;

        fixture.search("test").await;
        assert!(fixture.delegate(|d| d.items.len()) > 0);

        fixture.search("").await;
        fixture.delegate(|d| {
            assert_eq!(d.items.len(), 0);
        });
    }

    #[gpui::test]
    async fn test_quick_search_no_results_for_nonexistent_query(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(cx, json!({"file.rs": "fn main() {}\n"})).await;

        fixture.search("nonexistent_string_xyz_123").await;
        assert_eq!(fixture.delegate(|d| d.items.len()), 0);
    }

    #[gpui::test]
    async fn test_quick_search_finds_matches(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(
            cx,
            json!({
                "src": {
                    "main.rs": "fn main() {\n    println!(\"hello world\");\n}\n",
                    "lib.rs": "pub fn hello() {}\npub fn hello_world() {}\n",
                },
                "tests": { "test.rs": "fn test_hello() {}\n" }
            }),
        )
        .await;

        fixture.search("hello").await;

        fixture.delegate(|d| {
            assert!(d.match_count >= 3);
            assert!(d.file_count >= 2);
            assert!(!d.is_searching);
            assert!(d.regex_error.is_none());
        });
    }

    #[gpui::test]
    async fn test_quick_search_case_sensitive_option(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(
            cx,
            json!({"file.rs": "fn Hello() {}\nfn hello() {}\nfn HELLO() {}\n"}),
        )
        .await;

        fixture.search("Hello").await;
        let case_insensitive_count = fixture.delegate(|d| d.match_count);

        fixture.toggle_option(SearchOptions::CASE_SENSITIVE);
        fixture.search("Hello").await;

        fixture.delegate(|d| {
            assert!(d.search_options.contains(SearchOptions::CASE_SENSITIVE));
            assert_eq!(d.match_count, 1);
            assert!(case_insensitive_count > d.match_count);
        });
    }

    #[gpui::test]
    async fn test_quick_search_whole_word_option(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(
            cx,
            json!({"file.rs": "fn test() {}\nfn testing() {}\nfn my_test_fn() {}\n"}),
        )
        .await;

        fixture.search("test").await;
        let partial_count = fixture.delegate(|d| d.match_count);

        fixture.toggle_option(SearchOptions::WHOLE_WORD);
        fixture.search("test").await;

        fixture.delegate(|d| {
            assert!(d.search_options.contains(SearchOptions::WHOLE_WORD));
            assert!(d.match_count < partial_count);
        });
    }

    #[gpui::test]
    async fn test_quick_search_regex_option(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(
            cx,
            json!({"file.rs": "fn test1() {}\nfn test2() {}\nfn test10() {}\nfn other() {}\n"}),
        )
        .await;

        fixture.toggle_option(SearchOptions::REGEX);
        fixture.search("test\\d+").await;

        fixture.delegate(|d| {
            assert!(d.search_options.contains(SearchOptions::REGEX));
            assert_eq!(d.match_count, 3);
            assert!(d.regex_error.is_none());
        });
    }

    #[gpui::test]
    async fn test_quick_search_invalid_regex_shows_error(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(cx, json!({"file.rs": "fn test() {}\n"})).await;

        fixture.toggle_option(SearchOptions::REGEX);
        fixture.search("[invalid(regex").await;

        fixture.delegate(|d| {
            assert!(d.regex_error.is_some());
            assert_eq!(d.items.len(), 0);
            assert!(!d.is_searching);
        });
    }

    #[gpui::test]
    async fn test_quick_search_delegate_collapse_expand(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(cx, json!({"file.rs": ""})).await;

        fixture.set_items(vec![
            file_header("test.rs", "src"),
            line_match("src/test.rs", 0, "fn test()"),
            line_match("src/test.rs", 1, "fn other()"),
            file_header("lib.rs", "src"),
            line_match("src/lib.rs", 0, "pub fn lib_test()"),
        ]);

        fixture.delegate(|d| {
            assert_eq!(d.visible_indices.len(), 5);
            assert_eq!(d.visible_line_match_indices.len(), 3);
        });

        let file_key: SharedString = "src/test.rs".into();
        fixture.toggle_file_collapsed(&file_key);

        fixture.delegate(|d| {
            assert!(d.collapsed_files.contains(&file_key));
            assert_eq!(d.visible_indices.len(), 3);
            assert_eq!(d.visible_line_match_indices.len(), 1);
        });

        fixture.toggle_file_collapsed(&file_key);

        fixture.delegate(|d| {
            assert!(!d.collapsed_files.contains(&file_key));
            assert_eq!(d.visible_indices.len(), 5);
            assert_eq!(d.visible_line_match_indices.len(), 3);
        });
    }

    #[gpui::test]
    async fn test_quick_search_toggle_all_files_collapsed(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(cx, json!({"file.rs": ""})).await;

        fixture.set_items(vec![
            file_header("test.rs", "src"),
            line_match("src/test.rs", 0, "fn test()"),
            file_header("lib.rs", "src"),
            line_match("src/lib.rs", 0, "pub fn lib()"),
        ]);

        assert_eq!(fixture.delegate(|d| d.visible_indices.len()), 4);

        let file_key: SharedString = "src/test.rs".into();
        fixture.toggle_all_files_collapsed(&file_key);

        fixture.delegate(|d| {
            assert_eq!(d.collapsed_files.len(), 2);
            assert_eq!(d.visible_indices.len(), 2);
            assert_eq!(d.visible_line_match_indices.len(), 0);
        });

        fixture.toggle_all_files_collapsed(&file_key);

        fixture.delegate(|d| {
            assert_eq!(d.collapsed_files.len(), 0);
            assert_eq!(d.visible_indices.len(), 4);
        });
    }

    #[gpui::test]
    async fn test_quick_search_find_nearest_line_match(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(cx, json!({"file.rs": ""})).await;

        fixture.set_items(vec![
            file_header("test.rs", "src"),
            line_match("src/test.rs", 0, "fn test()"),
            file_header("lib.rs", "src"),
            line_match("src/lib.rs", 0, "pub fn lib()"),
        ]);

        fixture.delegate(|d| {
            assert_eq!(d.find_nearest_line_match(0, true), Some(1));
            assert_eq!(d.find_nearest_line_match(2, false), Some(1));
            assert_eq!(d.find_nearest_line_match(1, true), Some(1));
            assert_eq!(d.find_nearest_line_match(3, true), Some(3));
        });
    }

    #[gpui::test]
    async fn test_quick_search_navigation_down_expands_collapsed_file(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(cx, json!({"file.rs": ""})).await;

        fixture.set_items(vec![
            file_header("test.rs", "src"),
            line_match("src/test.rs", 0, "fn test()"),
            line_match("src/test.rs", 1, "fn other()"),
            file_header("lib.rs", "src"),
            line_match("src/lib.rs", 0, "pub fn lib()"),
        ]);

        let file_key: SharedString = "src/lib.rs".into();
        fixture.toggle_file_collapsed(&file_key);

        fixture.delegate(|d| {
            assert!(d.collapsed_files.contains(&file_key));
            assert_eq!(d.visible_indices.len(), 4);
        });

        fixture
            .quick_search
            .update_in(&mut fixture.cx, |modal, window, cx| {
                modal.picker.update(cx, |picker, cx| {
                    picker.delegate.selected_index = 2;
                    picker.delegate.set_selected_index(3, window, cx);
                });
            });

        fixture.delegate(|d| {
            assert!(!d.collapsed_files.contains(&file_key));
            assert_eq!(d.visible_indices.len(), 5);
            assert_eq!(d.selected_index, 4);
        });
    }

    #[gpui::test]
    async fn test_quick_search_navigation_up_expands_collapsed_file(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(cx, json!({"file.rs": ""})).await;

        fixture.set_items(vec![
            file_header("test.rs", "src"),
            line_match("src/test.rs", 0, "fn test()"),
            line_match("src/test.rs", 1, "fn other()"),
            file_header("lib.rs", "src"),
            line_match("src/lib.rs", 0, "pub fn lib()"),
            line_match("src/lib.rs", 1, "pub fn lib2()"),
        ]);

        let file_key: SharedString = "src/test.rs".into();
        fixture.toggle_file_collapsed(&file_key);

        fixture.delegate(|d| {
            assert!(d.collapsed_files.contains(&file_key));
            assert_eq!(d.visible_indices.len(), 4);
        });

        fixture
            .quick_search
            .update_in(&mut fixture.cx, |modal, window, cx| {
                modal.picker.update(cx, |picker, cx| {
                    picker.delegate.selected_index = 2;
                    picker.delegate.set_selected_index(1, window, cx);
                });
            });

        fixture.delegate(|d| {
            assert!(!d.collapsed_files.contains(&file_key));
            assert_eq!(d.visible_indices.len(), 6);
            assert_eq!(d.selected_index, 2);
        });
    }

    #[gpui::test]
    async fn test_quick_search_navigation_up_skipping_collapsed_file(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(cx, json!({"file.rs": ""})).await;

        fixture.set_items(vec![
            file_header("a.rs", "src"),
            line_match("src/a.rs", 0, "fn a()"),
            file_header("b.rs", "src"),
            line_match("src/b.rs", 0, "fn b()"),
            file_header("c.rs", "src"),
            line_match("src/c.rs", 0, "fn c()"),
        ]);

        let file_key_b: SharedString = "src/b.rs".into();
        fixture.toggle_file_collapsed(&file_key_b);

        fixture.delegate(|d| {
            assert!(d.collapsed_files.contains(&file_key_b));
            assert_eq!(d.visible_indices.len(), 5);
        });

        fixture
            .quick_search
            .update_in(&mut fixture.cx, |modal, window, cx| {
                modal.picker.update(cx, |picker, cx| {
                    picker.delegate.selected_index = 4;
                    picker.delegate.set_selected_index(3, window, cx);
                });
            });

        fixture.delegate(|d| {
            assert!(!d.collapsed_files.contains(&file_key_b));
            assert_eq!(d.visible_indices.len(), 6);
            assert_eq!(d.selected_index, 3);
        });
    }

    #[gpui::test]
    async fn test_quick_search_navigation_up_multiple_collapsed_expands_nearest(
        cx: &mut TestAppContext,
    ) {
        let mut fixture = TestFixture::new(cx, json!({"file.rs": ""})).await;

        fixture.set_items(vec![
            file_header("a.rs", "src"),
            line_match("src/a.rs", 0, "fn a()"),
            file_header("b.rs", "src"),
            line_match("src/b.rs", 0, "fn b()"),
            file_header("c.rs", "src"),
            line_match("src/c.rs", 0, "fn c()"),
            file_header("d.rs", "src"),
            line_match("src/d.rs", 0, "fn d()"),
        ]);

        let file_key_a: SharedString = "src/a.rs".into();
        let file_key_b: SharedString = "src/b.rs".into();
        let file_key_c: SharedString = "src/c.rs".into();
        fixture.toggle_file_collapsed(&file_key_a);
        fixture.toggle_file_collapsed(&file_key_b);
        fixture.toggle_file_collapsed(&file_key_c);

        fixture.delegate(|d| {
            assert_eq!(d.collapsed_files.len(), 3);
            assert_eq!(d.visible_indices.len(), 5);
        });

        fixture
            .quick_search
            .update_in(&mut fixture.cx, |modal, window, cx| {
                modal.picker.update(cx, |picker, cx| {
                    picker.delegate.selected_index = 4;
                    picker.delegate.set_selected_index(3, window, cx);
                });
            });

        fixture.delegate(|d| {
            assert!(d.collapsed_files.contains(&file_key_a));
            assert!(d.collapsed_files.contains(&file_key_b));
            assert!(!d.collapsed_files.contains(&file_key_c));
            assert_eq!(d.collapsed_files.len(), 2);
        });
    }

    #[gpui::test]
    fn test_truncate_preview() {
        assert_eq!(
            truncate_preview("fn test() {}", MAX_PREVIEW_BYTES).as_ref(),
            "fn test() {}"
        );

        let long_text = "a".repeat(300);
        let truncated = truncate_preview(&long_text, MAX_PREVIEW_BYTES);
        assert!(truncated.len() <= MAX_PREVIEW_BYTES + 3);
        assert!(truncated.ends_with('…'));

        assert_eq!(
            truncate_preview("   fn test()   ", MAX_PREVIEW_BYTES).as_ref(),
            "fn test()"
        );
    }

    #[gpui::test]
    fn test_format_file_key() {
        assert_eq!(format_file_key("src", "main.rs").as_ref(), "src/main.rs");
        assert_eq!(format_file_key("", "main.rs").as_ref(), "main.rs");
    }

    #[gpui::test]
    fn test_build_search_query_text() {
        assert!(build_search_query("test", SearchOptions::NONE).is_ok());
        assert!(build_search_query("test", SearchOptions::CASE_SENSITIVE).is_ok());
        assert!(build_search_query("test", SearchOptions::WHOLE_WORD).is_ok());
    }

    #[gpui::test]
    fn test_build_search_query_regex() {
        assert!(build_search_query("test\\d+", SearchOptions::REGEX).is_ok());

        let query = build_search_query("[invalid", SearchOptions::REGEX);
        assert!(query.is_err());
        assert!(!query.unwrap_err().is_empty());
    }

    #[gpui::test]
    async fn test_quick_search_initial_query_from_selection(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new_with_query(
            cx,
            json!({"file.rs": "fn hello() {}\nfn world() {}\n"}),
            Some("hello".to_string()),
        )
        .await;

        assert_eq!(fixture.delegate(|d| d.current_query.clone()), "hello");

        fixture.search("hello").await;
        assert!(fixture.delegate(|d| d.match_count) > 0);
    }

    #[gpui::test]
    async fn test_quick_search_many_matches(cx: &mut TestAppContext) {
        let content = (0..500)
            .map(|i| format!("fn test_function_{}() {{}}", i))
            .collect::<Vec<_>>()
            .join("\n");

        let mut fixture = TestFixture::new(cx, json!({ "large_file.rs": content })).await;

        fixture.search("test_function").await;

        let (match_count, file_count) = fixture.delegate(|d| (d.match_count, d.file_count));

        assert_eq!(match_count, 500);
        assert_eq!(file_count, 1);
    }

    #[gpui::test]
    async fn test_quick_search_rapid_query_updates(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(cx, json!({"file.rs": "fn main() {}\n"})).await;

        fixture.set_query("fn");
        fixture.set_query("fn m");
        fixture.set_query("fn ma");
        fixture.set_query("fn mai");
        fixture.set_query("fn main");

        fixture.quick_search.update(&mut fixture.cx, |modal, cx| {
            modal.picker.update(cx, |picker, cx| {
                picker.delegate.is_searching = false;
                cx.notify();
            });
        });

        fixture.search("fn main").await;

        let match_count = fixture.delegate(|d| d.match_count);
        assert!(match_count > 0);
    }

    #[gpui::test]
    async fn test_quick_search_unicode_query(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(
            cx,
            json!({
                "unicode.rs": "// 日本語コメント\nfn main() { println!(\"こんにちは\"); }\n",
                "emoji.rs": "// 🎉 celebration\nfn party() {}\n"
            }),
        )
        .await;

        fixture.search("日本語").await;
        assert!(fixture.delegate(|d| d.match_count) > 0);

        fixture.search("🎉").await;
        assert!(fixture.delegate(|d| d.match_count) > 0);
    }

    #[gpui::test]
    async fn test_quick_search_special_regex_chars(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(
            cx,
            json!({
                "file.rs": "let x = a + b;\nlet y = (a * b);\n"
            }),
        )
        .await;

        fixture.search("(a * b)").await;
        let match_count = fixture.delegate(|d| d.match_count);
        assert!(match_count > 0);
    }

    #[gpui::test]
    async fn test_quick_search_empty_file(cx: &mut TestAppContext) {
        let mut fixture = TestFixture::new(
            cx,
            json!({
                "empty.rs": "",
                "nonempty.rs": "fn main() {}"
            }),
        )
        .await;

        fixture.search("fn main").await;
        let (match_count, file_count) = fixture.delegate(|d| (d.match_count, d.file_count));

        assert_eq!(match_count, 1);
        assert_eq!(file_count, 1);
    }
}
