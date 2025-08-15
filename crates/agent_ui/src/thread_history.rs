use crate::{AgentPanel, RemoveSelectedThread};
use agent::history_store::{HistoryEntry, HistoryStore};
use chrono::{Datelike as _, Local, NaiveDate, TimeDelta};
use editor::{Editor, EditorEvent};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    App, ClickEvent, Empty, Entity, FocusHandle, Focusable, ScrollStrategy, Stateful, Task,
    UniformListScrollHandle, WeakEntity, Window, uniform_list,
};
use std::{fmt::Display, ops::Range, sync::Arc};
use time::{OffsetDateTime, UtcOffset};
use ui::{
    HighlightedLabel, IconButtonShape, ListItem, ListItemSpacing, Scrollbar, ScrollbarState,
    Tooltip, prelude::*,
};
use util::ResultExt;

pub struct ThreadHistory {
    agent_panel: WeakEntity<AgentPanel>,
    history_store: Entity<HistoryStore>,
    scroll_handle: UniformListScrollHandle,
    selected_index: usize,
    hovered_index: Option<usize>,
    search_editor: Entity<Editor>,
    all_entries: Arc<Vec<HistoryEntry>>,
    // When the search is empty, we display date separators between history entries
    // This vector contains an enum of either a separator or an actual entry
    separated_items: Vec<ListItemType>,
    // Maps entry indexes to list item indexes
    separated_item_indexes: Vec<u32>,
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

enum ListItemType {
    BucketSeparator(TimeBucket),
    Entry {
        index: usize,
        format: EntryTimeFormat,
    },
}

impl ListItemType {
    fn entry_index(&self) -> Option<usize> {
        match self {
            ListItemType::BucketSeparator(_) => None,
            ListItemType::Entry { index, .. } => Some(*index),
        }
    }
}

impl ThreadHistory {
    pub(crate) fn new(
        agent_panel: WeakEntity<AgentPanel>,
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
            agent_panel,
            history_store,
            scroll_handle,
            selected_index: 0,
            hovered_index: None,
            search_state: SearchState::Empty,
            all_entries: Default::default(),
            separated_items: Default::default(),
            separated_item_indexes: Default::default(),
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
        let new_entries: Arc<Vec<HistoryEntry>> = self
            .history_store
            .update(cx, |store, cx| store.entries(cx))
            .into();

        self._separated_items_task.take();

        let mut items = Vec::with_capacity(new_entries.len() + 1);
        let mut indexes = Vec::with_capacity(new_entries.len() + 1);

        let bg_task = cx.background_spawn(async move {
            let mut bucket = None;
            let today = Local::now().naive_local().date();

            for (index, entry) in new_entries.iter().enumerate() {
                let entry_date = entry
                    .updated_at()
                    .with_timezone(&Local)
                    .naive_local()
                    .date();
                let entry_bucket = TimeBucket::from_dates(today, entry_date);

                if Some(entry_bucket) != bucket {
                    bucket = Some(entry_bucket);
                    items.push(ListItemType::BucketSeparator(entry_bucket));
                }

                indexes.push(items.len() as u32);
                items.push(ListItemType::Entry {
                    index,
                    format: entry_bucket.into(),
                });
            }
            (new_entries, items, indexes)
        });

        let task = cx.spawn(async move |this, cx| {
            let (new_entries, items, indexes) = bg_task.await;
            this.update(cx, |this, cx| {
                let previously_selected_entry =
                    this.all_entries.get(this.selected_index).map(|e| e.id());

                this.all_entries = new_entries;
                this.separated_items = items;
                this.separated_item_indexes = indexes;

                match &this.search_state {
                    SearchState::Empty => {
                        if this.selected_index >= this.all_entries.len() {
                            this.set_selected_entry_index(
                                this.all_entries.len().saturating_sub(1),
                                cx,
                            );
                        } else if let Some(prev_id) = previously_selected_entry {
                            if let Some(new_ix) = this
                                .all_entries
                                .iter()
                                .position(|probe| probe.id() == prev_id)
                            {
                                this.set_selected_entry_index(new_ix, cx);
                            }
                        }
                    }
                    SearchState::Searching { query, .. } | SearchState::Searched { query, .. } => {
                        this.search(query.clone(), cx);
                    }
                }

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
                    true,
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

                        this.set_selected_entry_index(0, cx);
                        cx.notify();
                    };
                })
                .log_err();
            }
        });

