use std::fmt::Display;
use std::ops::Range;
use std::sync::Arc;

use assistant_context_editor::SavedContextMetadata;
use chrono::{Datelike as _, Local, NaiveDate, TimeDelta};
use editor::{Editor, EditorEvent};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    App, Empty, Entity, FocusHandle, Focusable, ScrollStrategy, Stateful, Task,
    UniformListScrollHandle, WeakEntity, Window, uniform_list,
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
    // When the search is empty, we display date separators between history entries
    // This vector contains an enum of either a separator or an actual entry
    separated_items: Vec<HistoryListItem>,
    _separated_items_task: Option<Task<()>>,
    search_state: SearchState,
    scrollbar_visibility: bool,
    scrollbar_state: ScrollbarState,
    _subscriptions: Vec<gpui::Subscription>,
}

enum SearchState {
    Empty,
    Searching {
        query: SharedString,
        _task: Task<()>,
    },
    Searched {
        query: SharedString,
        matches: Vec<StringMatch>,
    },
}

enum HistoryListItem {
    BucketSeparator(TimeBucket),
    Entry {
        index: usize,
        format: EntryTimeFormat,
    },
}

impl HistoryListItem {
    fn entry_index(&self) -> Option<usize> {
        match self {
            HistoryListItem::BucketSeparator(_) => None,
            HistoryListItem::Entry { index, .. } => Some(*index),
        }
    }
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

