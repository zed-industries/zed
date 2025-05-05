use std::ops::Range;
use std::sync::Arc;

use assistant_context_editor::SavedContextMetadata;
use editor::{Editor, EditorEvent};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    App, Entity, FocusHandle, Focusable, ScrollStrategy, Stateful, Task, UniformListScrollHandle,
    WeakEntity, Window, uniform_list,
};
use time::{OffsetDateTime, UtcOffset};
use ui::{
    HighlightedLabel, IconButtonShape, ListItem, ListItemSpacing, Scrollbar, ScrollbarState,
    Tooltip, prelude::*,
};
use util::ResultExt;

use crate::history_store::{HistoryEntry, HistoryStore};
use crate::thread_store::SerializedThreadMetadata;
use crate::{AssistantPanel, RemoveSelectedThread};

pub struct ThreadHistory {
    assistant_panel: WeakEntity<AssistantPanel>,
    history_store: Entity<HistoryStore>,
    scroll_handle: UniformListScrollHandle,
    selected_index: usize,
    search_editor: Entity<Editor>,
    all_entries: Arc<Vec<HistoryEntry>>,
    list_content: ListContent,
    scrollbar_visibility: bool,
    scrollbar_state: ScrollbarState,
    _subscriptions: Vec<gpui::Subscription>,
}

#[derive(Default)]
enum ListContent {
    #[default]
    All,
    Searching {
        query: SharedString,
        _task: Task<()>,
    },
    Searched {
        query: SharedString,
        matches: Vec<StringMatch>,
    },
}