        self.search_state = SearchState::Searching { query, _task: task };
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
                self.set_selected_entry_index(count - 1, cx);
            } else {
                self.set_selected_entry_index(self.selected_index - 1, cx);
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
                self.set_selected_entry_index(0, cx);
            } else {
                self.set_selected_entry_index(self.selected_index + 1, cx);
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
            self.set_selected_entry_index(0, cx);
        }
    }

    fn select_last(&mut self, _: &menu::SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        let count = self.matched_count();
        if count > 0 {
            self.set_selected_entry_index(count - 1, cx);
        }
    }

    fn set_selected_entry_index(&mut self, entry_index: usize, cx: &mut Context<Self>) {
        self.selected_index = entry_index;

        let scroll_ix = match self.search_state {
            SearchState::Empty | SearchState::Searching { .. } => self
                .separated_item_indexes
                .get(entry_index)
                .map(|ix| *ix as usize)
                .unwrap_or(entry_index + 1),
            SearchState::Searched { .. } => entry_index,
        };

        self.scroll_handle
            .scroll_to_item(scroll_ix, ScrollStrategy::Top);

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
                HistoryEntry::Thread(thread) => self.agent_panel.update(cx, move |this, cx| {
                    this.open_thread_by_id(&thread.id, window, cx)
                }),
                HistoryEntry::Context(context) => self.agent_panel.update(cx, move |this, cx| {
                    this.open_saved_prompt_editor(context.path.clone(), window, cx)
                }),
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
                    .agent_panel
                    .update(cx, |this, cx| this.delete_thread(&thread.id, cx)),
                HistoryEntry::Context(context) => self
                    .agent_panel
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
                        &ListItemType::Entry {
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
        item: &ListItemType,
        highlight_positions: Vec<usize>,
        cx: &Context<Self>,
    ) -> AnyElement {
        match item {
            ListItemType::Entry { index, format } => match self.all_entries.get(*index) {
                Some(entry) => h_flex()
                    .w_full()
                    .pb_1()
                    .child(
                        HistoryEntryElement::new(entry.clone(), self.agent_panel.clone())
                            .highlight_positions(highlight_positions)
                            .timestamp_format(*format)
                            .selected(list_entry_ix == Some(self.selected_index))
                            .hovered(list_entry_ix == self.hovered_index)
                            .on_hover(cx.listener(move |this, is_hovered, _window, cx| {
                                if *is_hovered {
                                    this.hovered_index = list_entry_ix;
                                } else if this.hovered_index == list_entry_ix {
                                    this.hovered_index = None;
                                }

                                cx.notify();
                            }))
                            .into_any_element(),
                    )
                    .into_any(),
                None => Empty.into_any_element(),
            },
            ListItemType::BucketSeparator(bucket) => div()
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
                                "thread-history",
                                self.list_item_count(),
                                cx.processor(|this, range: Range<usize>, window, cx| {
                                    this.list_items(range, window, cx)
                                }),
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
pub struct HistoryEntryElement {
    entry: HistoryEntry,
    agent_panel: WeakEntity<AgentPanel>,
    selected: bool,
    hovered: bool,
    highlight_positions: Vec<usize>,
    timestamp_format: EntryTimeFormat,
    on_hover: Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>,
}

impl HistoryEntryElement {
    pub fn new(entry: HistoryEntry, agent_panel: WeakEntity<AgentPanel>) -> Self {
        Self {
            entry,
            agent_panel,
            selected: false,
            hovered: false,
            highlight_positions: vec![],
            timestamp_format: EntryTimeFormat::DateAndTime,
            on_hover: Box::new(|_, _, _| {}),
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }

    pub fn highlight_positions(mut self, positions: Vec<usize>) -> Self {
        self.highlight_positions = positions;
        self
    }

    pub fn on_hover(mut self, on_hover: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_hover = Box::new(on_hover);
        self
    }

    pub fn timestamp_format(mut self, format: EntryTimeFormat) -> Self {
        self.timestamp_format = format;
        self
    }
}

impl RenderOnce for HistoryEntryElement {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (id, summary, timestamp) = match &self.entry {
            HistoryEntry::Thread(thread) => (
                thread.id.to_string(),
                thread.summary.clone(),
                thread.updated_at.timestamp(),
            ),
            HistoryEntry::Context(context) => (
                context.path.to_string_lossy().to_string(),
                context.title.clone(),
                context.mtime.timestamp(),
            ),
        };

        let thread_timestamp =
            self.timestamp_format
                .format_timestamp(&self.agent_panel, timestamp, cx);

        ListItem::new(SharedString::from(id))
            .rounded()
            .toggle_state(self.selected)
            .spacing(ListItemSpacing::Sparse)
            .start_slot(
                h_flex()
                    .w_full()
                    .gap_2()
                    .justify_between()
                    .child(
                        HighlightedLabel::new(summary, self.highlight_positions)
                            .size(LabelSize::Small)
                            .truncate(),
                    )
                    .child(
                        Label::new(thread_timestamp)
                            .color(Color::Muted)
                            .size(LabelSize::XSmall),
                    ),
            )
            .on_hover(self.on_hover)
            .end_slot::<IconButton>(if self.hovered || self.selected {
                Some(
                    IconButton::new("delete", IconName::Trash)
                        .shape(IconButtonShape::Square)
                        .icon_size(IconSize::XSmall)
                        .icon_color(Color::Muted)
                        .tooltip(move |window, cx| {
                            Tooltip::for_action("Delete", &RemoveSelectedThread, window, cx)
                        })
                        .on_click({
                            let agent_panel = self.agent_panel.clone();

                            let f: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static> =
                                match &self.entry {
                                    HistoryEntry::Thread(thread) => {
                                        let id = thread.id.clone();

                                        Box::new(move |_event, _window, cx| {
                                            agent_panel
                                                .update(cx, |this, cx| {
                                                    this.delete_thread(&id, cx)
                                                        .detach_and_log_err(cx);
                                                })
                                                .ok();
                                        })
                                    }
                                    HistoryEntry::Context(context) => {
                                        let path = context.path.clone();

                                        Box::new(move |_event, _window, cx| {
                                            agent_panel
                                                .update(cx, |this, cx| {
                                                    this.delete_context(path.clone(), cx)
                                                        .detach_and_log_err(cx);
                                                })
                                                .ok();
                                        })
                                    }
                                };
                            f
                        }),
                )
            } else {
                None
            })
            .on_click({
                let agent_panel = self.agent_panel.clone();

                let f: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static> = match &self.entry
                {
                    HistoryEntry::Thread(thread) => {
                        let id = thread.id.clone();
                        Box::new(move |_event, window, cx| {
                            agent_panel
                                .update(cx, |this, cx| {
                                    this.open_thread_by_id(&id, window, cx)
                                        .detach_and_log_err(cx);
                                })
                                .ok();
                        })
                    }
                    HistoryEntry::Context(context) => {
                        let path = context.path.clone();
                        Box::new(move |_event, window, cx| {
                            agent_panel
                                .update(cx, |this, cx| {
                                    this.open_saved_prompt_editor(path.clone(), window, cx)
                                        .detach_and_log_err(cx);
                                })
                                .ok();
                        })
                    }
                };
                f
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
        agent_panel: &WeakEntity<AgentPanel>,
        timestamp: i64,
        cx: &App,
    ) -> String {
        let timestamp = OffsetDateTime::from_unix_timestamp(timestamp).unwrap();
        let timezone = agent_panel
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