        let mut this = Self {
            assistant_panel,
            history_store,
            scroll_handle,
            selected_index: 0,
            search_state: SearchState::Empty,
            all_entries: Default::default(),
            separated_items: Default::default(),
            search_editor,
            scrollbar_visibility: true,
            scrollbar_state,
            _subscriptions: vec![search_editor_subscription, history_store_subscription],
            _separated_items_task: None,
        };
        this.update_all_entries(cx);
        this
    }

    fn update_all_entries(&mut self, cx: &mut Context<Self>) {
        self.all_entries = self
            .history_store
            .update(cx, |store, cx| store.entries(cx))
            .into();

        self.set_selected_index(0, cx);
        self.update_separated_items(cx);

        match &self.search_state {
            SearchState::Empty => {}
            SearchState::Searching { query, .. } | SearchState::Searched { query, .. } => {
                self.search(query.clone(), cx);
            }
        }
        cx.notify();
    }

    fn update_separated_items(&mut self, cx: &mut Context<Self>) {
        self._separated_items_task.take();

        let mut separated_items = std::mem::take(&mut self.separated_items);
        separated_items.clear();
        let all_entries = self.all_entries.clone();

        let bg_task = cx.background_spawn(async move {
            let mut bucket = None;
            let today = Local::now().naive_local().date();

            for (index, entry) in all_entries.iter().enumerate() {
                let entry_date = entry
                    .updated_at()
                    .with_timezone(&Local)
                    .naive_local()
                    .date();
                let entry_bucket = TimeBucket::from_dates(today, entry_date);

                if Some(entry_bucket) != bucket {
                    bucket = Some(entry_bucket);
                    separated_items.push(HistoryListItem::BucketSeparator(entry_bucket));
                }
                separated_items.push(HistoryListItem::Entry {
                    index,
                    format: entry_bucket.into(),
                });
            }
            separated_items
        });

        let task = cx.spawn(async move |this, cx| {
            let separated_items = bg_task.await;
            this.update(cx, |this, cx| {
                this.separated_items = separated_items;
                cx.notify();
            })
            .log_err();
        });
        self._separated_items_task = Some(task);
    }

    fn search(&mut self, query: SharedString, cx: &mut Context<Self>) {
        if query.is_empty() {
            self.search_state = SearchState::Empty;
            cx.notify();
            return;
        }

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
                    let SearchState::Searching {
                        query: current_query,
                        _task,
                    } = &this.search_state
                    else {
                        return;
                    };

                    if &query == current_query {
                        this.search_state = SearchState::Searched {
                            query: query.clone(),
                            matches,
                        };

                        this.set_selected_index(0, cx);
                        cx.notify();
                    };
                })
                .log_err();
            }
        });

        self.search_state = SearchState::Searching {
            query: query.clone(),
            _task: task,
        };
        cx.notify();
    }

    fn matched_count(&self) -> usize {
        match &self.search_state {
            SearchState::Empty => self.all_entries.len(),
            SearchState::Searching { .. } => 0,
            SearchState::Searched { matches, .. } => matches.len(),
        }
    }

    fn list_item_count(&self) -> usize {
        match &self.search_state {
            SearchState::Empty => self.separated_items.len(),
            SearchState::Searching { .. } => 0,
            SearchState::Searched { matches, .. } => matches.len(),
        }
    }

    fn search_produced_no_matches(&self) -> bool {
        match &self.search_state {
            SearchState::Empty => false,
            SearchState::Searching { .. } => false,
            SearchState::Searched { matches, .. } => matches.is_empty(),
        }
    }

    fn get_match(&self, ix: usize) -> Option<&HistoryEntry> {
        match &self.search_state {
            SearchState::Empty => self.all_entries.get(ix),
            SearchState::Searching { .. } => None,
            SearchState::Searched { matches, .. } => matches
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
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let range_start = range.start;

        match &self.search_state {
            SearchState::Empty => self
                .separated_items
                .get(range)
                .iter()
                .flat_map(|items| {
                    items
                        .iter()
                        .map(|item| self.render_list_item(item.entry_index(), item, vec![], cx))
                })
                .collect(),
            SearchState::Searched { matches, .. } => matches[range]
                .iter()
                .enumerate()
                .map(|(ix, m)| {
                    self.render_list_item(
                        Some(range_start + ix),
                        &HistoryListItem::Entry {
                            index: m.candidate_id,
                            format: EntryTimeFormat::DateAndTime,
                        },
                        m.positions.clone(),
                        cx,
                    )
                })
                .collect(),
            SearchState::Searching { .. } => {
                vec![]
            }
        }
    }

    fn render_list_item(
        &self,
        list_entry_ix: Option<usize>,
        item: &HistoryListItem,
        highlight_positions: Vec<usize>,
        cx: &App,
    ) -> AnyElement {
        match item {
            HistoryListItem::Entry { index, format } => match self.all_entries.get(*index) {
                Some(entry) => h_flex()
                    .w_full()
                    .pb_1()
                    .child(self.render_history_entry(
                        entry,
                        list_entry_ix == Some(self.selected_index),
                        highlight_positions,
                        *format,
                    ))
                    .into_any(),
                None => Empty.into_any_element(),
            },
            HistoryListItem::BucketSeparator(bucket) => div()
                .px(DynamicSpacing::Base06.rems(cx))
                .pt_2()
                .pb_1()
                .child(
                    Label::new(bucket.to_string())
                        .size(LabelSize::XSmall)
                        .color(Color::Muted),
                )
                .into_any_element(),
        }
    }

    fn render_history_entry(
        &self,
        entry: &HistoryEntry,
        is_active: bool,
        highlight_positions: Vec<usize>,
        format: EntryTimeFormat,
    ) -> AnyElement {
        match entry {
            HistoryEntry::Thread(thread) => PastThread::new(
                thread.clone(),
                self.assistant_panel.clone(),
                is_active,
                highlight_positions,
                format,
            )
            .into_any_element(),
            HistoryEntry::Context(context) => PastContext::new(
                context.clone(),
                self.assistant_panel.clone(),
                is_active,
                highlight_positions,
                format,
            )
            .into_any_element(),
        }
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
                                self.list_item_count(),
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
    timestamp_format: EntryTimeFormat,
}

impl PastThread {
    pub fn new(
        thread: SerializedThreadMetadata,
        assistant_panel: WeakEntity<AssistantPanel>,
        selected: bool,
        highlight_positions: Vec<usize>,
        timestamp_format: EntryTimeFormat,
    ) -> Self {
        Self {
            thread,
            assistant_panel,
            selected,
            highlight_positions,
            timestamp_format,
        }
    }
}

impl RenderOnce for PastThread {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let summary = self.thread.summary;

        let thread_timestamp = self.timestamp_format.format_timestamp(
            &self.assistant_panel,
            self.thread.updated_at.timestamp(),
            cx,
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
    timestamp_format: EntryTimeFormat,
}

impl PastContext {
    pub fn new(
        context: SavedContextMetadata,
        assistant_panel: WeakEntity<AssistantPanel>,
        selected: bool,
        highlight_positions: Vec<usize>,
        timestamp_format: EntryTimeFormat,
    ) -> Self {
        Self {
            context,
            assistant_panel,
            selected,
            highlight_positions,
            timestamp_format,
        }
    }
}

impl RenderOnce for PastContext {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let summary = self.context.title;
        let context_timestamp = self.timestamp_format.format_timestamp(
            &self.assistant_panel,
            self.context.mtime.timestamp(),
            cx,
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

#[derive(Clone, Copy)]
pub enum EntryTimeFormat {
    DateAndTime,
    TimeOnly,
}

impl EntryTimeFormat {
    fn format_timestamp(
        &self,
        assistant_panel: &WeakEntity<AssistantPanel>,
        timestamp: i64,
        cx: &App,
    ) -> String {
        let timestamp = OffsetDateTime::from_unix_timestamp(timestamp).unwrap();
        let timezone = assistant_panel
            .read_with(cx, |this, _cx| this.local_timezone())
            .unwrap_or(UtcOffset::UTC);

        match &self {
            EntryTimeFormat::DateAndTime => time_format::format_localized_timestamp(
                timestamp,
                OffsetDateTime::now_utc(),
                timezone,
                time_format::TimestampFormat::EnhancedAbsolute,
            ),
            EntryTimeFormat::TimeOnly => time_format::format_time(timestamp),
        }
    }
}

impl From<TimeBucket> for EntryTimeFormat {
    fn from(bucket: TimeBucket) -> Self {
        match bucket {
            TimeBucket::Today => EntryTimeFormat::TimeOnly,
            TimeBucket::Yesterday => EntryTimeFormat::TimeOnly,
            TimeBucket::ThisWeek => EntryTimeFormat::DateAndTime,
            TimeBucket::PastWeek => EntryTimeFormat::DateAndTime,
            TimeBucket::All => EntryTimeFormat::DateAndTime,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum TimeBucket {
    Today,
    Yesterday,
    ThisWeek,
    PastWeek,
    All,
}

impl TimeBucket {
    fn from_dates(reference: NaiveDate, date: NaiveDate) -> Self {
        if date == reference {
            return TimeBucket::Today;
        }

        if date == reference - TimeDelta::days(1) {
            return TimeBucket::Yesterday;
        }

        let week = date.iso_week();

        if reference.iso_week() == week {
            return TimeBucket::ThisWeek;
        }

        let last_week = (reference - TimeDelta::days(7)).iso_week();

        if week == last_week {
            return TimeBucket::PastWeek;
        }

        TimeBucket::All
    }
}

impl Display for TimeBucket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeBucket::Today => write!(f, "Today"),
            TimeBucket::Yesterday => write!(f, "Yesterday"),
            TimeBucket::ThisWeek => write!(f, "This Week"),
            TimeBucket::PastWeek => write!(f, "Past Week"),
            TimeBucket::All => write!(f, "All"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_time_bucket_from_dates() {
        let today = NaiveDate::from_ymd_opt(2023, 1, 15).unwrap();

        let date = today;
        assert_eq!(TimeBucket::from_dates(today, date), TimeBucket::Today);

        let date = NaiveDate::from_ymd_opt(2023, 1, 14).unwrap();
        assert_eq!(TimeBucket::from_dates(today, date), TimeBucket::Yesterday);

        let date = NaiveDate::from_ymd_opt(2023, 1, 13).unwrap();
        assert_eq!(TimeBucket::from_dates(today, date), TimeBucket::ThisWeek);

        let date = NaiveDate::from_ymd_opt(2023, 1, 11).unwrap();
        assert_eq!(TimeBucket::from_dates(today, date), TimeBucket::ThisWeek);

        let date = NaiveDate::from_ymd_opt(2023, 1, 8).unwrap();
        assert_eq!(TimeBucket::from_dates(today, date), TimeBucket::PastWeek);

        let date = NaiveDate::from_ymd_opt(2023, 1, 5).unwrap();
        assert_eq!(TimeBucket::from_dates(today, date), TimeBucket::PastWeek);

        // All: not in this week or last week
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        assert_eq!(TimeBucket::from_dates(today, date), TimeBucket::All);

        // Test year boundary cases
        let new_year = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();

        let date = NaiveDate::from_ymd_opt(2022, 12, 31).unwrap();
        assert_eq!(
            TimeBucket::from_dates(new_year, date),
            TimeBucket::Yesterday
        );

        let date = NaiveDate::from_ymd_opt(2022, 12, 28).unwrap();
        assert_eq!(TimeBucket::from_dates(new_year, date), TimeBucket::ThisWeek);
    }
}