impl ThreadHistory {
    pub(crate) fn new(
        assistant_panel: WeakEntity<AssistantPanel>,
        history_store: Entity<HistoryStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let search_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search threads...", cx);
            editor
        });

        let search_editor_subscription =
            cx.subscribe(&search_editor, |this, search_editor, event, cx| {
                if let EditorEvent::BufferEdited = event {
                    let query = search_editor.read(cx).text(cx);
                    this.search(query.into(), cx);
                }
            });

        let history_store_subscription = cx.observe(&history_store, |this, _, cx| {
            this.update_all_entries(cx);
        });

        let scroll_handle = UniformListScrollHandle::default();
        let scrollbar_state = ScrollbarState::new(scroll_handle.clone());

        Self {
            assistant_panel,
            history_store,
            scroll_handle,
            selected_index: 0,
            list_content: ListContent::default(),
            all_entries: Default::default(),
            search_editor,
            _subscriptions: vec![search_editor_subscription, history_store_subscription],
            scrollbar_visibility: true,
            scrollbar_state,
        }
    }

    fn update_all_entries(&mut self, cx: &mut Context<Self>) {
        self.all_entries = self
            .history_store
            .update(cx, |store, cx| store.entries(cx))
            .into();

        self.set_selected_index(0, cx);

        match &self.list_content {
            ListContent::All => {}
            ListContent::Searching { query, .. } | ListContent::Searched { query, .. } => {
                self.search(query.clone(), cx);
            }
        }
        cx.notify();
    }

    fn search(&mut self, query: SharedString, cx: &mut Context<Self>) {
        let all_entries = self.all_entries.clone();

        let fuzzy_search_task = cx.background_spawn({
            let query = query.clone();
            let executor = cx.background_executor().clone();
            async move {
                let mut candidates = Vec::with_capacity(all_entries.len());

                for (idx, entry) in all_entries.iter().enumerate() {
                    match entry {
                        HistoryEntry::Thread(thread) => {
                            candidates.push(StringMatchCandidate::new(idx, &thread.summary));
                        }
                        HistoryEntry::Context(context) => {
                            candidates.push(StringMatchCandidate::new(idx, &context.title));
                        }
                    }
                }

                const MAX_MATCHES: usize = 100;

                fuzzy::match_strings(
                    &candidates,
                    &query,
                    false,
                    MAX_MATCHES,
                    &Default::default(),
                    executor,
                )
                .await
            }
        });

        let task = cx.spawn({
            let query = query.clone();
            async move |this, cx| {
                let matches = fuzzy_search_task.await;

                this.update(cx, |this, cx| {
                    let ListContent::Searching {
                        query: current_query,
                        _task,
                    } = &this.list_content
                    else {
                        return;
                    };

                    if &query != current_query {
                        this.list_content = ListContent::Searched {
                            query: query.clone(),
                            matches,
                        };

                        // todo! move to list content?
                        this.set_selected_index(0, cx);
                        cx.notify();
                    };
                })
                .log_err();
            }
        });

        self.list_content = ListContent::Searching {
            query: query.clone(),
            _task: task,
        };
        cx.notify();
    }

    fn matched_count(&self) -> usize {
        match &self.list_content {
            ListContent::All => self.all_entries.len(),
            ListContent::Searching { .. } => 0,
            ListContent::Searched { matches, .. } => matches.len(),
        }
    }

    fn search_produced_no_matches(&self) -> bool {
        match &self.list_content {
            ListContent::All => false,
            ListContent::Searching { .. } => false,
            ListContent::Searched { matches, .. } => matches.is_empty(),
        }
    }

    fn get_match(&self, ix: usize) -> Option<&HistoryEntry> {
        match &self.list_content {
            ListContent::All => self.all_entries.get(ix),
            ListContent::Searching { .. } => None,
            ListContent::Searched { matches, .. } => matches
                .get(ix)
                .and_then(|m| self.all_entries.get(m.candidate_id)),
        }
    }

    pub fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.matched_count();
        if count > 0 {
            if self.selected_index == 0 {
                self.set_selected_index(count - 1, cx);
            } else {
                self.set_selected_index(self.selected_index - 1, cx);
            }
        }
    }

    pub fn select_next(
        &mut self,
        _: &menu::SelectNext,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.matched_count();
        if count > 0 {
            if self.selected_index == count - 1 {
                self.set_selected_index(0, cx);
            } else {
                self.set_selected_index(self.selected_index + 1, cx);
            }
        }
    }

    fn select_first(
        &mut self,
        _: &menu::SelectFirst,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.matched_count();
        if count > 0 {
            self.set_selected_index(0, cx);
        }
    }

    fn select_last(&mut self, _: &menu::SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        let count = self.matched_count();
        if count > 0 {
            self.set_selected_index(count - 1, cx);
        }
    }

    fn set_selected_index(&mut self, index: usize, cx: &mut Context<Self>) {
        self.selected_index = index;
        self.scroll_handle
            .scroll_to_item(index, ScrollStrategy::Top);
        cx.notify();
    }

    fn render_scrollbar(&self, cx: &mut Context<Self>) -> Option<Stateful<Div>> {
        if !(self.scrollbar_visibility || self.scrollbar_state.is_dragging()) {
            return None;
        }

        Some(
            div()
                .occlude()
                .id("thread-history-scroll")
                .h_full()
                .bg(cx.theme().colors().panel_background.opacity(0.8))
                .border_l_1()
                .border_color(cx.theme().colors().border_variant)
                .absolute()
                .right_1()
                .top_0()
                .bottom_0()
                .w_4()
                .pl_1()
                .cursor_default()
                .on_mouse_move(cx.listener(|_, _, _window, cx| {
                    cx.notify();
                    cx.stop_propagation()
                }))
                .on_hover(|_, _window, cx| {
                    cx.stop_propagation();
                })
                .on_any_mouse_down(|_, _window, cx| {
                    cx.stop_propagation();
                })
                .on_scroll_wheel(cx.listener(|_, _, _window, cx| {
                    cx.notify();
                }))
                .children(Scrollbar::vertical(self.scrollbar_state.clone())),
        )
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(entry) = self.get_match(self.selected_index) {
            let task_result = match entry {
                HistoryEntry::Thread(thread) => self.assistant_panel.update(cx, move |this, cx| {
                    this.open_thread_by_id(&thread.id, window, cx)
                }),
                HistoryEntry::Context(context) => {
                    self.assistant_panel.update(cx, move |this, cx| {
                        this.open_saved_prompt_editor(context.path.clone(), window, cx)
                    })
                }
            };

            if let Some(task) = task_result.log_err() {
                task.detach_and_log_err(cx);
            };

            cx.notify();
        }
    }

    fn remove_selected_thread(
        &mut self,
        _: &RemoveSelectedThread,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(entry) = self.get_match(self.selected_index) {
            let task_result = match entry {
                HistoryEntry::Thread(thread) => self
                    .assistant_panel
                    .update(cx, |this, cx| this.delete_thread(&thread.id, cx)),
                HistoryEntry::Context(context) => self
                    .assistant_panel
                    .update(cx, |this, cx| this.delete_context(context.path.clone(), cx)),
            };

            if let Some(task) = task_result.log_err() {
                task.detach_and_log_err(cx);
            };

            cx.notify();
        }
    }

    fn list_items(
        &mut self,
        range: Range<usize>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Vec<Div> {
        let range_start = range.start;

        match &self.list_content {
            ListContent::All => self.all_entries[range]
                .iter()
                .enumerate()
                .map(|(index, entry)| self.render_list_item(index + range_start, entry, vec![]))
                .collect(),
            ListContent::Searched { matches, .. } => matches[range]
                .iter()
                .enumerate()
                .filter_map(|(index, m)| {
                    self.all_entries.get(m.candidate_id).map(|entry| {
                        self.render_list_item(index + range_start, entry, m.positions.clone())
                    })
                })
                .collect(),
            ListContent::Searching { .. } => {
                vec![]
            }
        }
    }

    fn render_list_item(
        &self,
        index: usize,
        entry: &HistoryEntry,
        highlight_positions: Vec<usize>,
    ) -> Div {
        h_flex().w_full().pb_1().child(match entry {
            HistoryEntry::Thread(thread) => PastThread::new(
                thread.clone(),
                self.assistant_panel.clone(),
                self.selected_index == index,
                highlight_positions,
            )
            .into_any_element(),
            HistoryEntry::Context(context) => PastContext::new(
                context.clone(),
                self.assistant_panel.clone(),
                self.selected_index == index,
                highlight_positions,
            )
            .into_any_element(),
        })
    }
}

