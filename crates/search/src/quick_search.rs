use collections::{HashMap, HashSet};
use futures::StreamExt;
use std::cell::RefCell;
use std::ops::Range;
use std::pin::pin;
use std::sync::Arc;
use std::time::Duration;

use crate::{
    NextHistoryQuery, PreviousHistoryQuery, SearchOption, SearchOptions, SelectNextMatch,
    SelectPreviousMatch, ToggleCaseSensitive, ToggleIncludeIgnored, ToggleRegex, ToggleReplace,
    ToggleWholeWord,
};
use editor::EditorSettings;
use editor::{Editor, EditorEvent, RowHighlightOptions};
use gpui::{
    Action, App, AsyncApp, Context, DismissEvent, DragMoveEvent, Entity, EventEmitter, FocusHandle,
    Focusable, HighlightStyle, KeyContext, ParentElement, Render, Styled, StyledText, Subscription,
    Task, WeakEntity, Window, actions, px, relative,
};
use language::Buffer;
use menu;
use multi_buffer::{ExcerptRange, MultiBuffer};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::SearchResults;
use project::search::{SearchInputKind, SearchQuery, SearchResult};
use project::search_history::SearchHistoryCursor;
use project::{Project, ProjectPath};
use settings::Settings;
use text::{Anchor, Point, ToOffset};
use theme::ActiveTheme;
use ui::Divider;
use ui::{
    ButtonLike, ContextMenu, IconButton, IconName, KeyBinding, ListItem, ListItemSpacing,
    PopoverMenu, PopoverMenuHandle, TintColor, Tooltip, prelude::*,
};
use ui_input::ErasedEditor;
use util::{ResultExt, paths::PathMatcher};
use workspace::{ModalView, SplitDirection, Workspace, pane, searchable::SearchableItem};
pub use zed_actions::quick_search::Toggle;

actions!(
    quick_search,
    [
        ReplaceNext,
        ReplaceAll,
        ToggleFilters,
        ToggleSplitMenu,
        ToggleHistory
    ]
);

const SEARCH_DEBOUNCE_MS: u64 = 100;
const DEFAULT_RESULTS_HEIGHT: f32 = 180.0;
const DEFAULT_PREVIEW_HEIGHT: f32 = 280.0;
const MIN_PANEL_HEIGHT: f32 = 80.0;
const AUTOSAVE_DELAY_MS: u64 = 500;
const DEFAULT_MODAL_WIDTH_REMS: f32 = 50.0;
const MIN_MODAL_WIDTH_REMS: f32 = 30.0;
const MAX_MODAL_WIDTH_REMS: f32 = 80.0;
const RESIZE_HANDLE_HEIGHT: f32 = 6.0;
const RESIZE_HANDLE_WIDTH: f32 = 6.0;
const CLICK_THRESHOLD_MS: u128 = 50;
const DOUBLE_CLICK_THRESHOLD_MS: u128 = 300;

const REPLACE_PLACEHOLDER: &str = "Replace in project…";
const INCLUDE_PLACEHOLDER: &str = "Include: e.g. src/**/*.rs";
const EXCLUDE_PLACEHOLDER: &str = "Exclude: e.g. vendor/*, *.lock";

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum InputPanel {
    Query,
    Include,
    Exclude,
}

#[derive(Clone, Copy)]
enum HistoryDirection {
    Next,
    Previous,
}

struct SearchMatchHighlight;
struct SearchMatchLineHighlight;

pub fn init(cx: &mut App) {
    cx.observe_new(QuickSearch::register).detach();
}

#[derive(Clone)]
pub struct SearchMatch {
    pub path: ProjectPath,
    pub buffer: Entity<Buffer>,
    pub anchor_range: Range<Anchor>,
    pub range: Range<usize>,
    pub relative_range: Range<usize>,
    pub line_text: String,
    pub line_number: u32,
}

pub struct QuickSearch {
    picker: Entity<Picker<QuickSearchDelegate>>,
    preview_editor: Entity<Editor>,
    replacement_editor: Arc<dyn ErasedEditor>,
    focus_handle: FocusHandle,
    offset: gpui::Point<Pixels>,
    modal_width: Pixels,
    results_height: Pixels,
    preview_height: Pixels,
    _subscriptions: Vec<Subscription>,
    _autosave_task: Option<Task<()>>,
}

#[derive(Clone, Copy)]
struct QuickSearchDrag {
    mouse_start: gpui::Point<Pixels>,
    offset_start: gpui::Point<Pixels>,
}

#[derive(Clone, Copy)]
struct ResizeDrag {
    mouse_start_y: Pixels,
    results_height_start: Pixels,
    preview_height_start: Pixels,
}

#[derive(Clone, Copy)]
struct BottomResizeDrag {
    mouse_start_y: Pixels,
    results_height_start: Pixels,
    preview_height_start: Pixels,
}

#[derive(Clone, Copy)]
struct LeftResizeDrag {
    mouse_start_x: Pixels,
    width_start: Pixels,
    offset_start_x: Pixels,
}

#[derive(Clone, Copy)]
struct RightResizeDrag {
    mouse_start_x: Pixels,
    width_start: Pixels,
    offset_start_x: Pixels,
}

