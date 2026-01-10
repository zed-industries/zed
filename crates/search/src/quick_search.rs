use collections::HashSet;
use futures::StreamExt;
use std::ops::Range;
use std::pin::pin;
use std::sync::Arc;
use std::time::Duration;

use editor::{Editor, EditorEvent};
use gpui::{
    Action, App, AsyncApp, Context, DismissEvent, DragMoveEvent, Entity, EventEmitter, FocusHandle,
    Focusable, Global, HighlightStyle, KeyBinding, KeyContext, ParentElement, Render, Styled,
    StyledText, Subscription, Task, WeakEntity, Window, actions, px, relative,
};
use language::Buffer;
use menu;
use multi_buffer::{ExcerptRange, MultiBuffer};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::SearchResults;
use project::search::{SearchQuery, SearchResult};
use project::{Project, ProjectPath};
use crate::{SearchOption, SearchOptions};
use editor::EditorSettings;
use settings::Settings;
use text::{Anchor, Point, ToOffset};
use theme::ActiveTheme;
use ui::Divider;
use ui::{
    Icon, IconButton, IconName, ListItem, ListItemSpacing, Tooltip, highlight_ranges, prelude::*,
};
use util::{ResultExt, paths::PathMatcher};
use workspace::{ModalView, Workspace};
pub use zed_actions::quick_search::Toggle;

actions!(quick_search, [ReplaceNext, ReplaceAll, ToggleFilters]);

const SEARCH_DEBOUNCE_MS: u64 = 100;

struct SearchMatchHighlight;

/// Global state for storing the recent search query.
struct RecentSearchState {
    last_query: String,
}

impl Global for RecentSearchState {}

fn get_recent_query(cx: &App) -> Option<String> {
    cx.try_global::<RecentSearchState>().and_then(|state| {
        if state.last_query.is_empty() {
            None
        } else {
            Some(state.last_query.clone())
        }
    })
}

fn save_recent_query(query: &str, cx: &mut App) {
    cx.set_global(RecentSearchState {
        last_query: query.to_string(),
    });
}

/// Initialize the quick_search module.
pub fn init(cx: &mut App) {
    cx.observe_new(QuickSearch::register).detach();
    cx.bind_keys([
        KeyBinding::new("escape", menu::Cancel, Some("QuickSearch")),
        KeyBinding::new("enter", ReplaceNext, Some("QuickSearch && in_replace")),
        KeyBinding::new(
            "cmd-enter",
            ReplaceAll,
            Some("QuickSearch && in_replace"),
        ),
        KeyBinding::new("alt-cmd-f", ToggleFilters, Some("QuickSearch")),
    ]);
}

#[derive(Clone)]
pub struct SearchMatch {
    pub path: ProjectPath,
    pub buffer: Entity<Buffer>,
    pub anchor_ranges: Vec<Range<Anchor>>,
    pub ranges: Vec<Range<usize>>,
    pub relative_ranges: Vec<Range<usize>>,
    pub line_text: String,
    pub line_number: u32,
}

pub struct QuickSearch {
    picker: Entity<Picker<QuickSearchDelegate>>,
    preview_editor: Entity<Editor>,
    replace_editor: Entity<Editor>,
    focus_handle: FocusHandle,
    offset: gpui::Point<Pixels>,
    _subscriptions: Vec<Subscription>,
    _autosave_task: Option<Task<()>>,
}

#[derive(Clone, Copy)]
struct QuickSearchDrag {
    mouse_start: gpui::Point<Pixels>,
    offset_start: gpui::Point<Pixels>,
}

struct DragPreview;

impl Render for DragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

impl ModalView for QuickSearch {}

impl EventEmitter<DismissEvent> for QuickSearch {}

impl Focusable for QuickSearch {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl QuickSearch {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(|workspace, _: &Toggle, window, cx| {
            let project = workspace.project().clone();
            let weak_workspace = cx.entity().downgrade();
            workspace.toggle_modal(window, cx, |window, cx| {
                QuickSearch::new(weak_workspace, project, window, cx)
            });
        });
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let capability = project.read(cx).capability();
        let preview_editor = cx.new(|cx| {
            let multi_buffer = cx.new(|_| MultiBuffer::new(capability));
            let mut editor =
                Editor::for_multibuffer(multi_buffer, Some(project.clone()), window, cx);
            editor.set_show_breadcrumbs(false, cx);
            editor
        });

