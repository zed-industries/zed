use collections::{HashMap, HashSet};
use std::ops::Range;
use std::sync::Arc;
use std::time::Duration;

use crate::quick_search::delegate::QuickSearchDelegate;
use crate::quick_search::state::{
    SavedQuickSearchLayout, StackedLayoutState, TelescopeLayoutState,
};
use crate::{EXCLUDE_PLACEHOLDER, INCLUDE_PLACEHOLDER, REPLACE_PLACEHOLDER};
use editor::{Editor, EditorEvent};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, MouseDownEvent,
    Subscription, Task, WeakEntity, Window, actions,
};
use language::Buffer;
use multi_buffer::MultiBuffer;
use picker::Picker;
use project::search::SearchInputKind;
use project::{Project, ProjectPath};
use text::{Anchor, ToOffset};
use ui::prelude::*;
use ui_input::ErasedEditor;
use util::ResultExt;
use workspace::{
    DismissDecision, ModalView, SplitDirection, Workspace, pane, searchable::SearchableItem,
};
pub use zed_actions::quick_search::Toggle;

mod delegate;
mod render;
mod state;

actions!(
    quick_search,
    [
        ReplaceNext,
        ReplaceAll,
        ToggleFilters,
        ToggleLayout,
        ToggleSplitMenu,
        ToggleHistory
    ]
);

const SEARCH_DEBOUNCE_MS: u64 = 100;
const AUTOSAVE_DELAY_MS: u64 = 500;
const RESIZE_CORNER_HANDLE_SIZE: f32 = 24.0;
const CLICK_THRESHOLD_MS: u128 = 50;
const DOUBLE_CLICK_THRESHOLD_MS: u128 = 300;
const SEARCH_RESULTS_BATCH_SIZE: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayoutMode {
    #[default]
    Stacked,
    Telescope,
}

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
    picker: Entity<Picker<delegate::QuickSearchDelegate>>,
    preview_editor: Entity<Editor>,
    replacement_editor: Arc<dyn ErasedEditor>,
    focus_handle: FocusHandle,
    offset: gpui::Point<Pixels>,
    modal_width: Pixels,
    layout_mode: LayoutMode,
    stacked: state::StackedLayoutState,
    telescope: state::TelescopeLayoutState,
    _subscriptions: Vec<Subscription>,
    _autosave_task: Option<Task<()>>,
}

#[derive(Clone, Copy)]
struct QuickSearchDrag {
    mouse_start: gpui::Point<Pixels>,
    offset_start: gpui::Point<Pixels>,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ResizeSide {
    Start,
    End,
}

fn handle_resize_mouse_down(_: &MouseDownEvent, window: &mut Window, cx: &mut App) {
    window.prevent_default();
    cx.stop_propagation();
}

fn resize_hover_handler(is_highlighted: Entity<bool>) -> impl Fn(&bool, &mut Window, &mut App) {
    move |&hovered, _window, cx| is_highlighted.write(cx, hovered)
}

fn clear_resize_highlight<T>(is_highlighted: Entity<bool>) -> impl Fn(&T, &mut Window, &mut App) {
    move |_, _, cx| is_highlighted.write(cx, false)
}

impl ModalView for QuickSearch {
    fn on_before_dismiss(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> DismissDecision {
        self.save_layout(cx);
        DismissDecision::Dismiss(true)
    }

    fn render_bare(&self) -> bool {
        true
    }
}

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
                let query =
                    editor.update(cx, |editor, cx| editor.query_suggestion(false, window, cx));
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

        let initial_query = initial_query.or_else(|| {
            project
                .read(cx)
                .search_history(SearchInputKind::Query)
                .iter()
                .next()
                .map(|s| s.to_string())
        });

        let delegate = delegate::QuickSearchDelegate::new(
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
                .max_height(None)
        });

        let this = cx.entity().downgrade();
        let subscriptions = vec![
            cx.observe(&picker, |_, _, cx| cx.notify()),
            save_history_on_dismiss(picker.clone(), window, cx),
            refresh_picker_on_editor_edit(&included_files_editor, this.clone(), window, cx),
            refresh_picker_on_editor_edit(&excluded_files_editor, this, window, cx),
            cx.on_focus_out(&focus_handle, window, |this, _, window, cx| {
                if window.is_window_active() && !this.focus_handle.contains_focused(window, cx) {
                    this.save_history(cx);
                    cx.emit(DismissEvent);
                }
            }),
            auto_save_when_edited(&preview_editor, window, cx),
        ];