#[derive(Clone, Copy)]
struct TopResizeDrag {
    mouse_start_y: Pixels,
    results_height_start: Pixels,
    preview_height_start: Pixels,
    offset_start_y: Pixels,
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
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.read(cx).focus_handle(cx)
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
            let initial_query = if let Some(editor) = workspace.active_item_as::<Editor>(cx) {
                let query = editor.update(cx, |editor, cx| editor.query_suggestion(window, cx));
                if !query.is_empty() { Some(query) } else { None }
            } else {
                None
            };
            workspace.toggle_modal(window, cx, |window, cx| {
                QuickSearch::new(weak_workspace, project, initial_query, window, cx)
            });
        });
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        initial_query: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let capability = project.read(cx).capability();
        let preview_editor = cx.new(|cx| {
            let multi_buffer = cx.new(|_| MultiBuffer::without_headers(capability));
            Editor::for_multibuffer(multi_buffer, Some(project.clone()), window, cx)
        });

        let replacement_editor = ui_input::ERASED_EDITOR_FACTORY.get().unwrap()(window, cx);
        replacement_editor.set_placeholder_text(REPLACE_PLACEHOLDER, window, cx);

        let included_files_editor = ui_input::ERASED_EDITOR_FACTORY.get().unwrap()(window, cx);
        included_files_editor.set_placeholder_text(INCLUDE_PLACEHOLDER, window, cx);

        let excluded_files_editor = ui_input::ERASED_EDITOR_FACTORY.get().unwrap()(window, cx);
        excluded_files_editor.set_placeholder_text(EXCLUDE_PLACEHOLDER, window, cx);

        let focus_handle = cx.focus_handle();
        let this_handle = cx.entity().downgrade();

        let initial_query = initial_query.or_else(|| {
            project
                .read(cx)
                .search_history(SearchInputKind::Query)
                .iter()
                .next()
                .map(|s| s.to_string())
        });

        let delegate = QuickSearchDelegate::new(
            workspace,
            project,
            preview_editor.clone(),
            replacement_editor.clone(),
            included_files_editor.clone(),
            excluded_files_editor.clone(),
            initial_query,
            cx,
        );
        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .modal(false)
                .show_scrollbar(true)
        });

        let subscriptions = vec![
            cx.observe(&picker, |_, _, cx| cx.notify()),
            cx.subscribe_in(&picker, window, |this, _, _: &DismissEvent, _, cx| {
                this.save_history(cx);
                cx.emit(DismissEvent);
            }),
            included_files_editor.subscribe(
                Box::new({
                    let this_handle = this_handle.clone();
                    move |event, window, cx| {
                        if matches!(event, ui_input::ErasedEditorEvent::BufferEdited) {
                            let _ = this_handle.update(cx, |this: &mut QuickSearch, cx: &mut Context<QuickSearch>| {
                                cx.update_entity(&this.picker, |picker: &mut Picker<QuickSearchDelegate>, cx: &mut Context<Picker<QuickSearchDelegate>>| {
                                    picker.refresh(window, cx);
                                });
                            });
                        }
                    }
                }),
                window,
                cx,
            ),
            excluded_files_editor.subscribe(
                Box::new({
                    let this_handle = this_handle;
                    move |event, window, cx| {
                        if matches!(event, ui_input::ErasedEditorEvent::BufferEdited) {
                            let _ = this_handle.update(cx, |this: &mut QuickSearch, cx: &mut Context<QuickSearch>| {
                                cx.update_entity(&this.picker, |picker: &mut Picker<QuickSearchDelegate>, cx: &mut Context<Picker<QuickSearchDelegate>>| {
                                    picker.refresh(window, cx);
                                });
                            });
                        }
                    }
                }),
                window,
                cx,
            ),
            cx.on_focus_out(&focus_handle, window, |this, _, window, cx| {
                if window.is_window_active() && !this.focus_handle.contains_focused(window, cx) {
                    this.save_history(cx);
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
                                .timer(Duration::from_millis(AUTOSAVE_DELAY_MS))
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
                            })
                            .log_err();
                        }));
                    }
                },
            ),
        ];

        let modal_width = rems(DEFAULT_MODAL_WIDTH_REMS).to_pixels(window.rem_size());

        Self {
            picker,
            preview_editor,
            replacement_editor,
            focus_handle,
            offset: gpui::Point::default(),
            modal_width,
            results_height: px(DEFAULT_RESULTS_HEIGHT),
            preview_height: px(DEFAULT_PREVIEW_HEIGHT),
            _subscriptions: subscriptions,
            _autosave_task: None,
        }
    }

    fn save_history(&mut self, cx: &mut Context<Self>) {
        self.picker.update(cx, |picker, cx| {
            let delegate = &mut picker.delegate;
            let query = delegate.current_query.clone();

            if query.is_empty() {
                return;
            }

            delegate.project.update(cx, |project, _| {
                // Only add to history if it's different from the last entry
                let last_query = project.search_history(SearchInputKind::Query).iter().next();
                if last_query != Some(query.as_str()) {
                    project
                        .search_history_mut(SearchInputKind::Query)
                        .add(&mut delegate.search_history_cursor, query);
                }
            });
        });
    }

    fn replacement(&self, cx: &App) -> String {
        self.replacement_editor.text(cx)
    }

    fn replace_next(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let delegate = &self.picker.read(cx).delegate;
        let Some(selected_match) = delegate.matches.get(delegate.selected_index) else {
            return;
        };

        let replacement = self.replacement(cx);
        let buffer = selected_match.buffer.clone();
        let project = delegate.project.clone();
        let anchor_range = selected_match.anchor_range.clone();

        buffer.update(cx, |buffer, cx| {
            let snapshot = buffer.snapshot();
            let range =
                anchor_range.start.to_offset(&snapshot)..anchor_range.end.to_offset(&snapshot);
            buffer.edit([(range, replacement.as_str())], None, cx);
        });

        project
            .update(cx, |project, cx| {
                let mut buffers = HashSet::default();
                buffers.insert(buffer);
                project.save_buffers(buffers, cx)
            })
            .detach_and_log_err(cx);

        self.picker.update(cx, |picker, cx| {
            picker.refresh(window, cx);
        });
    }

    fn replace_all(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let replacement = self.replacement(cx);

        let delegate = &self.picker.read(cx).delegate;
        let matches: Vec<_> = delegate.matches.clone();
        let project = delegate.project.clone();

        let mut buffer_edits: HashMap<gpui::EntityId, (Entity<Buffer>, Vec<Range<Anchor>>)> =
            HashMap::default();

        for m in &matches {
            let buffer_id = m.buffer.entity_id();
            buffer_edits
                .entry(buffer_id)
                .or_insert_with(|| (m.buffer.clone(), Vec::new()))
                .1
                .push(m.anchor_range.clone());
        }

        let mut edited_buffers: HashSet<Entity<Buffer>> = HashSet::default();

        for (_, (buffer, mut anchor_ranges)) in buffer_edits {
            buffer.update(cx, |buf, cx| {
                let snapshot = buf.snapshot();
                // Sort descending to avoid offset invalidation when editing
                anchor_ranges.sort_by(|a, b| {
                    b.start
                        .to_offset(&snapshot)
                        .cmp(&a.start.to_offset(&snapshot))
                });

                for anchor_range in anchor_ranges {
                    let snapshot = buf.snapshot();
                    let range = anchor_range.start.to_offset(&snapshot)
                        ..anchor_range.end.to_offset(&snapshot);
                    buf.edit([(range, replacement.as_str())], None, cx);
                }
            });
            edited_buffers.insert(buffer);
        }

        if !edited_buffers.is_empty() {
            project
                .update(cx, |project, cx| project.save_buffers(edited_buffers, cx))
                .detach_and_log_err(cx);
        }

        self.picker.update(cx, |picker, cx| {
            picker.refresh(window, cx);
        });
    }

    fn go_to_file_split_left(
        &mut self,
        _: &pane::SplitLeft,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_file_split_inner(SplitDirection::Left, window, cx);
    }

    fn go_to_file_split_right(
        &mut self,
        _: &pane::SplitRight,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_file_split_inner(SplitDirection::Right, window, cx);
    }

    fn go_to_file_split_up(
        &mut self,
        _: &pane::SplitUp,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_file_split_inner(SplitDirection::Up, window, cx);
    }

    fn go_to_file_split_down(
        &mut self,
        _: &pane::SplitDown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_file_split_inner(SplitDirection::Down, window, cx);
    }

    fn go_to_file_split_inner(
        &mut self,
        split_direction: SplitDirection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let delegate = &self.picker.read(cx).delegate;
        let Some(selected_match) = delegate.matches.get(delegate.selected_index) else {
            return;
        };

        let path = selected_match.path.clone();
        let Some(workspace) = delegate.workspace.upgrade() else {
            return;
        };

        cx.emit(DismissEvent);

        workspace
            .update(cx, |workspace, cx| {
                workspace.split_path_preview(path, false, Some(split_direction), window, cx)
            })
            .detach_and_log_err(cx);
    }

    fn navigate_history(
        &mut self,
        direction: HistoryDirection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let picker = self.picker.read(cx);
        if !picker.focus_handle(cx).is_focused(window) {
            return;
        }

        let query_text = picker.query(cx);

        let new_query = self.picker.update(cx, |picker, cx| {
            let cursor = &mut picker.delegate.search_history_cursor;

            picker.delegate.project.update(cx, |project, _| {
                let history = project.search_history_mut(SearchInputKind::Query);

                let mut result = if query_text.is_empty() {
                    history.current(cursor).map(str::to_string)
                } else {
                    None
                };

                if result.is_none() {
                    result = match direction {
                        HistoryDirection::Next => history.next(cursor),
                        HistoryDirection::Previous => history.previous(cursor),
                    }
                    .map(str::to_string);
                }

                if result.as_deref() == Some(query_text.as_str()) {
                    result = match direction {
                        HistoryDirection::Next => history.next(cursor),
                        HistoryDirection::Previous => history.previous(cursor),
                    }
                    .map(str::to_string);
                }

                result
            })
        });

        match (new_query, direction) {
            (Some(query), _) => {
                self.picker.update(cx, |picker, cx| {
                    picker.set_query(&query, window, cx);
                });
            }
            (None, HistoryDirection::Next) => {
                self.picker.update(cx, |picker, cx| {
                    picker.delegate.search_history_cursor.reset();
                    picker.set_query("", window, cx);
                });
            }
            (None, HistoryDirection::Previous) => {}
        }
    }
}