        let replace_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Replace in project…", window, cx);
            editor
        });

        let include_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Include: e.g. src/**/*.rs", window, cx);
            editor
        });

        let exclude_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Exclude: e.g. vendor/*, *.lock", window, cx);
            editor
        });

        let delegate = QuickSearchDelegate::new(
            workspace,
            project,
            preview_editor.clone(),
            replace_editor.clone(),
            include_editor.clone(),
            exclude_editor.clone(),
            cx,
        );
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx).modal(false));

        // Restore recent query if available
        if let Some(recent_query) = get_recent_query(cx) {
            picker.update(cx, |picker, cx| {
                picker.set_query(recent_query, window, cx);
            });
        }

        let focus_handle = cx.focus_handle();

        let subscriptions = vec![
            cx.subscribe_in(&picker, window, |_, _, _: &DismissEvent, _, cx| {
                cx.emit(DismissEvent);
            }),
            cx.subscribe_in(
                &include_editor,
                window,
                |this, _, event: &EditorEvent, window, cx| {
                    if matches!(event, EditorEvent::Edited { .. }) {
                        this.picker.update(cx, |picker, cx| {
                            picker.refresh(window, cx);
                        });
                    }
                },
            ),
            cx.subscribe_in(
                &exclude_editor,
                window,
                |this, _, event: &EditorEvent, window, cx| {
                    if matches!(event, EditorEvent::Edited { .. }) {
                        this.picker.update(cx, |picker, cx| {
                            picker.refresh(window, cx);
                        });
                    }
                },
            ),
            cx.on_focus_out(&focus_handle, window, |this, _, window, cx| {
                if window.is_window_active() && !this.focus_handle.contains_focused(window, cx) {
                    cx.emit(DismissEvent);
                }
            }),
            cx.subscribe_in(
                &preview_editor,
                window,
                |this, _, event: &EditorEvent, window, cx| {
                    if matches!(event, EditorEvent::Edited { .. }) {
                        this._autosave_task = Some(cx.spawn_in(window, async move |this, cx| {
                            cx.background_executor()
                                .timer(Duration::from_millis(500))
                                .await;

                            this.update_in(cx, |this, _window, cx| {
                                let delegate = &this.picker.read(cx).delegate;
                                if let Some(m) = delegate.matches.get(delegate.selected_index) {
                                    let buffer = m.buffer.clone();
                                    let project = delegate.project.clone();
                                    let mut buffers = HashSet::default();
                                    buffers.insert(buffer);
                                    project
                                        .update(cx, |p, cx| p.save_buffers(buffers, cx))
                                        .detach_and_log_err(cx);
                                }
                            }).log_err();
                        }));
                    }
                },
            ),
        ];

        Self {
            picker,
            preview_editor,
            replace_editor,
            focus_handle,
            offset: gpui::Point::default(),
            _subscriptions: subscriptions,
            _autosave_task: None,
        }
    }

    fn replacement_text(&self, cx: &App) -> String {
        self.replace_editor.read(cx).text(cx)
    }

    fn replace_next(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let delegate = &self.picker.read(cx).delegate;
        let Some(selected_match) = delegate.matches.get(delegate.selected_index) else {
            return;
        };

        let replacement = self.replacement_text(cx);

        if selected_match.anchor_ranges.is_empty() {
            return;
        }

        let buffer = selected_match.buffer.clone();
        let project = delegate.project.clone();
        let anchor_range = selected_match.anchor_ranges[0].clone();

        buffer.update(cx, |buffer, cx| {
            // Convert anchors to offsets at edit time
            let snapshot = buffer.snapshot();
            let range =
                anchor_range.start.to_offset(&snapshot)..anchor_range.end.to_offset(&snapshot);
            buffer.edit([(range, replacement.as_str())], None, cx);
        });

        // Save the buffer so changes persist without manual Cmd+S
        project
            .update(cx, |project, cx| {
                let mut buffers = HashSet::default();
                buffers.insert(buffer);
                project.save_buffers(buffers, cx)
            })
            .detach_and_log_err(cx);

        // Refresh the search and move to next match
        self.picker.update(cx, |picker, cx| {
            picker.refresh(window, cx);
        });
    }

    fn replace_all(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let replacement = self.replacement_text(cx);

        // Collect all matches grouped by buffer
        let delegate = &self.picker.read(cx).delegate;
        let matches: Vec<_> = delegate.matches.clone();
        let project = delegate.project.clone();

        // Group anchor ranges by buffer
        let mut buffer_edits: std::collections::HashMap<
            gpui::EntityId,
            (Entity<Buffer>, Vec<Range<Anchor>>),
        > = std::collections::HashMap::new();

        for m in &matches {
            let buffer_id = m.buffer.entity_id();
            for anchor_range in &m.anchor_ranges {
                buffer_edits
                    .entry(buffer_id)
                    .or_insert_with(|| (m.buffer.clone(), Vec::new()))
                    .1
                    .push(anchor_range.clone());
            }
        }

        // Collect buffers that were edited for saving
        let mut edited_buffers: HashSet<Entity<Buffer>> = HashSet::default();

        // Apply all edits for each buffer
        for (_, (buffer, mut anchor_ranges)) in buffer_edits {
            buffer.update(cx, |buf, cx| {
                // Sort anchors by position descending (apply from end to start)
                let snapshot = buf.snapshot();
                anchor_ranges.sort_by(|a, b| {
                    b.start
                        .to_offset(&snapshot)
                        .cmp(&a.start.to_offset(&snapshot))
                });

                // Apply each edit, getting fresh offset each time
                for anchor_range in anchor_ranges {
                    let snapshot = buf.snapshot();
                    let range = anchor_range.start.to_offset(&snapshot)
                        ..anchor_range.end.to_offset(&snapshot);
                    buf.edit([(range, replacement.as_str())], None, cx);
                }
            });
            edited_buffers.insert(buffer);
        }

        // Save all edited buffers
        if !edited_buffers.is_empty() {
            project
                .update(cx, |project, cx| project.save_buffers(edited_buffers, cx))
                .detach_and_log_err(cx);
        }

        // Refresh the search
        self.picker.update(cx, |picker, cx| {
            picker.refresh(window, cx);
        });
    }
}

impl Render for QuickSearch {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let modal_width = rems(50.).to_pixels(window.rem_size());

        let delegate = &self.picker.read(cx).delegate;
        let match_count = delegate.matches.len();
        let file_count = delegate
            .matches
            .iter()
            .map(|m| &m.path)
            .collect::<HashSet<_>>()
            .len();
        let search_in_progress = delegate.search_in_progress;
        let replace_enabled = delegate.replace_enabled;
        let filters_enabled = delegate.filters_enabled;

        let has_matches = match_count > 0;

        let focus_handle = self.picker.focus_handle(cx);
        let in_replace = self.replace_editor.focus_handle(cx).is_focused(window);

        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("QuickSearch");
        if in_replace {
            key_context.add("in_replace");
        }

        v_flex()
            .key_context(key_context)
            .id("quick-search")
            .track_focus(&focus_handle)
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| {
                cx.emit(DismissEvent);
            }))
            .on_action(cx.listener(|this, _: &ReplaceNext, window, cx| {
                this.replace_next(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ReplaceAll, window, cx| {
                this.replace_all(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleFilters, window, cx| {
                this.picker.update(cx, |picker, cx| {
                    picker.delegate.filters_enabled = !picker.delegate.filters_enabled;
                    picker.refresh(window, cx);
                });
            }))
            .m_4()
            .relative()
            .top(self.offset.y)
            .left(self.offset.x)
            .w(modal_width)
            .bg(cx.theme().colors().elevated_surface_background)
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .shadow_lg()
            .on_drag(
                QuickSearchDrag {
                    mouse_start: window.mouse_position(),
                    offset_start: self.offset,
                },
                |_, _, _, cx| cx.new(|_| DragPreview),
            )
            .on_drag_move::<QuickSearchDrag>(cx.listener(
                |this, event: &DragMoveEvent<QuickSearchDrag>, _window, cx| {
                    let drag = event.drag(cx);
                    this.offset = drag.offset_start + (event.event.position - drag.mouse_start);
                    cx.notify();
                },
            ))
            .child(
                h_flex()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(Label::new("Quick Search").size(LabelSize::Default))
                            .when(search_in_progress, |this| {
                                this.child(
                                    Label::new(format!(
                                        "Searching... {} matches in {} files",
                                        match_count, file_count
                                    ))
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                                )
                            })
                            .when(!search_in_progress && has_matches, |this| {
                                this.child(
                                    Label::new(format!(
                                        "{} matches in {} files",
                                        match_count, file_count
                                    ))
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                                )
                            }),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .items_center()
                            // Replace toggle button
                            .child(
                                IconButton::new("replace-toggle", IconName::Replace)
                                    .size(ButtonSize::Compact)
                                    .toggle_state(replace_enabled)
                                    .tooltip(Tooltip::text("Toggle Replace"))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.picker.update(cx, |picker, _| {
                                            picker.delegate.replace_enabled =
                                                !picker.delegate.replace_enabled;
                                        });
                                        cx.notify();
                                    })),
                            )
                            // Filters toggle button
                            .child(
                                IconButton::new("filters-toggle", IconName::Filter)
                                    .size(ButtonSize::Compact)
                                    .toggle_state(filters_enabled)
                                    .tooltip(|_window, cx| {
                                        Tooltip::for_action("Toggle Filters", &ToggleFilters, cx)
                                    })
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.picker.update(cx, |picker, cx| {
                                            picker.delegate.filters_enabled =
                                                !picker.delegate.filters_enabled;
                                            picker.refresh(window, cx);
                                        });
                                    })),
                            )
                            // Close button
                            .child(
                                div()
                                    .id("close-button")
                                    .cursor_pointer()
                                    .rounded_sm()
                                    .p_1()
                                    .hover(|style| style.bg(cx.theme().colors().element_hover))
                                    .on_click(cx.listener(|_, _, _window, cx| {
                                        cx.emit(DismissEvent);
                                    }))
                                    .child(
                                        Icon::new(IconName::Close)
                                            .size(IconSize::Small)
                                            .color(Color::Muted),
                                    ),
                            ),
                    ),
            )
            .child(
                div()
                    .max_h(px(250.))
                    .overflow_hidden()
                    .child(self.picker.clone()),
            )
            .when(has_matches, |this| {
                let delegate = &self.picker.read(cx).delegate;
                let selected_match = delegate.matches.get(delegate.selected_index);

                let preview_header = selected_match.map(|m| {
                    let path = &m.path.path;
                    let file_name = path
                        .file_name()
                        .map(|name| name.to_string())
                        .unwrap_or_default();
                    let directory = path
                        .parent()
                        .map(|path| path.as_std_path().to_string_lossy().to_string())
                        .unwrap_or_default();

                    h_flex()
                        .px_2()
                        .py_1()
                        .gap_2()
                        .border_t_1()
                        .border_b_1()
                        .border_color(cx.theme().colors().border)
                        .bg(cx.theme().colors().editor_background)
                        .child(Label::new(file_name).size(LabelSize::Small))
                        .child(
                            Label::new(directory)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                });

                this.child(
                    v_flex().children(preview_header).child(
                        div()
                            .h(px(200.))
                            .overflow_hidden()
                            .child(self.preview_editor.clone()),
                    ),
                )
            })
    }
}

pub struct QuickSearchDelegate {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    preview_editor: Entity<Editor>,
    replace_editor: Entity<Editor>,
    include_editor: Entity<Editor>,
    exclude_editor: Entity<Editor>,
    replace_enabled: bool,
    filters_enabled: bool,
    matches: Vec<SearchMatch>,
    selected_index: usize,
    cancel_flag: Arc<std::sync::atomic::AtomicBool>,
    /// Set when selection changes via click, cleared on confirm. Used to detect click vs Enter.
    has_changed_selected_index: bool,
    last_confirm_time: Option<std::time::Instant>,
    search_options: SearchOptions,
    search_in_progress: bool,
}

impl QuickSearchDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        preview_editor: Entity<Editor>,
        replace_editor: Entity<Editor>,
        include_editor: Entity<Editor>,
        exclude_editor: Entity<Editor>,
        cx: &App,
    ) -> Self {
        Self {
            workspace,
            project,
            preview_editor,
            replace_editor,
            include_editor,
            exclude_editor,
            replace_enabled: false,
            filters_enabled: false,
            matches: Vec::new(),
            selected_index: 0,
            cancel_flag: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            has_changed_selected_index: false,
            last_confirm_time: None,
            search_options: SearchOptions::from_settings(&EditorSettings::get_global(cx).search),
            search_in_progress: false,
        }
    }

    fn update_preview(&self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(selected_match) = self.matches.get(self.selected_index) else {
            self.preview_editor.update(cx, |editor, cx| {
                editor.buffer().update(cx, |multi_buffer, cx| {
                    multi_buffer.clear(cx);
                });
            });
            return;
        };

        let buffer = selected_match.buffer.clone();
        let match_row = selected_match.line_number.saturating_sub(1);
        let ranges = selected_match.ranges.clone();
        let anchor_ranges = selected_match.anchor_ranges.clone();

        self.preview_editor.update(cx, |editor, cx| {
            let multi_buffer = editor.buffer().clone();
            let buffer_snapshot = buffer.read(cx);
            let max_point = buffer_snapshot.max_point();

            let context_start = buffer_snapshot.anchor_before(Point::new(0, 0));
            let context_end = buffer_snapshot.anchor_after(max_point);

            let primary_range = if let Some(range) = ranges.first() {
                let start = buffer_snapshot.anchor_before(range.start);
                let end = buffer_snapshot.anchor_after(range.end);
                start..end
            } else {
                let start = buffer_snapshot.anchor_before(Point::new(match_row, 0));
                let end = buffer_snapshot
                    .anchor_after(Point::new(match_row, buffer_snapshot.line_len(match_row)));
                start..end
            };

            multi_buffer.update(cx, |multi_buffer, cx| {
                multi_buffer.clear(cx);
                multi_buffer.push_excerpts(
                    buffer.clone(),
                    [ExcerptRange {
                        context: context_start..context_end,
                        primary: primary_range,
                    }],
                    cx,
                );
            });

            // Highlight all matches with yellow background (like IntelliJ)
            let multi_buffer_snapshot = multi_buffer.read(cx);
            if let Some(excerpt_id) = multi_buffer_snapshot.excerpt_ids().first().copied() {
                let highlight_ranges: Vec<_> = anchor_ranges
                    .iter()
                    .map(|range| {
                        editor::Anchor::range_in_buffer(excerpt_id, range.clone())
                    })
                    .collect();

                editor.highlight_background::<SearchMatchHighlight>(
                    &highlight_ranges,
                    |_, theme| theme.colors().search_match_background,
                    cx,
                );
            }

            if let Some(range) = ranges.first() {
                let start = multi_buffer::MultiBufferOffset(range.start);
                let end = multi_buffer::MultiBufferOffset(range.end);
                editor.change_selections(Default::default(), window, cx, |s| {
                    s.select_ranges([start..end]);
                });
            } else {
                editor.go_to_singleton_buffer_point(Point::new(match_row, 0), window, cx);
            }
        });
    }

    fn parse_path_matches(&self, text: String, cx: &App) -> PathMatcher {
        let queries: Vec<String> = text
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
            .collect();

        if queries.is_empty() {
            return PathMatcher::default();
        }

        let path_style = self.project.read(cx).path_style(cx);
        PathMatcher::new(&queries, path_style).unwrap_or_default()
    }

    fn build_search_query(&self, query: &str, cx: &Context<Picker<Self>>) -> Option<SearchQuery> {
        if query.is_empty() {
            return None;
        }

        let files_to_include = if self.filters_enabled {
            let include_text = self.include_editor.read(cx).text(cx);
            self.parse_path_matches(include_text, cx)
        } else {
            PathMatcher::default()
        };

        let files_to_exclude = if self.filters_enabled {
            let exclude_text = self.exclude_editor.read(cx).text(cx);
            self.parse_path_matches(exclude_text, cx)
        } else {
            PathMatcher::default()
        };

        let match_full_paths = self.project.read(cx).visible_worktrees(cx).count() > 1;
        let case_sensitive = self.search_options.contains(SearchOptions::CASE_SENSITIVE);
        let whole_word = self.search_options.contains(SearchOptions::WHOLE_WORD);

        if self.search_options.contains(SearchOptions::REGEX) {
            SearchQuery::regex(
                query,
                whole_word,
                case_sensitive,
                false,
                false,
                files_to_include,
                files_to_exclude,
                match_full_paths,
                None,
            )
            .ok()
        } else {
            SearchQuery::text(
                query,
                whole_word,
                case_sensitive,
                false,
                files_to_include,
                files_to_exclude,
                match_full_paths,
                None,
            )
            .ok()
        }
    }

    fn process_search_result(
        buffer: &Entity<Buffer>,
        ranges: &[Range<Anchor>],
        cx: &AsyncApp,
    ) -> Vec<SearchMatch> {
        if ranges.is_empty() {
            return Vec::new();
        }

        let buffer_data = buffer.read_with(cx, |buf, cx| {
            let file = buf.file();
            let path = file.map(|f| ProjectPath {
                worktree_id: f.worktree_id(cx),
                path: f.path().clone(),
            });
            let text = buf.text();

            let mut result = Vec::new();
            for range in ranges {
                let start_offset: usize = buf.summary_for_anchor(&range.start);
                let end_offset: usize = buf.summary_for_anchor(&range.end);
                let match_row = buf.offset_to_point(start_offset).row;
                let line_number = match_row + 1;
                let line_start = text[..start_offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
                let line_end = text[start_offset..]
                    .find('\n')
                    .map(|i| start_offset + i)
                    .unwrap_or(text.len());
                let line_text = text[line_start..line_end].to_string();

                let relative_start = start_offset - line_start;
                let relative_end = end_offset - line_start;
                let relative_range = relative_start..relative_end;

                if let Some(path) = &path {
                    result.push((
                        path.clone(),
                        range.clone(),
                        start_offset..end_offset,
                        relative_range,
                        line_text,
                        line_number,
                        buffer.clone(),
                    ));
                }
            }
            result
        });

        buffer_data
            .into_iter()
            .map(
                |(path, anchor_range, range, relative_range, line_text, line_number, buffer)| {
                    SearchMatch {
                        path,
                        buffer,
                        anchor_ranges: vec![anchor_range],
                        ranges: vec![range],
                        relative_ranges: vec![relative_range],
                        line_text,
                        line_number,
                    }
                },
            )
            .collect()
    }
}

impl PickerDelegate for QuickSearchDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search all files...".into()
    }

    fn render_editor(
        &self,
        editor: &Entity<Editor>,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        let search_options = self.search_options;

        v_flex()
            // Search input row with toggle buttons
            .child(
                h_flex()
                    .flex_none()
                    .h_9()
                    .px_2p5()
                    .gap_1()
                    .child(div().flex_1().overflow_hidden().child(editor.clone()))
                    .child(
                        h_flex()
                            .flex_none()
                            .gap_0p5()
                            .child(
                                IconButton::new("case-sensitive", SearchOption::CaseSensitive.icon())
                                    .size(ButtonSize::Compact)
                                    .toggle_state(
                                        search_options.contains(SearchOptions::CASE_SENSITIVE),
                                    )
                                    .tooltip(Tooltip::text(SearchOption::CaseSensitive.label()))
                                    .on_click(cx.listener(|picker, _, window, cx| {
                                        picker
                                            .delegate
                                            .search_options
                                            .toggle(SearchOptions::CASE_SENSITIVE);
                                        picker.refresh(window, cx);
                                    })),
                            )
                            .child(
                                IconButton::new("whole-word", SearchOption::WholeWord.icon())
                                    .size(ButtonSize::Compact)
                                    .toggle_state(
                                        search_options.contains(SearchOptions::WHOLE_WORD),
                                    )
                                    .tooltip(Tooltip::text(SearchOption::WholeWord.label()))
                                    .on_click(cx.listener(|picker, _, window, cx| {
                                        picker
                                            .delegate
                                            .search_options
                                            .toggle(SearchOptions::WHOLE_WORD);
                                        picker.refresh(window, cx);
                                    })),
                            )
                            .child(
                                IconButton::new("regex", SearchOption::Regex.icon())
                                    .size(ButtonSize::Compact)
                                    .toggle_state(search_options.contains(SearchOptions::REGEX))
                                    .on_click(cx.listener(|picker, _, window, cx| {
                                        picker.delegate.search_options.toggle(SearchOptions::REGEX);
                                        picker.refresh(window, cx);
                                    }))
                                    .tooltip(Tooltip::text(SearchOption::Regex.label())),
                            ),
                    ),
            )
            // Replace input row (shown when replace_enabled)
            .when(self.replace_enabled, |this| {
                this.child(Divider::horizontal()).child(
                    h_flex()
                        .flex_none()
                        .h_9()
                        .px_2p5()
                        .gap_1()
                        .child(
                            div()
                                .flex_1()
                                .overflow_hidden()
                                .child(self.replace_editor.clone()),
                        )
                        .child(
                            h_flex()
                                .flex_none()
                                .gap_0p5()
                                .child(
                                    IconButton::new("replace-next", IconName::ReplaceNext)
                                        .shape(ui::IconButtonShape::Square)
                                        .tooltip(Tooltip::text("Replace Next (Enter)"))
                                        .on_click(|_, window, cx| {
                                            window.dispatch_action(ReplaceNext.boxed_clone(), cx);
                                        }),
                                )
                                .child(
                                    IconButton::new("replace-all", IconName::ReplaceAll)
                                        .shape(ui::IconButtonShape::Square)
                                        .tooltip(Tooltip::text("Replace All (Cmd+Enter)"))
                                        .on_click(|_, window, cx| {
                                            window.dispatch_action(ReplaceAll.boxed_clone(), cx);
                                        }),
                                ),
                        ),
                )
            })
            // Filters row (shown when filters_enabled)
            .when(self.filters_enabled, |this| {
                this.child(Divider::horizontal()).child(
                    h_flex()
                        .flex_none()
                        .h_9()
                        .px_2p5()
                        .gap_2()
                        .child(
                            h_flex()
                                .flex_1()
                                .gap_1()
                                .child(
                                    Label::new("Include:")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .overflow_hidden()
                                        .child(self.include_editor.clone()),
                                ),
                        )
                        .child(
                            h_flex()
                                .flex_1()
                                .gap_1()
                                .child(
                                    Label::new("Exclude:")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .overflow_hidden()
                                        .child(self.exclude_editor.clone()),
                                ),
                        ),
                )
            })
            // Divider after inputs
            .when(
                self.editor_position() == PickerEditorPosition::Start,
                |this| this.child(Divider::horizontal()),
            )
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
        self.has_changed_selected_index = true;
        self.update_preview(window, cx);
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        if !query.is_empty() {
            save_recent_query(&query, cx);
        }

        self.cancel_flag
            .store(true, std::sync::atomic::Ordering::SeqCst);
        self.cancel_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));

        let cancel_flag = self.cancel_flag.clone();

        let Some(search_query) = self.build_search_query(&query, cx) else {
            self.matches.clear();
            self.selected_index = 0;
            self.search_in_progress = false;
            cx.notify();
            return Task::ready(());
        };

        let search_results = self
            .project
            .update(cx, |project, cx| project.search(search_query, cx));

        self.matches.clear();
        self.selected_index = 0;
        self.search_in_progress = true;
        cx.notify();

        cx.spawn_in(window, async move |picker, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(SEARCH_DEBOUNCE_MS))
                .await;

            if cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
                return;
            }

            let SearchResults { rx, _task_handle } = search_results;
            let mut results_stream = pin!(rx.ready_chunks(256));

            while let Some(results) = results_stream.next().await {
                if cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }

                let mut batch_matches = Vec::new();
                let mut limit_reached = false;

                for result in results {
                    match result {
                        SearchResult::Buffer { buffer, ranges } => {
                            let matches = QuickSearchDelegate::process_search_result(
                                &buffer, &ranges, cx,
                            );
                            batch_matches.extend(matches);
                        }
                        SearchResult::LimitReached => {
                            limit_reached = true;
                        }
                    }
                }

                picker
                    .update_in(cx, |picker, window, cx| {
                        let delegate = &mut picker.delegate;
                        delegate.matches.extend(batch_matches);

                        if delegate.selected_index >= delegate.matches.len()
                            && !delegate.matches.is_empty()
                        {
                            delegate.selected_index = 0;
                        }

                        if delegate.matches.len() == delegate.selected_index + 1
                            || delegate.selected_index == 0
                        {
                            delegate.update_preview(window, cx);
                        }

                        cx.notify();
                    })
                    .log_err();

                if limit_reached {
                    break;
                }

                smol::future::yield_now().await;
            }

            picker
                .update_in(cx, |picker, _window, cx| {
                    picker.delegate.search_in_progress = false;
                    cx.notify();
                })
                .log_err();
        })
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        // If replace editor is focused, handle replace actions instead of confirm
        let in_replace =
            self.replace_enabled && self.replace_editor.focus_handle(cx).is_focused(window);

        if in_replace {
            if secondary {
                // Cmd+Enter: Replace All
                window.dispatch_action(ReplaceAll.boxed_clone(), cx);
            } else {
                // Enter: Replace Next
                window.dispatch_action(ReplaceNext.boxed_clone(), cx);
            }
            return;
        }

        // Check if this confirm was triggered by a click (selection just changed)
        // or by Enter key (no selection change)
        if self.has_changed_selected_index {
            self.has_changed_selected_index = false;

            // For clicks, require double-click (two confirms within 300ms)
            let now = std::time::Instant::now();
            let is_double_click = self
                .last_confirm_time
                .map(|t| now.duration_since(t).as_millis() < 300)
                .unwrap_or(false);
            self.last_confirm_time = Some(now);

            if !is_double_click {
                return;
            }
        }
        // For Enter key (not a click), proceed immediately

        let Some(selected_match) = self.matches.get(self.selected_index) else {
            return;
        };

        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let path = selected_match.path.clone();
        let line_number = selected_match.line_number;

        let open_task = workspace.update(cx, |workspace, cx| {
            workspace.open_path_preview(path, None, true, false, true, window, cx)
        });

        let row = line_number.saturating_sub(1);
        cx.spawn_in(window, async move |_, cx| {
            let item = open_task.await.log_err()?;
            if let Some(active_editor) = item.downcast::<editor::Editor>() {
                active_editor
                    .downgrade()
                    .update_in(cx, |editor, window, cx| {
                        editor.go_to_singleton_buffer_point(Point::new(row, 0), window, cx);
                    })
                    .log_err();
            }
            Some(())
        })
        .detach();

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let search_match = self.matches.get(ix)?;
        let path_style = self.project.read(cx).path_style(cx);
        let path_str = search_match.path.path.display(path_style).to_string();

        let original_line = &search_match.line_text;
        let line_text = original_line.trim_start();
        let trim_offset = original_line.len() - line_text.len();

        let mut highlight_indices = Vec::new();
        for range in &search_match.relative_ranges {
            for i in range.clone() {
                if i >= trim_offset {
                    let adjusted_i = i - trim_offset;
                    if line_text.is_char_boundary(adjusted_i) {
                        highlight_indices.push(adjusted_i);
                    }
                }
            }
        }

        let line_text_string = line_text.to_string();
        let highlights = highlight_ranges(
            &line_text_string,
            &highlight_indices,
            HighlightStyle {
                background_color: Some(cx.theme().colors().search_match_background),
                font_weight: Some(gpui::FontWeight::BOLD),
                ..Default::default()
            },
        );

        let text_style = window.text_style();
        // Since we are not using HighlightedLabel which handles its own base styling,
        // we might need to be careful. StyledText doesn't have a size() method directly.
        // We'll trust the default text style or adjust it if needed.

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .w_full()
                        .gap_4()
                        .justify_between()
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .overflow_hidden()
                                .text_ellipsis()
                                .whitespace_nowrap()
                                .child(
                                    StyledText::new(line_text_string)
                                        .with_default_highlights(&text_style, highlights),
                                ),
                        )
                        .child(
                            h_flex()
                                .w(relative(0.35))
                                .flex_none()
                                .justify_between()
                                .gap_2()
                                .child(
                                    div().flex_1().overflow_hidden().child(
                                        Label::new(path_str)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                            .truncate(),
                                    ),
                                )
                                .child(
                                    Label::new(search_match.line_number.to_string())
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                ),
                        ),
                ),
        )
    }
}