        let (modal_width, layout_mode, stacked, telescope) =
            if let Some(saved) = cx.try_global::<SavedQuickSearchLayout>() {
                (
                    saved.modal_width,
                    saved.layout_mode,
                    StackedLayoutState {
                        results_height: saved.stacked_results_height,
                        preview_height: saved.stacked_preview_height,
                    },
                    TelescopeLayoutState {
                        content_height: saved.telescope_content_height,
                        preview_width: saved.telescope_preview_width,
                    },
                )
            } else {
                let modal_width =
                    rems(StackedLayoutState::DEFAULT_MODAL_WIDTH_REMS).to_pixels(window.rem_size());
                (
                    modal_width,
                    LayoutMode::default(),
                    StackedLayoutState::new(),
                    TelescopeLayoutState::new(),
                )
            };

        Self {
            picker,
            preview_editor,
            replacement_editor,
            focus_handle,
            offset: gpui::Point::default(),
            modal_width,
            layout_mode,
            stacked,
            telescope,
            _subscriptions: subscriptions,
            _autosave_task: None,
        }
    }

    fn save_history(&mut self, cx: &mut Context<Self>) {
        self.picker.update(cx, |picker, cx| {
            let query = picker.query(cx);

            if query.is_empty() {
                return;
            }

            let delegate = &mut picker.delegate;
            delegate.project.update(cx, |project, _| {
                let last_query = project.search_history(SearchInputKind::Query).iter().next();
                if last_query != Some(query.as_str()) {
                    project
                        .search_history_mut(SearchInputKind::Query)
                        .add(&mut delegate.search_history_cursor, query);
                }
            });
        });
    }

    fn save_layout(&self, cx: &mut Context<Self>) {
        cx.set_global(SavedQuickSearchLayout {
            modal_width: self.modal_width,
            layout_mode: self.layout_mode,
            stacked_results_height: self.stacked.results_height,
            stacked_preview_height: self.stacked.preview_height,
            telescope_content_height: self.telescope.content_height,
            telescope_preview_width: self.telescope.preview_width,
        });
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
                        HistoryDirection::Previous => history.previous(cursor, &query_text),
                    }
                    .map(str::to_string);
                }

                if result.as_deref() == Some(query_text.as_str()) {
                    result = match direction {
                        HistoryDirection::Next => history.next(cursor),
                        HistoryDirection::Previous => history.previous(cursor, &query_text),
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

        self.save_history(cx);
        cx.emit(DismissEvent);

        workspace
            .update(cx, |workspace, cx| {
                workspace.split_path_preview(path, false, Some(split_direction), window, cx)
            })
            .detach_and_log_err(cx);
    }
}

fn auto_save_when_edited(
    preview_editor: &Entity<Editor>,
    window: &mut Window,
    cx: &mut Context<'_, QuickSearch>,
) -> Subscription {
    cx.subscribe_in(
        preview_editor,
        window,
        |this, _, event: &EditorEvent, window, cx| {
            if matches!(event, EditorEvent::Edited { .. }) {
                this._autosave_task = Some(cx.spawn_in(window, async move |this, cx| {
                    // TODO!(yara) this should hook into the normal autosave mechanic
                    // not be done here
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
    )
}

fn refresh_picker_on_editor_edit(
    editor: &Arc<dyn ErasedEditor>,
    quick_search: WeakEntity<QuickSearch>,
    window: &mut Window,
    cx: &mut Context<'_, QuickSearch>,
) -> Subscription {
    editor.subscribe(
        Box::new(move |event, window, cx| {
            if matches!(event, ui_input::ErasedEditorEvent::BufferEdited) {
                quick_search
                    .update(cx, |quick_search, cx| {
                        quick_search.picker.update(cx, |picker, cx| {
                            picker.refresh(window, cx);
                        });
                    })
                    .log_err();
            }
        }),
        window,
        cx,
    )
}

fn save_history_on_dismiss(
    picker: Entity<Picker<QuickSearchDelegate>>,
    window: &mut Window,
    cx: &mut Context<'_, QuickSearch>,
) -> Subscription {
    cx.subscribe_in(&picker, window, |this, _, _: &DismissEvent, _, cx| {
        this.save_history(cx);
        cx.emit(DismissEvent);
    })
}