impl Render for QuickSearch {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let modal_width = self.modal_width;
        let min_width = rems(MIN_MODAL_WIDTH_REMS).to_pixels(window.rem_size());
        let max_width = rems(MAX_MODAL_WIDTH_REMS).to_pixels(window.rem_size());

        let delegate = &self.picker.read(cx).delegate;
        let match_count = delegate.match_count;
        let file_count = delegate.file_count;
        let search_in_progress = delegate.search_in_progress;
        let replace_enabled = delegate.replace_enabled;
        let filters_enabled = delegate.filters_enabled;
        let selected_index = delegate.selected_index;

        let has_matches = match_count > 0;

        let focus_handle = self.focus_handle.clone();
        let in_replace = self.replacement_editor.focus_handle(cx).is_focused(window);

        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("QuickSearch");
        if in_replace {
            key_context.add("in_replace");
        }

        v_flex()
            .child(
                // Top resize handle (absolutely positioned)
                div()
                    .id("top-resize-handle")
                    .absolute()
                    .top(-px(RESIZE_HANDLE_HEIGHT))
                    .left_0()
                    .right_0()
                    .h(px(RESIZE_HANDLE_HEIGHT))
                    .cursor_row_resize()
                    .on_drag(
                        TopResizeDrag {
                            mouse_start_y: window.mouse_position().y,
                            results_height_start: self.results_height,
                            preview_height_start: self.preview_height,
                            offset_start_y: self.offset.y,
                        },
                        |_, _, _, cx| cx.new(|_| DragPreview),
                    )
                    .on_drag_move::<TopResizeDrag>(cx.listener(
                        move |this, event: &DragMoveEvent<TopResizeDrag>, _window, cx| {
                            let drag = event.drag(cx);
                            let delta = event.event.position.y - drag.mouse_start_y;

                            let total_start = drag.results_height_start + drag.preview_height_start;
                            let min_total = px(MIN_PANEL_HEIGHT * 2.0);
                            
                            let new_total = (total_start - delta).max(min_total);
                            let scale = new_total / total_start;

                            let new_results = drag.results_height_start * scale;
                            let new_preview = drag.preview_height_start * scale;
                            
                            let total_change = new_total - total_start;

                            this.results_height = new_results;
                            this.preview_height = new_preview;
                            this.offset.y = drag.offset_start_y - total_change;
                            cx.notify();
                        },
                    )),
            )
            .child(
                // Left resize handle (absolutely positioned)
                div()
                    .id("left-resize-handle")
                    .absolute()
                    .left(-px(RESIZE_HANDLE_WIDTH))
                    .top_0()
                    .bottom_0()
                    .w(px(RESIZE_HANDLE_WIDTH))
                    .cursor_col_resize()
                    .on_drag(
                        LeftResizeDrag {
                            mouse_start_x: window.mouse_position().x,
                            width_start: self.modal_width,
                            offset_start_x: self.offset.x,
                        },
                        |_, _, _, cx| cx.new(|_| DragPreview),
                    )
                    .on_drag_move::<LeftResizeDrag>(cx.listener(
                        move |this, event: &DragMoveEvent<LeftResizeDrag>, _window, cx| {
                            let drag = event.drag(cx);
                            let delta = drag.mouse_start_x - event.event.position.x;

                            let new_width = (drag.width_start + delta)
                                .max(min_width)
                                .min(max_width);

                            let width_change = new_width - drag.width_start;
                            this.modal_width = new_width;
                            this.offset.x = drag.offset_start_x - (width_change / 2.0);
                            cx.notify();
                        },
                    )),
            )
            .child(
                // Right resize handle (absolutely positioned)
                div()
                    .id("right-resize-handle")
                    .absolute()
                    .right(-px(RESIZE_HANDLE_WIDTH))
                    .top_0()
                    .bottom_0()
                    .w(px(RESIZE_HANDLE_WIDTH))
                    .cursor_col_resize()
                    .on_drag(
                        RightResizeDrag {
                            mouse_start_x: window.mouse_position().x,
                            width_start: self.modal_width,
                            offset_start_x: self.offset.x,
                        },
                        |_, _, _, cx| cx.new(|_| DragPreview),
                    )
                    .on_drag_move::<RightResizeDrag>(cx.listener(
                        move |this, event: &DragMoveEvent<RightResizeDrag>, _window, cx| {
                            let drag = event.drag(cx);
                            let delta = event.event.position.x - drag.mouse_start_x;

                            let new_width = (drag.width_start + delta)
                                .max(min_width)
                                .min(max_width);

                            let width_change = new_width - drag.width_start;
                            this.modal_width = new_width;
                            this.offset.x = drag.offset_start_x + (width_change / 2.0);
                            cx.notify();
                        },
                    )),
            )
            .m_4()
            .relative()
            .top(self.offset.y)
            .left(self.offset.x)
            .child(
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
                            let focus_handle = if picker.delegate.filters_enabled {
                                picker.delegate.included_files_editor.focus_handle(cx)
                            } else {
                                picker.focus_handle(cx)
                            };
                            window.focus(&focus_handle, cx);
                        });
                        cx.notify();
                    }))
                    .on_action(cx.listener(|this, _: &NextHistoryQuery, window, cx| {
                        this.navigate_history(HistoryDirection::Next, window, cx);
                    }))
                    .on_action(cx.listener(|this, _: &PreviousHistoryQuery, window, cx| {
                        this.navigate_history(HistoryDirection::Previous, window, cx);
                    }))
                    .on_action(cx.listener(|this, _: &ToggleHistory, window, cx| {
                        let handle = this
                            .picker
                            .read(cx)
                            .delegate
                            .history_popover_menu_handle
                            .clone();
                        handle.toggle(window, cx);
                        cx.notify();
                    }))
                    .on_action(cx.listener(|this, _: &ToggleCaseSensitive, window, cx| {
                        this.picker.update(cx, |picker, cx| {
                            picker
                                .delegate
                                .search_options
                                .toggle(SearchOptions::CASE_SENSITIVE);
                            picker.refresh(window, cx);
                        });
                    }))
                    .on_action(cx.listener(|this, _: &ToggleWholeWord, window, cx| {
                        this.picker.update(cx, |picker, cx| {
                            picker
                                .delegate
                                .search_options
                                .toggle(SearchOptions::WHOLE_WORD);
                            picker.refresh(window, cx);
                        });
                    }))
                    .on_action(cx.listener(|this, _: &ToggleIncludeIgnored, window, cx| {
                        this.picker.update(cx, |picker, cx| {
                            picker
                                .delegate
                                .search_options
                                .toggle(SearchOptions::INCLUDE_IGNORED);
                            picker.refresh(window, cx);
                        });
                    }))
                    .on_action(cx.listener(|this, _: &ToggleRegex, window, cx| {
                        this.picker.update(cx, |picker, cx| {
                            picker.delegate.search_options.toggle(SearchOptions::REGEX);
                            picker.refresh(window, cx);
                        });
                    }))
                    .on_action(cx.listener(|this, _: &ToggleReplace, window, cx| {
                        this.picker.update(cx, |picker, cx| {
                            picker.delegate.replace_enabled = !picker.delegate.replace_enabled;
                            let focus_handle = if picker.delegate.replace_enabled {
                                picker.delegate.replacement_editor.focus_handle(cx)
                            } else {
                                picker.focus_handle(cx)
                            };
                            window.focus(&focus_handle, cx);
                        });
                        cx.notify();
                    }))
                    .on_action(cx.listener(|this, _: &SelectNextMatch, window, cx| {
                        this.picker.update(cx, |picker, cx| {
                            let match_count = picker.delegate.matches.len();
                            if match_count > 0 {
                                let new_index = (picker.delegate.selected_index + 1) % match_count;
                                picker.set_selected_index(new_index, None, true, window, cx);
                            }
                        });
                    }))
                    .on_action(cx.listener(|this, _: &SelectPreviousMatch, window, cx| {
                        this.picker.update(cx, |picker, cx| {
                            let match_count = picker.delegate.matches.len();
                            if match_count > 0 {
                                let new_index = if picker.delegate.selected_index == 0 {
                                    match_count - 1
                                } else {
                                    picker.delegate.selected_index - 1
                                };
                                picker.set_selected_index(new_index, None, true, window, cx);
                            }
                        });
                    }))
                    .on_action(cx.listener(|this, _: &ToggleSplitMenu, window, cx| {
                        this.picker.update(cx, |picker, cx| {
                            let menu_handle = &picker.delegate.split_popover_menu_handle;
                            if menu_handle.is_deployed() {
                                menu_handle.hide(cx);
                            } else {
                                menu_handle.show(window, cx);
                            }
                        });
                    }))
                    .on_action(cx.listener(Self::go_to_file_split_left))
                    .on_action(cx.listener(Self::go_to_file_split_right))
                    .on_action(cx.listener(Self::go_to_file_split_up))
                    .on_action(cx.listener(Self::go_to_file_split_down))
                    .w(modal_width)
                    .bg(cx.theme().colors().elevated_surface_background)
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .rounded_lg()
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
                            .child({
                                let focus_handle = self.picker.focus_handle(cx);
                                IconButton::new("replace-toggle", IconName::Replace)
                                    .size(ButtonSize::Compact)
                                    .toggle_state(replace_enabled)
                                    .tooltip(move |_window, cx| {
                                        Tooltip::for_action_in(
                                            "Toggle Replace",
                                            &ToggleReplace,
                                            &focus_handle,
                                            cx,
                                        )
                                    })
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.picker.update(cx, |picker, cx| {
                                            picker.delegate.replace_enabled =
                                                !picker.delegate.replace_enabled;
                                            let focus_handle = if picker.delegate.replace_enabled {
                                                picker.delegate.replacement_editor.focus_handle(cx)
                                            } else {
                                                picker.focus_handle(cx)
                                            };
                                            window.focus(&focus_handle, cx);
                                        });
                                        cx.notify();
                                    }))
                            })
                            .child({
                                let focus_handle = self.picker.focus_handle(cx);
                                IconButton::new("filters-toggle", IconName::Filter)
                                    .size(ButtonSize::Compact)
                                    .toggle_state(filters_enabled)
                                    .tooltip(move |_window, cx| {
                                        Tooltip::for_action_in(
                                            "Toggle Filters",
                                            &ToggleFilters,
                                            &focus_handle,
                                            cx,
                                        )
                                    })
                                    .on_click(|_, window, cx| {
                                        window.dispatch_action(ToggleFilters.boxed_clone(), cx);
                                    })
                            })
                            .child({
                                let focus_handle = self.picker.focus_handle(cx);
                                IconButton::new("select-prev-match", IconName::ChevronLeft)
                                    .size(ButtonSize::Compact)
                                    .tooltip(move |_window, cx| {
                                        Tooltip::for_action_in(
                                            "Previous Match",
                                            &SelectPreviousMatch,
                                            &focus_handle,
                                            cx,
                                        )
                                    })
                                    .on_click(|_, window, cx| {
                                        window
                                            .dispatch_action(SelectPreviousMatch.boxed_clone(), cx);
                                    })
                            })
                            .child({
                                let focus_handle = self.picker.focus_handle(cx);
                                IconButton::new("select-next-match", IconName::ChevronRight)
                                    .size(ButtonSize::Compact)
                                    .tooltip(move |_window, cx| {
                                        Tooltip::for_action_in(
                                            "Next Match",
                                            &SelectNextMatch,
                                            &focus_handle,
                                            cx,
                                        )
                                    })
                                    .on_click(|_, window, cx| {
                                        window.dispatch_action(SelectNextMatch.boxed_clone(), cx);
                                    })
                            })
                            .when(match_count > 0, |this| {
                                this.child(
                                    Label::new(format!("{}/{}", selected_index + 1, match_count))
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                            }),
                    ),
            )
            .child(
                div()
                    .h(self.results_height)
                    .overflow_hidden()
                    .child(self.picker.clone()),
            )
            .child({
                // Resize handle between results and preview
                div()
                    .id("resize-handle")
                    .h(px(RESIZE_HANDLE_HEIGHT))
                    .w_full()
                    .cursor_row_resize()
                    .bg(cx.theme().colors().border)
                    .hover(|style| style.bg(cx.theme().colors().border_focused))
                    .on_drag(
                        ResizeDrag {
                            mouse_start_y: window.mouse_position().y,
                            results_height_start: self.results_height,
                            preview_height_start: self.preview_height,
                        },
                        |_, _, _, cx| cx.new(|_| DragPreview),
                    )
                    .on_drag_move::<ResizeDrag>(cx.listener(
                        |this, event: &DragMoveEvent<ResizeDrag>, _window, cx| {
                            let drag = event.drag(cx);
                            let delta = event.event.position.y - drag.mouse_start_y;
                            let total_height =
                                drag.results_height_start + drag.preview_height_start;

                            let new_results = (drag.results_height_start + delta)
                                .max(px(MIN_PANEL_HEIGHT))
                                .min(total_height - px(MIN_PANEL_HEIGHT));
                            let new_preview = total_height - new_results;

                            this.results_height = new_results;
                            this.preview_height = new_preview;
                            cx.notify();
                        },
                    ))
            })
            .child({
                let delegate = &self.picker.read(cx).delegate;
                let selected_match = delegate.matches.get(delegate.selected_index);

                let preview_header = if let Some(m) = selected_match {
                    let path = &m.path.path;
                    let file_name = path
                        .file_name()
                        .map(|name| name.to_string())
                        .unwrap_or_default();
                    let directory = path
                        .parent()
                        .map(|path| path.as_std_path().to_string_lossy().to_string())
                        .unwrap_or_default();

                    let split_menu_handle = delegate.split_popover_menu_handle.clone();
                    let focus_handle = self.focus_handle.clone();

                    h_flex()
                        .px_2()
                        .py_1()
                        .gap_2()
                        .border_b_1()
                        .border_color(cx.theme().colors().border)
                        .bg(cx.theme().colors().editor_background)
                        .justify_between()
                        .child(
                            h_flex()
                                .gap_2()
                                .child(Label::new(file_name).size(LabelSize::Small))
                                .child(
                                    Label::new(directory)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                ),
                        )
                        .child(
                            PopoverMenu::new("split-menu-popover")
                                .with_handle(split_menu_handle)
                                .attach(gpui::Corner::BottomRight)
                                .anchor(gpui::Corner::TopRight)
                                .offset(gpui::Point {
                                    x: px(0.0),
                                    y: px(-2.0),
                                })
                                .trigger_with_tooltip(
                                    ButtonLike::new("split-trigger")
                                        .child(Label::new("Split…").size(LabelSize::Small))
                                        .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                                        .child(
                                            KeyBinding::for_action_in(
                                                &ToggleSplitMenu,
                                                &focus_handle,
                                                cx,
                                            )
                                            .size(rems_from_px(10.)),
                                        ),
                                    {
                                        let focus_handle = focus_handle.clone();
                                        move |_window, cx| {
                                            Tooltip::for_action_in(
                                                "Open in Split",
                                                &ToggleSplitMenu,
                                                &focus_handle,
                                                cx,
                                            )
                                        }
                                    },
                                )
                                .menu({
                                    let focus_handle = focus_handle.clone();
                                    move |window, cx| {
                                        Some(ContextMenu::build(window, cx, {
                                            let focus_handle = focus_handle.clone();
                                            move |menu, _, _| {
                                                menu.context(focus_handle)
                                                    .action(
                                                        "Split Left",
                                                        pane::SplitLeft::default().boxed_clone(),
                                                    )
                                                    .action(
                                                        "Split Right",
                                                        pane::SplitRight::default().boxed_clone(),
                                                    )
                                                    .action(
                                                        "Split Up",
                                                        pane::SplitUp::default().boxed_clone(),
                                                    )
                                                    .action(
                                                        "Split Down",
                                                        pane::SplitDown::default().boxed_clone(),
                                                    )
                                            }
                                        }))
                                    }
                                }),
                        )
                } else {
                    h_flex().h(px(26.0))
                };

                v_flex().child(preview_header).child(
                    div()
                        .h(self.preview_height)
                        .overflow_hidden()
                        .child(self.preview_editor.clone()),
                )
            })
            .child({
                // Bottom resize handle for preview
                div()
                    .id("bottom-resize-handle")
                    .h(px(RESIZE_HANDLE_HEIGHT))
                    .w_full()
                    .cursor_row_resize()
                    .bg(cx.theme().colors().border)
                    .hover(|style| style.bg(cx.theme().colors().border_focused))
                    .on_drag(
                        BottomResizeDrag {
                            mouse_start_y: window.mouse_position().y,
                            results_height_start: self.results_height,
                            preview_height_start: self.preview_height,
                        },
                        |_, _, _, cx| cx.new(|_| DragPreview),
                    )
                    .on_drag_move::<BottomResizeDrag>(cx.listener(
                        |this, event: &DragMoveEvent<BottomResizeDrag>, _window, cx| {
                            let drag = event.drag(cx);
                            let delta = event.event.position.y - drag.mouse_start_y;

                            let total_start = drag.results_height_start + drag.preview_height_start;
                            let min_total = px(MIN_PANEL_HEIGHT * 2.0);
                            
                            let new_total = (total_start + delta).max(min_total);
                            let scale = new_total / total_start;

                            let new_results = drag.results_height_start * scale;
                            let new_preview = drag.preview_height_start * scale;

                            this.results_height = new_results;
                            this.preview_height = new_preview;
                            cx.notify();
                        },
                    ))
            }),
            )
    }
}

