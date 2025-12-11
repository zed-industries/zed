use collections::{HashMap, HashSet};
use editor::{Anchor as MultiBufferAnchor, Editor, EditorEvent};
use file_icons::FileIcons;
use futures::StreamExt;
use gpui::{
    Action, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Pixels,
    Render, SharedString, Subscription, Task, WeakEntity, Window, actions, prelude::*,
};
use language::Buffer;
use picker::{Picker, PickerDelegate};
use project::{Project, ProjectPath, search::SearchQuery};
use std::{path::Path, pin::pin, sync::Arc, time::Duration};
use text::ToPoint as _;
use ui::{
    Button, ButtonStyle, Color, Icon, IconButton, IconButtonShape, IconName, KeyBinding, Label,
    ListItem, ListItemSpacing, SpinnerLabel, Tooltip, prelude::*, rems_from_px,
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

struct LineData {
    line_label: SharedString,
    preview_text: SharedString,
    match_ranges: Vec<AnchorRange>,
}

const MODAL_HEIGHT: Pixels = px(650.);
const MODAL_WIDTH: Pixels = px(1100.);
const LEFT_PANEL_WIDTH: Pixels = px(300.);
const MAX_LINE_MATCHES: usize = 200;
const MAX_PREVIEW_BYTES: usize = 200;
const SEARCH_DEBOUNCE_MS: u64 = 100;
const PREVIEW_DEBOUNCE_MS: u64 = 50;
const EDIT_OPEN_DELAY_MS: u64 = 200;

actions!(search, [QuickSearch]);

struct LineMatchData {
    project_path: ProjectPath,
    file_key: SharedString,
    line: u32,
    line_label: SharedString,
    preview_text: SharedString,
    match_ranges: Arc<Vec<AnchorRange>>,
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

fn extract_file_matches(
    buf: &Buffer,
    ranges: &[AnchorRange],
    cx: &App,
) -> Option<FileMatchResult> {
    let file = buf.file()?;
    let project_path = ProjectPath {
        worktree_id: file.worktree_id(cx),
        path: file.path().clone(),
    };

    let (file_name, parent_path) = extract_path_parts(&project_path.path);
    let file_key = format_file_key(&parent_path, &file_name);

    let snapshot = buf.snapshot();
    let mut lines_data: HashMap<u32, LineData> = HashMap::default();
    let mut line_order = Vec::new();

    for range in ranges {
        let start_point = range.start.to_point(&snapshot);
        let line = start_point.row;

        if let Some(data) = lines_data.get_mut(&line) {
            data.match_ranges.push(range.clone());
        } else {
            let line_start = snapshot.point_to_offset(text::Point::new(line, 0));
            let line_end_col = snapshot.line_len(line);
            let line_end = snapshot.point_to_offset(text::Point::new(line, line_end_col));

            let line_text: String = snapshot.text_for_range(line_start..line_end).collect();
            let preview_text = truncate_preview(&line_text, MAX_PREVIEW_BYTES);
            let line_label: SharedString = (line + 1).to_string().into();

            lines_data.insert(
                line,
                LineData {
                    line_label,
                    preview_text,
                    match_ranges: vec![range.clone()],
                },
            );
            line_order.push(line);
        }

        if line_order.len() >= MAX_LINE_MATCHES {
            break;
        }
    }

    if line_order.is_empty() {
        return None;
    }

    let matches = line_order
        .into_iter()
        .filter_map(|line| {
            let data = lines_data.remove(&line)?;
            Some(LineMatchData {
                project_path: project_path.clone(),
                file_key: file_key.clone(),
                line,
                line_label: data.line_label,
                preview_text: data.preview_text,
                match_ranges: Arc::new(data.match_ranges),
            })
        })
        .collect();

    Some(FileMatchResult {
        file_name,
        parent_path,
        file_key,
        matches,
    })
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
    LineMatch {
        project_path: ProjectPath,
        file_key: SharedString,
        line: u32,
        line_label: SharedString,
        preview_text: SharedString,
        match_ranges: Arc<Vec<AnchorRange>>,
    },
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
    pending_search_id: usize,
    quick_search: WeakEntity<QuickSearchModal>,
    match_count: usize,
    file_count: usize,
    is_limited: bool,
    is_searching: bool,
    current_query: String,
    focus_handle: Option<FocusHandle>,
    regex_error: Option<String>,
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let preview_editor = self.preview_editor.clone();
        let picker = self.picker.clone();

        let delegate = &self.picker.read(cx).delegate;
        let is_searching = delegate.is_searching;
        let search_options = delegate.search_options;
        let focus_handle = self.picker.focus_handle(cx);

        div()
            .id("quick-search-modal")
            .relative()
            .h(MODAL_HEIGHT)
            .w(MODAL_WIDTH)
            .child(
                v_flex()
                    .elevation_3(cx)
                    .size_full()
                    .overflow_hidden()
                    .border_1()
                    .rounded_none()
                    .border_color(cx.theme().colors().border)
                    .on_mouse_down_out(cx.listener(|_, _, _, cx| {
                        cx.emit(DismissEvent);
                    }))
                    .child(
                        h_flex()
                            .w_full()
                            .flex_1()
                            .min_h_0()
                            .overflow_hidden()
                            .child(
                                v_flex()
                                    .w(LEFT_PANEL_WIDTH)
                                    .flex_shrink_0()
                                    .h_full()
                                    .min_h_0()
                                    .overflow_hidden()
                                    .border_r_1()
                                    .border_color(cx.theme().colors().border)
                                    .child(
                                        h_flex()
                                            .w_full()
                                            .px_3()
                                            .py_2()
                                            .bg(cx.theme().colors().title_bar_background)
                                            .border_b_1()
                                            .border_color(cx.theme().colors().border)
                                            .justify_between()
                                            .child(
                                                h_flex()
                                                    .gap_2()
                                                    .child(
                                                        Label::new("Quick Search")
                                                            .size(LabelSize::Small)
                                                            .color(Color::Muted),
                                                    )
                                                    .when(is_searching, |this| {
                                                        this.child(
                                                            SpinnerLabel::new()
                                                                .size(LabelSize::Small)
                                                                .color(Color::Muted),
                                                        )
                                                    }),
                                            )
                                            .child(
                                                h_flex()
                                                    .gap_0p5()
                                                    .child(Self::render_search_option_button(
                                                        SearchOption::CaseSensitive,
                                                        search_options,
                                                        focus_handle.clone(),
                                                        cx,
                                                    ))
                                                    .child(Self::render_search_option_button(
                                                        SearchOption::WholeWord,
                                                        search_options,
                                                        focus_handle.clone(),
                                                        cx,
                                                    ))
                                                    .child(Self::render_search_option_button(
                                                        SearchOption::Regex,
                                                        search_options,
                                                        focus_handle.clone(),
                                                        cx,
                                                    ))
                                                    .child(Self::render_search_option_button(
                                                        SearchOption::IncludeIgnored,
                                                        search_options,
                                                        focus_handle,
                                                        cx,
                                                    )),
                                            ),
                                    )
                                    .child(self.picker.clone()),
                            )
                            .child({
                                let project = self.project.clone();
                                let save_preview_editor = preview_editor.clone();
                                v_flex()
                                    .id("quick-search-preview")
                                    .relative()
                                    .flex_1()
                                    .h_full()
                                    .overflow_hidden()
                                    .bg(cx.theme().colors().editor_background)
                                    .on_click(move |_, window, cx| {
                                        window.focus(&picker.focus_handle(cx));
                                    })
                                    .on_action({
                                        move |_: &Save, window, cx| {
                                            if let Some(editor) = save_preview_editor.clone() {
                                                editor.update(cx, |editor, cx| {
                                                    editor
                                                        .save(
                                                            SaveOptions::default(),
                                                            project.clone(),
                                                            window,
                                                            cx,
                                                        )
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
                                                .child(
                                                    Label::new("Select a result to preview")
                                                        .color(Color::Muted),
                                                ),
                                        )
                                    })
                            }),
                    ),
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

    fn render_search_option_button(
        option: SearchOption,
        active: SearchOptions,
        focus_handle: FocusHandle,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let action = option.to_toggle_action();
        let label = option.label();
        let search_option = option.as_options();
        IconButton::new(label, option.icon())
            .on_click(cx.listener(move |modal, _, window, cx| {
                modal.picker.update(cx, |picker, cx| {
                    picker.delegate.toggle_search_option(search_option);
                    let query = picker.delegate.current_query.clone();
                    picker.set_query(query, window, cx);
                });
            }))
            .style(ButtonStyle::Subtle)
            .shape(IconButtonShape::Square)
            .toggle_state(active.contains(option.as_options()))
            .tooltip(move |_window, cx| Tooltip::for_action_in(label, action, &focus_handle, cx))
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
            pending_search_id: 0,
            quick_search: weak_self,
            match_count: 0,
            file_count: 0,
            is_limited: false,
            is_searching: false,
            current_query: initial_query.clone().unwrap_or_default(),
            focus_handle: None,
            regex_error: None,
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
                        window.focus(&preview_editor.focus_handle(cx));
                    })
                    .log_err();
                })
                .detach();
            })
            .log_err();
        }));
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
            let same_path = self
                .preview_buffer
                .as_ref()
                .and_then(|b| b.read(cx).file())
                .map_or(false, |file| {
                    file.worktree_id(cx) == project_path.worktree_id
                        && file.path() == &project_path.path
                });

            if same_path {
                self._preview_debounce_task = None;
                if let Some(editor) = &self.preview_editor {
                    editor.update(cx, |editor, cx| {
                        let match_ranges = data
                            .as_ref()
                            .map(|(_, _, ranges)| ranges.as_slice())
                            .unwrap_or(&[]);
                        Self::navigate_and_highlight_matches(editor, line, match_ranges, window, cx);
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

        let same_path = self
            .preview_buffer
            .as_ref()
            .and_then(|b| b.read(cx).file())
            .map_or(false, |file| {
                file.worktree_id(cx) == project_path.worktree_id
                    && file.path() == &project_path.path
            });

        if same_path {
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
    line_match_count: usize,
    file_count: usize,
    is_limited: bool,
}

impl SearchResults {
    fn first_line_match(&self) -> Option<(ProjectPath, u32, Arc<Vec<AnchorRange>>)> {
        self.items.iter().find_map(|item| {
            if let QuickSearchItem::LineMatch {
                project_path,
                line,
                match_ranges,
                ..
            } = item
            {
                Some((project_path.clone(), *line, match_ranges.clone()))
            } else {
                None
            }
        })
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
    fn update_visible_indices(&mut self) {
        self.visible_indices.clear();
        self.visible_line_match_indices.clear();

        for (idx, item) in self.items.iter().enumerate() {
            match item {
                QuickSearchItem::FileHeader { .. } => {
                    self.visible_indices.push(idx);
                }
                QuickSearchItem::LineMatch { file_key, .. } => {
                    if !self.collapsed_files.contains(file_key) {
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
            if matches!(
                self.items.get(actual_idx),
                Some(QuickSearchItem::LineMatch { file_key: fk, .. }) if fk == file_key
            ) {
                indices_to_remove.push(visible_idx);
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
                matches!(item, QuickSearchItem::LineMatch { file_key: fk, .. } if fk == file_key)
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
        for (visible_idx, &actual_idx) in self.visible_indices.iter().enumerate() {
            if matches!(self.items.get(actual_idx), Some(QuickSearchItem::LineMatch { .. })) {
                self.visible_line_match_indices.push(visible_idx);
            }
        }
    }

    fn toggle_search_option(&mut self, option: SearchOptions) {
        self.search_options.toggle(option);
    }

    fn actual_index(&self, visible_index: usize) -> Option<usize> {
        self.visible_indices.get(visible_index).copied()
    }

    fn is_line_match_at_visible_index(&self, visible_index: usize) -> bool {
        self.visible_indices
            .get(visible_index)
            .and_then(|&actual_idx| self.items.get(actual_idx))
            .map_or(false, |item| {
                matches!(item, QuickSearchItem::LineMatch { .. })
            })
    }

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
                                window.focus(&qs.picker.focus_handle(cx));
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
    ) -> ListItem {
        let quick_search = self.quick_search.clone();
        let visible_ix = ix;
        let line_label = line_label.clone();
        let preview_text = preview_text.clone();

        ListItem::new(ix)
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(selected)
            .on_click({
                move |event, window, cx| {
                    cx.stop_propagation();
                    if event.click_count() >= 2 {
                        window.dispatch_action(menu::Confirm.boxed_clone(), cx);
                    } else if let Some(qs) = quick_search.upgrade() {
                        let preview_data = {
                            let modal = qs.read(cx);
                            let delegate = &modal.picker.read(cx).delegate;
                            delegate.actual_index(visible_ix).and_then(|idx| {
                                match delegate.items.get(idx) {
                                    Some(QuickSearchItem::LineMatch {
                                        project_path,
                                        line,
                                        match_ranges,
                                        ..
                                    }) => Some((project_path.clone(), *line, match_ranges.clone())),
                                    _ => None,
                                }
                            })
                        };

                        qs.update(cx, |modal, cx| {
                            window.focus(&modal.picker.focus_handle(cx));
                            modal.picker.update(cx, |picker, cx| {
                                picker.delegate.selected_index = visible_ix;
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
                        div().flex_1().min_w_0().overflow_hidden().child(
                            Label::new(preview_text)
                                .size(ui::LabelSize::Small)
                                .color(Color::Default)
                                .truncate(),
                        ),
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

        if let Some(found) = self.find_nearest_line_match(ix, going_down) {
            self.selected_index = found;
        } else if let Some(found) = self.find_nearest_line_match(ix, !going_down) {
            self.selected_index = found;
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
            Some(QuickSearchItem::LineMatch {
                project_path,
                line,
                match_ranges,
                ..
            }) => Some((project_path.clone(), *line, match_ranges.clone())),
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

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.current_query = query.clone();

        if query.is_empty() {
            self.items.clear();
            self.visible_indices.clear();
            self.pending_search_id = 0;
            self.match_count = 0;
            self.file_count = 0;
            self.is_limited = false;
            self.is_searching = false;
            self.regex_error = None;
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

        self.pending_search_id += 1;
        let search_id = self.pending_search_id;
        let project = self.project.clone();
        let search_options = self.search_options;
        let quick_search = self.quick_search.clone();

        cx.spawn_in(window, async move |picker, cx| {
            smol::Timer::after(Duration::from_millis(SEARCH_DEBOUNCE_MS)).await;

            let is_stale = picker
                .update(cx, |picker, _| {
                    picker.delegate.pending_search_id != search_id
                })
                .unwrap_or(true);
            if is_stale {
                return;
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

            let Some(project_search_results) = project
                .update(cx, |project, cx| project.search(search_query, cx))
                .log_err()
            else {
                return;
            };

            let mut results = SearchResults {
                items: Vec::with_capacity(MAX_LINE_MATCHES + MAX_LINE_MATCHES / 10),
                line_match_count: 0,
                file_count: 0,
                is_limited: false,
            };

            let mut project_search_results = pin!(project_search_results);
            while let Some(result) = project_search_results.next().await {
                match result {
                    project::search::SearchResult::Buffer { buffer, ranges } => {
                        if ranges.is_empty() {
                            continue;
                        }

                        let file_result = cx
                            .read_entity(&buffer, |buf, cx| extract_file_matches(buf, &ranges, cx))
                            .ok()
                            .flatten();

                        let Some(file_result) = file_result else {
                            continue;
                        };

                        results.items.push(QuickSearchItem::FileHeader {
                            file_name: file_result.file_name,
                            parent_path: file_result.parent_path,
                            file_key: file_result.file_key,
                        });
                        results.file_count += 1;

                        for match_data in file_result.matches {
                            results.items.push(QuickSearchItem::LineMatch {
                                project_path: match_data.project_path,
                                file_key: match_data.file_key,
                                line: match_data.line,
                                line_label: match_data.line_label,
                                preview_text: match_data.preview_text,
                                match_ranges: match_data.match_ranges,
                            });

                            results.line_match_count += 1;
                            if results.line_match_count >= MAX_LINE_MATCHES {
                                results.is_limited = true;
                                break;
                            }
                        }

                        if results.line_match_count >= MAX_LINE_MATCHES {
                            break;
                        }
                    }
                    project::search::SearchResult::LimitReached => {
                        results.is_limited = true;
                        break;
                    }
                }
            }

            let first_line_match = results.first_line_match();

            picker
                .update_in(cx, |picker, window, cx| {
                    if picker.delegate.pending_search_id == search_id {
                        picker.delegate.items = results.items;
                        picker.delegate.update_visible_indices();

                        let first_selectable = picker
                            .delegate
                            .visible_indices
                            .iter()
                            .position(|&actual_idx| {
                                matches!(
                                    picker.delegate.items.get(actual_idx),
                                    Some(QuickSearchItem::LineMatch { .. })
                                )
                            })
                            .unwrap_or(0);

                        picker.delegate.selected_index = first_selectable;
                        picker.delegate.match_count = results.line_match_count;
                        picker.delegate.file_count = results.file_count;
                        picker.delegate.is_limited = results.is_limited;
                        picker.delegate.is_searching = false;
                        cx.notify();

                        if let Some(quick_search) = quick_search.upgrade() {
                            quick_search.update(cx, |qs, cx| {
                                qs.update_preview(first_line_match, window, cx);
                            });
                        }
                    }
                })
                .log_err();
        })
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let actual_index = match self.actual_index(self.selected_index) {
            Some(idx) => idx,
            None => return,
        };

        let Some(QuickSearchItem::LineMatch {
            project_path, line, ..
        }) = self.items.get(actual_index)
        else {
            return;
        };

        let project_path = project_path.clone();
        let line = *line;

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
            QuickSearchItem::LineMatch {
                line_label,
                preview_text,
                ..
            } => Some(self.render_line_match(ix, selected, line_label, preview_text)),
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

        if self.match_count > 0 && !self.is_searching {
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

        None
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

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            theme::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
    }

    #[gpui::test]
    async fn test_quick_search_modal_creation(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                "file.rs": "fn main() {}\n",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = window.root(cx).unwrap();
        let mut cx = VisualTestContext::from_window(*window.deref(), cx);

        let quick_search = cx.new_window_entity({
            let weak_workspace = workspace.downgrade();
            move |window, cx| QuickSearchModal::new(weak_workspace, project, None, window, cx)
        });

        quick_search.update(&mut cx, |modal, cx| {
            assert!(modal.preview_editor.is_none());
            assert!(modal.preview_buffer.is_none());
            assert_eq!(modal.picker.read(cx).delegate.items.len(), 0);
        });
    }

    #[gpui::test]
    async fn test_quick_search_empty_query_clears_results(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                "file.rs": "fn test() {}\n",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = window.root(cx).unwrap();
        let mut cx = VisualTestContext::from_window(*window.deref(), cx);

        let quick_search = cx.new_window_entity({
            let weak_workspace = workspace.downgrade();
            move |window, cx| QuickSearchModal::new(weak_workspace, project, None, window, cx)
        });

        quick_search.update_in(&mut cx, |modal, window, cx| {
            modal.picker.update(cx, |picker, cx| {
                picker.set_query("test", window, cx);
            });
        });

        quick_search.update(&mut cx, |modal, cx| {
            assert_eq!(modal.picker.read(cx).delegate.pending_search_id, 1);
        });

        quick_search.update_in(&mut cx, |modal, window, cx| {
            modal.picker.update(cx, |picker, cx| {
                picker.set_query("", window, cx);
            });
        });

        cx.background_executor.run_until_parked();

        quick_search.update(&mut cx, |modal, cx| {
            let delegate = &modal.picker.read(cx).delegate;
            assert_eq!(delegate.items.len(), 0, "Empty query should clear results");
            assert_eq!(
                delegate.pending_search_id, 0,
                "Empty query should reset search id"
            );
        });
    }

    #[gpui::test]
    fn test_quick_search_item_types(cx: &mut TestAppContext) {
        init_test(cx);

        let header = QuickSearchItem::FileHeader {
            file_name: "test.rs".into(),
            parent_path: "src".into(),
            file_key: "src/test.rs".into(),
        };
        assert!(matches!(header, QuickSearchItem::FileHeader { .. }));

        cx.update(|_cx| {
            let line_match = QuickSearchItem::LineMatch {
                project_path: ProjectPath {
                    worktree_id: project::WorktreeId::from_usize(0),
                    path: util::rel_path::rel_path("src/test.rs").into(),
                },
                file_key: "src/test.rs".into(),
                line: 0,
                line_label: "1".into(),
                preview_text: "fn test()".into(),
                match_ranges: Arc::new(Vec::new()),
            };
            assert!(matches!(line_match, QuickSearchItem::LineMatch { .. }));
        });
    }

    #[gpui::test]
    async fn test_quick_search_no_results_for_nonexistent_query(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                "file.rs": "fn main() {}\n",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = window.root(cx).unwrap();
        let mut cx = VisualTestContext::from_window(*window.deref(), cx);

        let quick_search = cx.new_window_entity({
            let weak_workspace = workspace.downgrade();
            move |window, cx| QuickSearchModal::new(weak_workspace, project, None, window, cx)
        });

        quick_search.update_in(&mut cx, |modal, window, cx| {
            modal.picker.update(cx, |picker, cx| {
                picker.set_query("nonexistent_string_xyz_123", window, cx);
            });
        });

        cx.executor().advance_clock(Duration::from_millis(150));
        cx.background_executor.run_until_parked();

        quick_search.update(&mut cx, |modal, cx| {
            let delegate = &modal.picker.read(cx).delegate;
            assert_eq!(
                delegate.items.len(),
                0,
                "Should have no results for non-matching query"
            );
        });
    }

    #[gpui::test]
    async fn test_quick_search_query_updates_search_id(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                "file.rs": "fn hello() {}\nfn world() {}\n",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = window.root(cx).unwrap();
        let mut cx = VisualTestContext::from_window(*window.deref(), cx);

        let quick_search = cx.new_window_entity({
            let weak_workspace = workspace.downgrade();
            move |window, cx| QuickSearchModal::new(weak_workspace, project, None, window, cx)
        });

        quick_search.update(&mut cx, |modal, cx| {
            assert_eq!(modal.picker.read(cx).delegate.pending_search_id, 0);
        });

        quick_search.update_in(&mut cx, |modal, window, cx| {
            modal.picker.update(cx, |picker, cx| {
                picker.set_query("hello", window, cx);
            });
        });

        quick_search.update(&mut cx, |modal, cx| {
            assert_eq!(
                modal.picker.read(cx).delegate.pending_search_id,
                1,
                "First search should have id 1"
            );
        });

        quick_search.update_in(&mut cx, |modal, window, cx| {
            modal.picker.update(cx, |picker, cx| {
                picker.set_query("world", window, cx);
            });
        });

        quick_search.update(&mut cx, |modal, cx| {
            assert_eq!(
                modal.picker.read(cx).delegate.pending_search_id,
                2,
                "Second search should have id 2"
            );
        });
    }

    #[gpui::test]
    fn test_quick_search_collapse_expand_files(cx: &mut TestAppContext) {
        init_test(cx);

        cx.update(|_cx| {
            let items = [
                QuickSearchItem::FileHeader {
                    file_name: "test.rs".into(),
                    parent_path: "src".into(),
                    file_key: "src/test.rs".into(),
                },
                QuickSearchItem::LineMatch {
                    project_path: ProjectPath {
                        worktree_id: project::WorktreeId::from_usize(0),
                        path: util::rel_path::rel_path("src/test.rs").into(),
                    },
                    file_key: "src/test.rs".into(),
                    line: 0,
                    line_label: "1".into(),
                    preview_text: "fn test()".into(),
                    match_ranges: Arc::new(Vec::new()),
                },
                QuickSearchItem::LineMatch {
                    project_path: ProjectPath {
                        worktree_id: project::WorktreeId::from_usize(0),
                        path: util::rel_path::rel_path("src/test.rs").into(),
                    },
                    file_key: "src/test.rs".into(),
                    line: 1,
                    line_label: "2".into(),
                    preview_text: "fn other()".into(),
                    match_ranges: Arc::new(Vec::new()),
                },
            ];

            let mut visible_indices = Vec::new();
            let mut collapsed_files: HashSet<SharedString> = HashSet::default();

            for (idx, item) in items.iter().enumerate() {
                match item {
                    QuickSearchItem::FileHeader { .. } => {
                        visible_indices.push(idx);
                    }
                    QuickSearchItem::LineMatch { file_key, .. } => {
                        if !collapsed_files.contains(file_key) {
                            visible_indices.push(idx);
                        }
                    }
                }
            }

            assert_eq!(visible_indices.len(), 3, "All 3 items should be visible");
            assert_eq!(visible_indices, vec![0, 1, 2]);

            let file_key: SharedString = "src/test.rs".into();
            collapsed_files.insert(file_key.clone());
            visible_indices.clear();
            for (idx, item) in items.iter().enumerate() {
                match item {
                    QuickSearchItem::FileHeader { .. } => {
                        visible_indices.push(idx);
                    }
                    QuickSearchItem::LineMatch { file_key, .. } => {
                        if !collapsed_files.contains(file_key) {
                            visible_indices.push(idx);
                        }
                    }
                }
            }

            assert_eq!(
                visible_indices.len(),
                1,
                "Only file header should be visible after collapse"
            );
            assert_eq!(visible_indices, vec![0]);

            collapsed_files.remove(&file_key);
            visible_indices.clear();
            for (idx, item) in items.iter().enumerate() {
                match item {
                    QuickSearchItem::FileHeader { .. } => {
                        visible_indices.push(idx);
                    }
                    QuickSearchItem::LineMatch { file_key, .. } => {
                        if !collapsed_files.contains(file_key) {
                            visible_indices.push(idx);
                        }
                    }
                }
            }

            assert_eq!(
                visible_indices.len(),
                3,
                "All items should be visible after expand"
            );
            assert_eq!(visible_indices, vec![0, 1, 2]);
        });
    }
}
