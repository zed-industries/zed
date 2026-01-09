use collections::HashSet;
use std::ops::Range;
use std::sync::{Arc, RwLock};

use editor::{Editor, EditorEvent};
use gpui::{
    Action, App, Context, DismissEvent, DragMoveEvent, Entity, EventEmitter, FocusHandle,
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
use text::{Anchor, Point, ToOffset};
use theme::ActiveTheme;
use ui::Divider;
use ui::{
    Icon, IconButton, IconName, ListItem, ListItemSpacing, Tooltip, highlight_ranges, prelude::*,
};
use util::{ResultExt, paths::PathMatcher};
use workspace::{ModalView, Workspace};
pub use zed_actions::search_everywhere::Toggle;

actions!(search_everywhere, [ReplaceNext, ReplaceAll, ToggleFilters,]);

/// Global state for storing the recent search query.
struct RecentSearchState {
    last_query: RwLock<String>,
}

impl Global for RecentSearchState {}

fn get_recent_query(cx: &App) -> Option<String> {
    cx.try_global::<RecentSearchState>().and_then(|state| {
        let query = state.last_query.read().ok()?;
        if query.is_empty() {
            None
        } else {
            Some(query.clone())
        }
    })
}

fn save_recent_query(query: &str, cx: &mut App) {
    if !cx.has_global::<RecentSearchState>() {
        cx.set_global(RecentSearchState {
            last_query: RwLock::new(String::new()),
        });
    }
    if let Some(state) = cx.try_global::<RecentSearchState>() {
        if let Ok(mut last_query) = state.last_query.write() {
            *last_query = query.to_string();
        }
    }
}

/// Initialize the search_everywhere crate.
pub fn init(cx: &mut App) {
    cx.observe_new(SearchEverywhere::register).detach();
    cx.bind_keys([
        KeyBinding::new("escape", menu::Cancel, Some("SearchEverywhere")),
        KeyBinding::new("enter", ReplaceNext, Some("SearchEverywhere && in_replace")),
        KeyBinding::new(
            "cmd-enter",
            ReplaceAll,
            Some("SearchEverywhere && in_replace"),
        ),
        KeyBinding::new("alt-cmd-f", ToggleFilters, Some("SearchEverywhere")),
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

pub struct SearchEverywhere {
    picker: Entity<Picker<SearchEverywhereDelegate>>,
    preview_editor: Entity<Editor>,
    replace_editor: Entity<Editor>,
    offset: gpui::Point<Pixels>,
    _subscriptions: Vec<Subscription>,
}

#[derive(Clone, Copy)]
struct SearchEverywhereDrag {
    mouse_start: gpui::Point<Pixels>,
    offset_start: gpui::Point<Pixels>,
}

struct DragPreview;

impl Render for DragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

impl ModalView for SearchEverywhere {}

impl EventEmitter<DismissEvent> for SearchEverywhere {}

impl Focusable for SearchEverywhere {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl SearchEverywhere {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(|workspace, _: &Toggle, window, cx| {
            let project = workspace.project().clone();
            let weak_workspace = cx.entity().downgrade();
            workspace.toggle_modal(window, cx, |window, cx| {
                SearchEverywhere::new(weak_workspace, project, window, cx)
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
            editor.set_read_only(true);
            editor.set_show_breadcrumbs(false, cx);
            editor
        });

        let replace_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Replace with…", window, cx);
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

        let delegate = SearchEverywhereDelegate::new(
            workspace,
            project,
            preview_editor.clone(),
            replace_editor.clone(),
            include_editor.clone(),
            exclude_editor.clone(),
        );
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

        // Restore recent query if available
        if let Some(recent_query) = get_recent_query(cx) {
            picker.update(cx, |picker, cx| {
                picker.set_query(recent_query, window, cx);
            });
        }

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
        ];

        Self {
            picker,
            preview_editor,
            replace_editor,
            offset: gpui::Point::default(),
            _subscriptions: subscriptions,
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

impl Render for SearchEverywhere {
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

        let has_matches = match_count > 0;

        let focus_handle = self.picker.focus_handle(cx);
        let in_replace = self.replace_editor.focus_handle(cx).is_focused(window);

        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("SearchEverywhere");
        if in_replace {
            key_context.add("in_replace");
        }

        v_flex()
            .key_context(key_context)
            .id("search-everywhere")
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
                cx.notify();
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
                SearchEverywhereDrag {
                    mouse_start: window.mouse_position(),
                    offset_start: self.offset,
                },
                |_, _, _, cx| cx.new(|_| DragPreview),
            )
            .on_drag_move::<SearchEverywhereDrag>(cx.listener(
                |this, event: &DragMoveEvent<SearchEverywhereDrag>, _window, cx| {
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
                            .child(Label::new("Search Everywhere").size(LabelSize::Default))
                            .when(has_matches, |this| {
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
                            .child({
                                let replace_enabled = self.picker.read(cx).delegate.replace_enabled;
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
                                    }))
                            })
                            // Filters toggle button
                            .child({
                                let filters_enabled = self.picker.read(cx).delegate.filters_enabled;
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
                                        cx.notify();
                                    }))
                            })
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

pub struct SearchEverywhereDelegate {
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
    last_confirm_time: Option<std::time::Instant>,
    /// Tracks when set_selected_index was last called (to detect clicks vs Enter)
    last_selection_change: Option<std::time::Instant>,
    case_sensitive: bool,
    whole_word: bool,
    regex: bool,
}

impl SearchEverywhereDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        preview_editor: Entity<Editor>,
        replace_editor: Entity<Editor>,
        include_editor: Entity<Editor>,
        exclude_editor: Entity<Editor>,
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
            last_confirm_time: None,
            last_selection_change: None,
            case_sensitive: false,
            whole_word: false,
            regex: false,
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

        self.preview_editor.update(cx, |editor, cx| {
            let multi_buffer = editor.buffer().clone();

            let buffer_snapshot = buffer.read(cx);
            let max_row = buffer_snapshot.max_point().row;
            let context_lines = 5u32;

            let context_start_row = match_row.saturating_sub(context_lines);
            let context_end_row = (match_row + context_lines + 1).min(max_row);

            let context_start_offset =
                buffer_snapshot.point_to_offset(Point::new(context_start_row, 0));

            let context_start = buffer_snapshot.anchor_before(Point::new(context_start_row, 0));
            let context_end = buffer_snapshot.anchor_after(Point::new(
                context_end_row,
                buffer_snapshot.line_len(context_end_row),
            ));

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

            if let Some(range) = ranges.first() {
                let start_in_excerpt = multi_buffer::MultiBufferOffset(
                    range.start.saturating_sub(context_start_offset),
                );
                let end_in_excerpt =
                    multi_buffer::MultiBufferOffset(range.end.saturating_sub(context_start_offset));
                editor.change_selections(Default::default(), window, cx, |s| {
                    s.select_ranges([start_in_excerpt..end_in_excerpt]);
                });
            } else {
                let excerpt_row = match_row.min(context_lines);
                editor.go_to_singleton_buffer_point(Point::new(excerpt_row, 0), window, cx);
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

    fn search_text(
        &self,
        query: &str,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<Vec<SearchMatch>> {
        if query.is_empty() {
            return Task::ready(Vec::new());
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

        let search_query = if self.regex {
            match SearchQuery::regex(
                query,
                self.whole_word,
                self.case_sensitive,
                false,
                false,
                files_to_include,
                files_to_exclude,
                match_full_paths,
                None,
            ) {
                Ok(q) => q,
                Err(_) => return Task::ready(Vec::new()),
            }
        } else {
            match SearchQuery::text(
                query,
                self.whole_word,
                self.case_sensitive,
                false,
                files_to_include,
                files_to_exclude,
                match_full_paths,
                None,
            ) {
                Ok(q) => q,
                Err(_) => return Task::ready(Vec::new()),
            }
        };

        let search_results = self.project.update(cx, |project, cx| {
            eprintln!("Search Everywhere: Calling project.search for '{}'", query);
            project.search(search_query, cx)
        });

        let cancel_flag = self.cancel_flag.clone();

        cx.spawn_in(window, async move |_, cx| {
            let mut matches = Vec::new();
            let SearchResults { rx, _task_handle } = search_results;

            while let Ok(result) = rx.recv().await {
                if cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }

                match result {
                    SearchResult::Buffer { buffer, ranges } => {
                        if ranges.is_empty() {
                            continue;
                        }

                        let buffer_data = buffer.read_with(cx, |buf, cx| {
                            let file = buf.file();
                            let path = file.map(|f| ProjectPath {
                                worktree_id: f.worktree_id(cx),
                                path: f.path().clone(),
                            });
                            let text = buf.text();

                            let mut result = Vec::new();
                            for range in &ranges {
                                let start_offset: usize = buf.summary_for_anchor(&range.start);
                                let end_offset: usize = buf.summary_for_anchor(&range.end);
                                let match_row = buf.offset_to_point(start_offset).row;
                                let line_number = match_row + 1;
                                let line_start =
                                    text[..start_offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
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
                                    ));
                                }
                            }
                            result
                        });

                        for (path, anchor_range, range, relative_range, line_text, line_number) in
                            buffer_data
                        {
                            matches.push(SearchMatch {
                                path,
                                buffer: buffer.clone(),
                                anchor_ranges: vec![anchor_range],
                                ranges: vec![range],
                                relative_ranges: vec![relative_range],
                                line_text,
                                line_number,
                            });
                        }
                    }
                    SearchResult::LimitReached => {
                        eprintln!("Search Everywhere: Search limit reached");
                        break;
                    }
                }
            }

            eprintln!(
                "Search Everywhere: project.search returned {} results",
                matches.len()
            );
            matches
        })
    }
}

impl PickerDelegate for SearchEverywhereDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search in project...".into()
    }

    fn render_editor(
        &self,
        editor: &Entity<Editor>,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        let case_sensitive = self.case_sensitive;
        let whole_word = self.whole_word;
        let regex = self.regex;

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
                                IconButton::new("case-sensitive", IconName::CaseSensitive)
                                    .size(ButtonSize::Compact)
                                    .toggle_state(case_sensitive)
                                    .tooltip(Tooltip::text("Match Case"))
                                    .on_click(cx.listener(|picker, _, window, cx| {
                                        picker.delegate.case_sensitive =
                                            !picker.delegate.case_sensitive;
                                        picker.refresh(window, cx);
                                    })),
                            )
                            .child(
                                IconButton::new("whole-word", IconName::WholeWord)
                                    .size(ButtonSize::Compact)
                                    .toggle_state(whole_word)
                                    .tooltip(Tooltip::text("Match Whole Word"))
                                    .on_click(cx.listener(|picker, _, window, cx| {
                                        picker.delegate.whole_word = !picker.delegate.whole_word;
                                        picker.refresh(window, cx);
                                    })),
                            )
                            .child(
                                IconButton::new("regex", IconName::Regex)
                                    .size(ButtonSize::Compact)
                                    .toggle_state(regex)
                                    .on_click(cx.listener(|picker, _, window, cx| {
                                        picker.delegate.regex = !picker.delegate.regex;
                                        picker.refresh(window, cx);
                                    }))
                                    .tooltip(Tooltip::text("Use Regular Expression")),
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
        self.last_selection_change = Some(std::time::Instant::now());
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

        let text_search = self.search_text(&query, window, cx);

        cx.spawn_in(window, async move |picker, cx| {
            let text_matches = text_search.await;

            picker
                .update_in(cx, |picker, window, cx| {
                    let delegate = &mut picker.delegate;
                    delegate.matches = text_matches;

                    if delegate.selected_index >= delegate.matches.len() {
                        delegate.selected_index = delegate.matches.len().saturating_sub(1);
                    }

                    delegate.update_preview(window, cx);
                    cx.notify();
                })
                .ok();
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

        let now = std::time::Instant::now();

        // Check if this confirm was triggered by a click (selection changed within 50ms)
        // or by Enter key (no recent selection change)
        let is_click = self
            .last_selection_change
            .map(|t| now.duration_since(t).as_millis() < 50)
            .unwrap_or(false);

        if is_click {
            // For clicks, require double-click (two confirms within 300ms)
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
                background_color: Some(cx.theme().status().warning_background),
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