pub struct QuickSearchDelegate {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    preview_editor: Entity<Editor>,
    replacement_editor: Arc<dyn ErasedEditor>,
    included_files_editor: Arc<dyn ErasedEditor>,
    excluded_files_editor: Arc<dyn ErasedEditor>,
    replace_enabled: bool,
    filters_enabled: bool,
    included_opened_only: bool,
    matches: Vec<SearchMatch>,
    selected_index: usize,
    cancel_flag: Arc<std::sync::atomic::AtomicBool>,
    last_selection_change_time: Option<std::time::Instant>,
    last_confirm_time: Option<std::time::Instant>,
    search_options: SearchOptions,
    search_in_progress: bool,
    pending_initial_query: RefCell<Option<String>>,
    panels_with_errors: HashMap<InputPanel, String>,
    split_popover_menu_handle: PopoverMenuHandle<ContextMenu>,
    history_popover_menu_handle: PopoverMenuHandle<ContextMenu>,
    search_history_cursor: SearchHistoryCursor,
    current_query: String,
    match_count: usize,
    file_count: usize,
    unique_files: HashSet<ProjectPath>,
}

impl QuickSearchDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        preview_editor: Entity<Editor>,
        replacement_editor: Arc<dyn ErasedEditor>,
        included_files_editor: Arc<dyn ErasedEditor>,
        excluded_files_editor: Arc<dyn ErasedEditor>,
        initial_query: Option<String>,
        cx: &App,
    ) -> Self {
        Self {
            workspace,
            project,
            preview_editor,
            replacement_editor,
            included_files_editor,
            excluded_files_editor,
            replace_enabled: false,
            filters_enabled: false,
            included_opened_only: false,
            matches: Vec::new(),
            selected_index: 0,
            cancel_flag: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            last_selection_change_time: None,
            last_confirm_time: None,
            search_options: SearchOptions::from_settings(&EditorSettings::get_global(cx).search),
            search_in_progress: false,
            pending_initial_query: RefCell::new(initial_query),
            panels_with_errors: HashMap::default(),
            split_popover_menu_handle: PopoverMenuHandle::default(),
            history_popover_menu_handle: PopoverMenuHandle::default(),
            search_history_cursor: SearchHistoryCursor::default(),
            current_query: String::new(),
            match_count: 0,
            file_count: 0,
            unique_files: HashSet::default(),
        }
    }

    fn open_buffers(&self, cx: &App) -> Vec<Entity<Buffer>> {
        let mut buffers = Vec::new();
        if let Some(workspace) = self.workspace.upgrade() {
            let workspace = workspace.read(cx);
            for editor in workspace.items_of_type::<Editor>(cx) {
                if let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton() {
                    buffers.push(buffer);
                }
            }
        }
        buffers
    }

    fn update_preview(&self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(selected_match) = self.matches.get(self.selected_index) else {
            self.preview_editor.update(cx, |editor, cx| {
                editor.buffer().update(cx, |multi_buffer, cx| {
                    if !multi_buffer.read(cx).is_empty() {
                        multi_buffer.clear(cx);
                    }
                });
            });
            return;
        };

        let buffer = selected_match.buffer.clone();
        let range = selected_match.range.clone();
        let anchor_range = selected_match.anchor_range.clone();

        self.preview_editor.update(cx, |editor, cx| {
            let multi_buffer = editor.buffer().clone();
            let buffer_snapshot = buffer.read(cx);
            let max_point = buffer_snapshot.max_point();

            let context_start = buffer_snapshot.anchor_before(Point::new(0, 0));
            let context_end = buffer_snapshot.anchor_after(max_point);

            let primary_range = {
                let start = buffer_snapshot.anchor_before(range.start);
                let end = buffer_snapshot.anchor_after(range.end);
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

            let multi_buffer_snapshot = multi_buffer.read(cx);
            if let Some(excerpt_id) = multi_buffer_snapshot.excerpt_ids().first().copied() {
                // Highlight the entire row (including gutter)
                let row_anchor = editor::Anchor::in_buffer(excerpt_id, anchor_range.start);
                editor.highlight_rows::<SearchMatchLineHighlight>(
                    row_anchor..row_anchor,
                    cx.theme().colors().editor_active_line_background,
                    RowHighlightOptions::default(),
                    cx,
                );

                // Highlight the match itself
                let highlight_range =
                    editor::Anchor::range_in_buffer(excerpt_id, anchor_range.clone());

                editor.highlight_background::<SearchMatchHighlight>(
                    &[highlight_range],
                    |_, theme| theme.colors().search_match_background,
                    cx,
                );
            }

            let start = multi_buffer::MultiBufferOffset(range.start);
            let end = multi_buffer::MultiBufferOffset(range.end);
            editor.change_selections(Default::default(), window, cx, |s| {
                s.select_ranges([start..end]);
            });
        });
    }

    fn parse_path_matches(&self, text: String, cx: &App) -> anyhow::Result<PathMatcher> {
        let path_style = self.project.read(cx).path_style(cx);
        let queries: Vec<String> = text
            .split(',')
            .map(str::trim)
            .filter(|maybe_glob_str| !maybe_glob_str.is_empty())
            .map(str::to_owned)
            .collect::<Vec<_>>();
        Ok(PathMatcher::new(&queries, path_style)?)
    }

    fn build_search_query(
        &mut self,
        query: &str,
        open_buffers: Option<Vec<Entity<Buffer>>>,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<SearchQuery> {
        if query.is_empty() {
            self.panels_with_errors.remove(&InputPanel::Query);
            return None;
        }

        let files_to_include = if self.filters_enabled {
            let include_text = self.included_files_editor.text(cx);
            match self.parse_path_matches(include_text, cx) {
                Ok(matcher) => {
                    if self
                        .panels_with_errors
                        .remove(&InputPanel::Include)
                        .is_some()
                    {
                        cx.notify();
                    }
                    matcher
                }
                Err(e) => {
                    if self
                        .panels_with_errors
                        .insert(InputPanel::Include, e.to_string())
                        .is_none()
                    {
                        cx.notify();
                    }
                    PathMatcher::default()
                }
            }
        } else {
            self.panels_with_errors.remove(&InputPanel::Include);
            PathMatcher::default()
        };

        let files_to_exclude = if self.filters_enabled {
            let exclude_text = self.excluded_files_editor.text(cx);
            match self.parse_path_matches(exclude_text, cx) {
                Ok(matcher) => {
                    if self
                        .panels_with_errors
                        .remove(&InputPanel::Exclude)
                        .is_some()
                    {
                        cx.notify();
                    }
                    matcher
                }
                Err(e) => {
                    if self
                        .panels_with_errors
                        .insert(InputPanel::Exclude, e.to_string())
                        .is_none()
                    {
                        cx.notify();
                    }
                    PathMatcher::default()
                }
            }
        } else {
            self.panels_with_errors.remove(&InputPanel::Exclude);
            PathMatcher::default()
        };

        // If the project contains multiple visible worktrees, we match the
        // include/exclude patterns against full paths to allow them to be
        // disambiguated. For single worktree projects we use worktree relative
        // paths for convenience.
        let match_full_paths = self.
            project.
            read(cx).
            visible_worktrees(cx).
            count()
            > 1;


        let result = if self.search_options.contains(SearchOptions::REGEX) {
            SearchQuery::regex(
                query,
                self.search_options.contains(SearchOptions::WHOLE_WORD),
                self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                self.search_options.contains(SearchOptions::INCLUDE_IGNORED),
                self.search_options
                    .contains(SearchOptions::ONE_MATCH_PER_LINE),
                files_to_include,
                files_to_exclude,
                match_full_paths,
                open_buffers,
            )
        } else {
            SearchQuery::text(
                query,
                self.search_options.contains(SearchOptions::WHOLE_WORD),
                self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                self.search_options.contains(SearchOptions::INCLUDE_IGNORED),
                files_to_include,
                files_to_exclude,
                match_full_paths,
                open_buffers,
            )
        };

        match result {
            Ok(search_query) => {
                if self.panels_with_errors.remove(&InputPanel::Query).is_some() {
                    cx.notify();
                }
                Some(search_query)
            }
            Err(e) => {
                if self
                    .panels_with_errors
                    .insert(InputPanel::Query, e.to_string())
                    .is_none()
                {
                    cx.notify();
                }
                None
            }
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

        buffer.read_with(cx, |buf, cx| {
            let file = buf.file();
            let path = file.map(|f| ProjectPath {
                worktree_id: f.worktree_id(cx),
                path: f.path().clone(),
            });
            let text = buf.text();

            let mut matches = Vec::new();
            for anchor_range in ranges {
                let start_offset: usize = buf.summary_for_anchor(&anchor_range.start);
                let end_offset: usize = buf.summary_for_anchor(&anchor_range.end);
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

                if let Some(path) = &path {
                    matches.push(SearchMatch {
                        path: path.clone(),
                        buffer: buffer.clone(),
                        anchor_range: anchor_range.clone(),
                        range: start_offset..end_offset,
                        relative_range: relative_start..relative_end,
                        line_text,
                        line_number,
                    });
                }
            }
            matches
        })
    }
    fn render_history_menu(
        project: &Entity<Project>,
        editor: &Arc<dyn ErasedEditor>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Entity<ContextMenu>> {
        let history_entries: Vec<String> = project
            .read(cx)
            .search_history(SearchInputKind::Query)
            .iter()
            .map(str::to_string)
            .collect();

        let editor = editor.clone();
        Some(ContextMenu::build(
            window,
            cx,
            move |mut menu, _window, _| {
                if history_entries.is_empty() {
                    menu.header("No recent searches")
                } else {
                    for query in history_entries {
                        let editor = editor.clone();
                        let query_for_click: String = query.clone();
                        menu = menu.entry(query, None, move |window, cx| {
                            editor.set_text(&query_for_click, window, cx);
                        });
                    }
                    menu
                }
            },
        ))
    }
}

impl PickerDelegate for QuickSearchDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search all files...".into()
    }

    fn render_editor(
        &self,
        editor: &Arc<dyn ErasedEditor>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        let search_options = self.search_options;
        editor.set_multiline(Some(4), window, cx);
        let focus_handle = editor.focus_handle(cx);

        if let Some(query) = self.pending_initial_query.borrow_mut().take() {
            editor.set_text(&query, window, cx);
        }

        v_flex()
            .child(
                h_flex()
                    .flex_none()
                    .min_h_9()
                    .px_2p5()
                    .gap_1()
                    .items_start()
                    .child(
                        h_flex()
                            .flex_1()
                            .overflow_hidden()
                            .py_1p5()
                            .border_1()
                            .rounded_md()
                            .pl_0p5()
                            .pr_1()
                            .border_color(
                                if self.panels_with_errors.contains_key(&InputPanel::Query) {
                                    Color::Error.color(cx)
                                } else {
                                    gpui::transparent_black()
                                },
                            )
                            .gap_1()
                            .child(
                                PopoverMenu::new("history-menu-popover")
                                    .with_handle(self.history_popover_menu_handle.clone())
                                    .trigger(
                                        IconButton::new(
                                            "search-history",
                                            IconName::MagnifyingGlass,
                                        )
                                        .tooltip({
                                            let focus_handle = editor.focus_handle(cx);
                                            move |_window, cx| {
                                                Tooltip::for_action_in(
                                                    "Search History",
                                                    &ToggleHistory,
                                                    &focus_handle,
                                                    cx,
                                                )
                                            }
                                        }),
                                    )
                                    .menu({
                                        let editor = editor.clone();
                                        let project = self.project.clone();
                                        move |window, cx| {
                                            Self::render_history_menu(&project, &editor, window, cx)
                                        }
                                    }),
                            )
                            .child(div().flex_1().min_w_0().child(editor.render(window, cx))),
                    )
                    .child({
                        let focus_handle = focus_handle.clone();
                        h_flex()
                            .flex_none()
                            .gap_0p5()
                            .pt_1()
                            .child({
                                let editor_for_click = editor.clone();
                                IconButton::new("insert-newline", IconName::Return)
                                    .size(ButtonSize::Compact)
                                    .tooltip(Tooltip::text("Insert New Line"))
                                    .on_click(move |_, window, cx| {
                                        let text = editor_for_click.text(cx);
                                        editor_for_click.set_text(&(text + "\n"), window, cx);
                                    })
                            })
                            .child({
                                let focus_handle = focus_handle.clone();
                                IconButton::new(
                                    "case-sensitive",
                                    SearchOption::CaseSensitive.icon(),
                                )
                                .size(ButtonSize::Compact)
                                .toggle_state(
                                    search_options.contains(SearchOptions::CASE_SENSITIVE),
                                )
                                .tooltip(move |_window, cx| {
                                    Tooltip::for_action_in(
                                        SearchOption::CaseSensitive.label(),
                                        &ToggleCaseSensitive,
                                        &focus_handle,
                                        cx,
                                    )
                                })
                                .on_click(cx.listener(
                                    |picker, _, window, cx| {
                                        picker
                                            .delegate
                                            .search_options
                                            .toggle(SearchOptions::CASE_SENSITIVE);
                                        picker.refresh(window, cx);
                                    },
                                ))
                            })
                            .child({
                                let focus_handle = focus_handle.clone();
                                IconButton::new("whole-word", SearchOption::WholeWord.icon())
                                    .size(ButtonSize::Compact)
                                    .toggle_state(
                                        search_options.contains(SearchOptions::WHOLE_WORD),
                                    )
                                    .tooltip(move |_window, cx| {
                                        Tooltip::for_action_in(
                                            SearchOption::WholeWord.label(),
                                            &ToggleWholeWord,
                                            &focus_handle,
                                            cx,
                                        )
                                    })
                                    .on_click(cx.listener(|picker, _, window, cx| {
                                        picker
                                            .delegate
                                            .search_options
                                            .toggle(SearchOptions::WHOLE_WORD);
                                        picker.refresh(window, cx);
                                    }))
                            })
                            .child(
                                IconButton::new("regex", SearchOption::Regex.icon())
                                    .size(ButtonSize::Compact)
                                    .toggle_state(search_options.contains(SearchOptions::REGEX))
                                    .tooltip(move |_window, cx| {
                                        Tooltip::for_action_in(
                                            SearchOption::Regex.label(),
                                            &ToggleRegex,
                                            &focus_handle,
                                            cx,
                                        )
                                    })
                                    .on_click(cx.listener(|picker, _, window, cx| {
                                        picker.delegate.search_options.toggle(SearchOptions::REGEX);
                                        picker.refresh(window, cx);
                                    })),
                            )
                    }),
            )
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
                                .child(self.replacement_editor.render(window, cx)),
                        )
                        .child({
                            h_flex()
                                .flex_none()
                                .gap_0p5()
                                .child({
                                    let focus_handle = focus_handle.clone();
                                    IconButton::new("replace-next", IconName::ReplaceNext)
                                        .shape(ui::IconButtonShape::Square)
                                        .tooltip(move |_window, cx| {
                                            Tooltip::for_action_in(
                                                "Replace Next Match",
                                                &ReplaceNext,
                                                &focus_handle,
                                                cx,
                                            )
                                        })
                                        .on_click(|_, window, cx| {
                                            window.dispatch_action(ReplaceNext.boxed_clone(), cx);
                                        })
                                })
                                .child({
                                    let focus_handle = focus_handle.clone();
                                    IconButton::new("replace-all", IconName::ReplaceAll)
                                        .shape(ui::IconButtonShape::Square)
                                        .tooltip(move |_window, cx| {
                                            Tooltip::for_action_in(
                                                "Replace All",
                                                &ReplaceAll,
                                                &focus_handle,
                                                cx,
                                            )
                                        })
                                        .on_click(|_, window, cx| {
                                            window.dispatch_action(ReplaceAll.boxed_clone(), cx);
                                        })
                                })
                        }),
                )
            })
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
                                        .border_1()
                                        .rounded_md()
                                        .px_1()
                                        .border_color(
                                            if self
                                                .panels_with_errors
                                                .contains_key(&InputPanel::Include)
                                            {
                                                Color::Error.color(cx)
                                            } else {
                                                gpui::transparent_black()
                                            },
                                        )
                                        .child(self.included_files_editor.render(window, cx)),
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
                                        .border_1()
                                        .rounded_md()
                                        .px_1()
                                        .border_color(
                                            if self
                                                .panels_with_errors
                                                .contains_key(&InputPanel::Exclude)
                                            {
                                                Color::Error.color(cx)
                                            } else {
                                                gpui::transparent_black()
                                            },
                                        )
                                        .child(self.excluded_files_editor.render(window, cx)),
                                ),
                        )
                        .child(
                            h_flex()
                                .gap_0p5()
                                .child(
                                    IconButton::new("opened-only", IconName::FolderSearch)
                                        .size(ButtonSize::Compact)
                                        .toggle_state(self.included_opened_only)
                                        .tooltip(Tooltip::text("Only Search Open Files"))
                                        .on_click(cx.listener(|picker, _, window, cx| {
                                            picker.delegate.included_opened_only =
                                                !picker.delegate.included_opened_only;
                                            picker.refresh(window, cx);
                                        })),
                                )
                                .child(
                                    IconButton::new("include-ignored", IconName::Sliders)
                                        .size(ButtonSize::Compact)
                                        .toggle_state(
                                            self.search_options
                                                .contains(SearchOptions::INCLUDE_IGNORED),
                                        )
                                        .tooltip(Tooltip::text(
                                            "Also search files ignored by configuration",
                                        ))
                                        .on_click(cx.listener(|picker, _, window, cx| {
                                            picker
                                                .delegate
                                                .search_options
                                                .toggle(SearchOptions::INCLUDE_IGNORED);
                                            picker.refresh(window, cx);
                                        })),
                                ),
                        ),
                )
            })
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
        self.last_selection_change_time = Some(std::time::Instant::now());
        self.update_preview(window, cx);
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.current_query = query.clone();

        self.cancel_flag
            .store(true, std::sync::atomic::Ordering::SeqCst);
        self.cancel_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));

        let cancel_flag = self.cancel_flag.clone();

        let open_buffers = if self.included_opened_only {
            Some(self.open_buffers(cx))
        } else {
            None
        };

        let Some(search_query) = self.build_search_query(&query, open_buffers, cx) else {
            self.matches.clear();
            self.selected_index = 0;
            self.search_in_progress = false;
            cx.notify();
            return Task::ready(());
        };

        let search_results = self
            .project
            .update(cx, |project, cx| project.search(search_query, cx));

        self.search_in_progress = true;
        cx.notify();

        cx.spawn_in(window, async move |picker, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(SEARCH_DEBOUNCE_MS))
                .await;

            if cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
                return;
            }

            let mut first_batch = true;
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
                            let matches =
                                QuickSearchDelegate::process_search_result(&buffer, &ranges, cx);
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

                        if first_batch {
                            delegate.matches.clear();
                            delegate.match_count = 0;
                            delegate.file_count = 0;
                            delegate.unique_files.clear();
                            delegate.selected_index = 0;
                            first_batch = false;
                        }

                        for m in &batch_matches {
                            if delegate.unique_files.insert(m.path.clone()) {
                                delegate.file_count += 1;
                            }
                        }
                        delegate.matches.extend(batch_matches);
                        delegate.match_count = delegate.matches.len();

                        if delegate.selected_index >= delegate.match_count
                            && delegate.match_count > 0
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
        let in_replace =
            self.replace_enabled && self.replacement_editor.focus_handle(cx).is_focused(window);

        if in_replace {
            if secondary {
                window.dispatch_action(ReplaceAll.boxed_clone(), cx);
            } else {
                window.dispatch_action(ReplaceNext.boxed_clone(), cx);
            }
            return;
        }

        // Clicks (set_selected_index called immediately before confirm) require double-click.
        // Enter key proceeds immediately.
        let now = std::time::Instant::now();
        let is_click = self
            .last_selection_change_time
            .map(|t| now.duration_since(t).as_millis() < CLICK_THRESHOLD_MS)
            .unwrap_or(false);

        if is_click {
            let is_double_click = self
                .last_confirm_time
                .map(|t| now.duration_since(t).as_millis() < DOUBLE_CLICK_THRESHOLD_MS)
                .unwrap_or(false);
            self.last_confirm_time = Some(now);

            if !is_double_click {
                return;
            }
        }

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

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

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
        let line_text_string = line_text.to_string();

        // Build search match range (takes precedence over syntax highlighting)
        let search_match_style = HighlightStyle {
            background_color: Some(cx.theme().colors().search_match_background),
            font_weight: Some(gpui::FontWeight::BOLD),
            ..Default::default()
        };

        let mut highlights: Vec<(Range<usize>, HighlightStyle)> = Vec::new();

        // Get syntax highlighting from the buffer
        {
            let line_start_abs = search_match.range.start - search_match.relative_range.start;
            let visible_start_abs = line_start_abs + trim_offset;
            let visible_end_abs = line_start_abs + original_line.len();
            let match_start_abs = search_match.range.start;
            let match_end_abs = search_match.range.end;

            // Determine the "effective" match range within the visible area
            let effective_match_start = match_start_abs.max(visible_start_abs);
            let effective_match_end = match_end_abs.min(visible_end_abs);

            let ranges = [
                (visible_start_abs..effective_match_start, false),
                (effective_match_start..effective_match_end, true),
                (effective_match_end..visible_end_abs, false),
            ];

            let snapshot = search_match.buffer.read(cx).snapshot();
            let syntax_theme = cx.theme().syntax();
            let mut current_offset = 0;

            for (range, is_match) in ranges {
                if range.start >= range.end {
                    continue;
                }

                for chunk in snapshot.chunks(range, true) {
                    let chunk_len = chunk.text.len();
                    let syntax_style = chunk
                        .syntax_highlight_id
                        .and_then(|id| id.style(&syntax_theme));

                    let style = if is_match {
                        let mut style = syntax_style.unwrap_or_default();
                        if let Some(bg) = search_match_style.background_color {
                            style.background_color = Some(bg);
                        }
                        if let Some(weight) = search_match_style.font_weight {
                            style.font_weight = Some(weight);
                        }
                        style
                    } else {
                        syntax_style.unwrap_or_default()
                    };

                    highlights.push((current_offset..current_offset + chunk_len, style));
                    current_offset += chunk_len;
                }
            }
        }

        let text_style = window.text_style();

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
                                .text_buffer(cx)
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
                                    div().pr_2().child(
                                        Label::new(search_match.line_number.to_string())
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    ),
                                ),
                        ),
                ),
        )
    }
}