impl Focusable for ThreadHistory {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.search_editor.focus_handle(cx)
    }
}

impl Render for ThreadHistory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("ThreadHistory")
            .size_full()
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::remove_selected_thread))
            .when(!self.all_entries.is_empty(), |parent| {
                parent.child(
                    h_flex()
                        .h(px(41.)) // Match the toolbar perfectly
                        .w_full()
                        .py_1()
                        .px_2()
                        .gap_2()
                        .justify_between()
                        .border_b_1()
                        .border_color(cx.theme().colors().border)
                        .child(
                            Icon::new(IconName::MagnifyingGlass)
                                .color(Color::Muted)
                                .size(IconSize::Small),
                        )
                        .child(self.search_editor.clone()),
                )
            })
            .child({
                let view = v_flex()
                    .id("list-container")
                    .relative()
                    .overflow_hidden()
                    .flex_grow();

                if self.all_entries.is_empty() {
                    view.justify_center()
                        .child(
                            h_flex().w_full().justify_center().child(
                                Label::new("You don't have any past threads yet.")
                                    .size(LabelSize::Small),
                            ),
                        )
                } else if self.search_produced_no_matches() {
                    view.justify_center().child(
                        h_flex().w_full().justify_center().child(
                            Label::new("No threads match your search.").size(LabelSize::Small),
                        ),
                    )
                } else {
                    view.pr_5()
                        .child(
                            uniform_list(
                                cx.entity().clone(),
                                "thread-history",
                                self.matched_count(),
                                Self::list_items,
                            )
                            .p_1()
                            .track_scroll(self.scroll_handle.clone())
                            .flex_grow(),
                        )
                        .when_some(self.render_scrollbar(cx), |div, scrollbar| {
                            div.child(scrollbar)
                        })
                }
            })
    }
}

#[derive(IntoElement)]
pub struct PastThread {
    thread: SerializedThreadMetadata,
    assistant_panel: WeakEntity<AssistantPanel>,
    selected: bool,
    highlight_positions: Vec<usize>,
}

impl PastThread {
    pub fn new(
        thread: SerializedThreadMetadata,
        assistant_panel: WeakEntity<AssistantPanel>,
        selected: bool,
        highlight_positions: Vec<usize>,
    ) -> Self {
        Self {
            thread,
            assistant_panel,
            selected,
            highlight_positions,
        }
    }
}

impl RenderOnce for PastThread {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let summary = self.thread.summary;

        let thread_timestamp = time_format::format_localized_timestamp(
            OffsetDateTime::from_unix_timestamp(self.thread.updated_at.timestamp()).unwrap(),
            OffsetDateTime::now_utc(),
            self.assistant_panel
                .update(cx, |this, _cx| this.local_timezone())
                .unwrap_or(UtcOffset::UTC),
            time_format::TimestampFormat::EnhancedAbsolute,
        );

        ListItem::new(SharedString::from(self.thread.id.to_string()))
            .rounded()
            .toggle_state(self.selected)
            .spacing(ListItemSpacing::Sparse)
            .start_slot(
                div().max_w_4_5().child(
                    HighlightedLabel::new(summary, self.highlight_positions)
                        .size(LabelSize::Small)
                        .truncate(),
                ),
            )
            .end_slot(
                h_flex()
                    .gap_1p5()
                    .child(
                        Label::new(thread_timestamp)
                            .color(Color::Muted)
                            .size(LabelSize::XSmall),
                    )
                    .child(
                        IconButton::new("delete", IconName::TrashAlt)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::XSmall)
                            .icon_color(Color::Muted)
                            .tooltip(move |window, cx| {
                                Tooltip::for_action("Delete", &RemoveSelectedThread, window, cx)
                            })
                            .on_click({
                                let assistant_panel = self.assistant_panel.clone();
                                let id = self.thread.id.clone();
                                move |_event, _window, cx| {
                                    assistant_panel
                                        .update(cx, |this, cx| {
                                            this.delete_thread(&id, cx).detach_and_log_err(cx);
                                        })
                                        .ok();
                                }
                            }),
                    ),
            )
            .on_click({
                let assistant_panel = self.assistant_panel.clone();
                let id = self.thread.id.clone();
                move |_event, window, cx| {
                    assistant_panel
                        .update(cx, |this, cx| {
                            this.open_thread_by_id(&id, window, cx)
                                .detach_and_log_err(cx);
                        })
                        .ok();
                }
            })
    }
}

#[derive(IntoElement)]
pub struct PastContext {
    context: SavedContextMetadata,
    assistant_panel: WeakEntity<AssistantPanel>,
    selected: bool,
    highlight_positions: Vec<usize>,
}

impl PastContext {
    pub fn new(
        context: SavedContextMetadata,
        assistant_panel: WeakEntity<AssistantPanel>,
        selected: bool,
        highlight_positions: Vec<usize>,
    ) -> Self {
        Self {
            context,
            assistant_panel,
            selected,
            highlight_positions,
        }
    }
}

impl RenderOnce for PastContext {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let summary = self.context.title;
        let context_timestamp = time_format::format_localized_timestamp(
            OffsetDateTime::from_unix_timestamp(self.context.mtime.timestamp()).unwrap(),
            OffsetDateTime::now_utc(),
            self.assistant_panel
                .update(cx, |this, _cx| this.local_timezone())
                .unwrap_or(UtcOffset::UTC),
            time_format::TimestampFormat::EnhancedAbsolute,
        );

        ListItem::new(SharedString::from(
            self.context.path.to_string_lossy().to_string(),
        ))
        .rounded()
        .toggle_state(self.selected)
        .spacing(ListItemSpacing::Sparse)
        .start_slot(
            div().max_w_4_5().child(
                HighlightedLabel::new(summary, self.highlight_positions)
                    .size(LabelSize::Small)
                    .truncate(),
            ),
        )
        .end_slot(
            h_flex()
                .gap_1p5()
                .child(
                    Label::new(context_timestamp)
                        .color(Color::Muted)
                        .size(LabelSize::XSmall),
                )
                .child(
                    IconButton::new("delete", IconName::TrashAlt)
                        .shape(IconButtonShape::Square)
                        .icon_size(IconSize::XSmall)
                        .icon_color(Color::Muted)
                        .tooltip(move |window, cx| {
                            Tooltip::for_action("Delete", &RemoveSelectedThread, window, cx)
                        })
                        .on_click({
                            let assistant_panel = self.assistant_panel.clone();
                            let path = self.context.path.clone();
                            move |_event, _window, cx| {
                                assistant_panel
                                    .update(cx, |this, cx| {
                                        this.delete_context(path.clone(), cx)
                                            .detach_and_log_err(cx);
                                    })
                                    .ok();
                            }
                        }),
                ),
        )
        .on_click({
            let assistant_panel = self.assistant_panel.clone();
            let path = self.context.path.clone();
            move |_event, window, cx| {
                assistant_panel
                    .update(cx, |this, cx| {
                        this.open_saved_prompt_editor(path.clone(), window, cx)
                            .detach_and_log_err(cx);
                    })
                    .ok();
            }
        })
    }
}
