use crate::acp::AcpThreadView;
use crate::{AgentPanel, RemoveSelectedThread};
use agent2::{HistoryEntry, HistoryStore};
use chrono::{Datelike as _, Local, NaiveDate, TimeDelta};
use editor::{Editor, EditorEvent};
use fuzzy::StringMatchCandidate;
use gpui::{
    App, Entity, EventEmitter, FocusHandle, Focusable, ScrollStrategy, Stateful, Task,
    UniformListScrollHandle, WeakEntity, Window, uniform_list,
};
use std::{fmt::Display, ops::Range};
use text::Bias;
use time::{OffsetDateTime, UtcOffset};
use ui::{
    HighlightedLabel, IconButtonShape, ListItem, ListItemSpacing, Scrollbar, ScrollbarState,
    Tooltip, prelude::*,
};

pub struct AcpThreadHistory {
    pub(crate) history_store: Entity<HistoryStore>,
    scroll_handle: UniformListScrollHandle,
    selected_index: usize,
    hovered_index: Option<usize>,
    search_editor: Entity<Editor>,
    search_query: SharedString,

    visible_items: Vec<ListItemType>,

    scrollbar_visibility: bool,
    scrollbar_state: ScrollbarState,
    local_timezone: UtcOffset,

    _update_task: Task<()>,
    _subscriptions: Vec<gpui::Subscription>,
}

enum ListItemType {
    BucketSeparator(TimeBucket),
    Entry {
        entry: HistoryEntry,
        format: EntryTimeFormat,
    },
    SearchResult {
        entry: HistoryEntry,
        positions: Vec<usize>,
    },
}

impl ListItemType {
    fn history_entry(&self) -> Option<&HistoryEntry> {
        match self {
            ListItemType::Entry { entry, .. } => Some(entry),
            ListItemType::SearchResult { entry, .. } => Some(entry),
            _ => None,
        }
    }
}

pub enum ThreadHistoryEvent {
    Open(HistoryEntry),
}

impl EventEmitter<ThreadHistoryEvent> for AcpThreadHistory {}

impl AcpThreadHistory {
    pub(crate) fn new(
        history_store: Entity<agent2::HistoryStore>,
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
                    if this.search_query != query {
                        this.search_query = query.into();
                        this.update_visible_items(false, cx);
                    }
                }
            });

        let history_store_subscription = cx.observe(&history_store, |this, _, cx| {
            this.update_visible_items(true, cx);
        });

        let scroll_handle = UniformListScrollHandle::default();
        let scrollbar_state = ScrollbarState::new(scroll_handle.clone());

        let mut this = Self {
            history_store,
            scroll_handle,
            selected_index: 0,
            hovered_index: None,
            visible_items: Default::default(),
            search_editor,
            scrollbar_visibility: true,
            scrollbar_state,
            local_timezone: UtcOffset::from_whole_seconds(
                chrono::Local::now().offset().local_minus_utc(),
            )
            .unwrap(),
            search_query: SharedString::default(),
            _subscriptions: vec![search_editor_subscription, history_store_subscription],
            _update_task: Task::ready(()),
        };
        this.update_visible_items(false, cx);
        this
    }

    fn update_visible_items(&mut self, preserve_selected_item: bool, cx: &mut Context<Self>) {
        let entries = self
            .history_store
            .update(cx, |store, _| store.entries().collect());
        let new_list_items = if self.search_query.is_empty() {
            self.add_list_separators(entries, cx)
        } else {
            self.filter_search_results(entries, cx)
        };
        let selected_history_entry = if preserve_selected_item {
            self.selected_history_entry().cloned()
        } else {
            None
        };

        self._update_task = cx.spawn(async move |this, cx| {
            let new_visible_items = new_list_items.await;
            this.update(cx, |this, cx| {
                let new_selected_index = if let Some(history_entry) = selected_history_entry {
                    let history_entry_id = history_entry.id();
                    new_visible_items
                        .iter()
                        .position(|visible_entry| {
                            visible_entry
                                .history_entry()
                                .is_some_and(|entry| entry.id() == history_entry_id)
                        })
                        .unwrap_or(0)
                } else {
                    0
                };

                this.visible_items = new_visible_items;
                this.set_selected_index(new_selected_index, Bias::Right, cx);
                cx.notify();
            })
            .ok();
        });
    }

    fn add_list_separators(&self, entries: Vec<HistoryEntry>, cx: &App) -> Task<Vec<ListItemType>> {
        cx.background_spawn(async move {
            let mut items = Vec::with_capacity(entries.len() + 1);
            let mut bucket = None;
            let today = Local::now().naive_local().date();

            for entry in entries.into_iter() {
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

                items.push(ListItemType::Entry {
                    entry,
                    format: entry_bucket.into(),
                });
            }
            items
        })
    }

    fn filter_search_results(
        &self,
        entries: Vec<HistoryEntry>,
        cx: &App,
    ) -> Task<Vec<ListItemType>> {
        let query = self.search_query.clone();
        cx.background_spawn({
            let executor = cx.background_executor().clone();
            async move {
                let mut candidates = Vec::with_capacity(entries.len());

                for (idx, entry) in entries.iter().enumerate() {
                    candidates.push(StringMatchCandidate::new(idx, entry.title()));
                }

                const MAX_MATCHES: usize = 100;

                let matches = fuzzy::match_strings(
                    &candidates,
                    &query,
                    false,
                    true,
                    MAX_MATCHES,
                    &Default::default(),
                    executor,
                )
                .await;

                matches
                    .into_iter()
                    .map(|search_match| ListItemType::SearchResult {
                        entry: entries[search_match.candidate_id].clone(),
                        positions: search_match.positions,
                    })
                    .collect()
            }
        })
    }

    fn search_produced_no_matches(&self) -> bool {
        self.visible_items.is_empty() && !self.search_query.is_empty()
    }

    fn selected_history_entry(&self) -> Option<&HistoryEntry> {
        self.get_history_entry(self.selected_index)
    }

    fn get_history_entry(&self, visible_items_ix: usize) -> Option<&HistoryEntry> {
        self.visible_items.get(visible_items_ix)?.history_entry()
    }

    fn set_selected_index(&mut self, mut index: usize, bias: Bias, cx: &mut Context<Self>) {
        if self.visible_items.len() == 0 {
            self.selected_index = 0;
            return;
        }
        while matches!(
            self.visible_items.get(index),
            None | Some(ListItemType::BucketSeparator(..))
        ) {
            index = match bias {
                Bias::Left => {
                    if index == 0 {
                        self.visible_items.len() - 1
                    } else {
                        index - 1
                    }
                }
                Bias::Right => {
                    if index >= self.visible_items.len() - 1 {
                        0
                    } else {
                        index + 1
                    }
                }
            };
        }
        self.selected_index = index;
        self.scroll_handle
            .scroll_to_item(index, ScrollStrategy::Top);
        cx.notify()
    }

    pub fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.selected_index == 0 {
            self.set_selected_index(self.visible_items.len() - 1, Bias::Left, cx);
        } else {
            self.set_selected_index(self.selected_index - 1, Bias::Left, cx);
        }
    }

    pub fn select_next(
        &mut self,
        _: &menu::SelectNext,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.selected_index == self.visible_items.len() - 1 {
            self.set_selected_index(0, Bias::Right, cx);
        } else {
            self.set_selected_index(self.selected_index + 1, Bias::Right, cx);
        }
    }

    fn select_first(
        &mut self,
        _: &menu::SelectFirst,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_selected_index(0, Bias::Right, cx);
    }

    fn select_last(&mut self, _: &menu::SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        self.set_selected_index(self.visible_items.len() - 1, Bias::Left, cx);
    }

    fn confirm(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        self.confirm_entry(self.selected_index, cx);
    }

    fn confirm_entry(&mut self, ix: usize, cx: &mut Context<Self>) {
        let Some(entry) = self.get_history_entry(ix) else {
            return;
        };
        cx.emit(ThreadHistoryEvent::Open(entry.clone()));
    }

    fn remove_selected_thread(
        &mut self,
        _: &RemoveSelectedThread,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.remove_thread(self.selected_index, cx)
    }

    fn remove_thread(&mut self, visible_item_ix: usize, cx: &mut Context<Self>) {
        let Some(entry) = self.get_history_entry(visible_item_ix) else {
            return;
        };

        let task = match entry {
            HistoryEntry::AcpThread(thread) => self
                .history_store
                .update(cx, |this, cx| this.delete_thread(thread.id.clone(), cx)),
            HistoryEntry::TextThread(context) => self.history_store.update(cx, |this, cx| {
                this.delete_text_thread(context.path.clone(), cx)
            }),
        };
        task.detach_and_log_err(cx);
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

    fn render_list_items(
        &mut self,
        range: Range<usize>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        self.visible_items
            .get(range.clone())
            .into_iter()
            .flatten()
            .enumerate()
            .map(|(ix, item)| self.render_list_item(item, range.start + ix, cx))
            .collect()
    }

    fn render_list_item(&self, item: &ListItemType, ix: usize, cx: &Context<Self>) -> AnyElement {
        match item {
            ListItemType::Entry { entry, format } => self
                .render_history_entry(entry, *format, ix, Vec::default(), cx)
                .into_any(),
            ListItemType::SearchResult { entry, positions } => self.render_history_entry(
                entry,
                EntryTimeFormat::DateAndTime,
                ix,
                positions.clone(),
                cx,
            ),
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

    fn render_history_entry(
        &self,
        entry: &HistoryEntry,
        format: EntryTimeFormat,
        ix: usize,
        highlight_positions: Vec<usize>,
        cx: &Context<Self>,
    ) -> AnyElement {
        let selected = ix == self.selected_index;
        let hovered = Some(ix) == self.hovered_index;
        let timestamp = entry.updated_at().timestamp();
        let thread_timestamp = format.format_timestamp(timestamp, self.local_timezone);

        h_flex()
            .w_full()
            .pb_1()
            .child(
                ListItem::new(ix)
                    .rounded()
                    .toggle_state(selected)
                    .spacing(ListItemSpacing::Sparse)
                    .start_slot(
                        h_flex()
                            .w_full()
                            .gap_2()
                            .justify_between()
                            .child(
                                HighlightedLabel::new(entry.title(), highlight_positions)
                                    .size(LabelSize::Small)
                                    .truncate(),
                            )
                            .child(
                                Label::new(thread_timestamp)
                                    .color(Color::Muted)
                                    .size(LabelSize::XSmall),
                            ),
                    )
                    .on_hover(cx.listener(move |this, is_hovered, _window, cx| {
                        if *is_hovered {
                            this.hovered_index = Some(ix);
                        } else if this.hovered_index == Some(ix) {
                            this.hovered_index = None;
                        }

                        cx.notify();
                    }))
                    .end_slot::<IconButton>(if hovered {
                        Some(
                            IconButton::new("delete", IconName::Trash)
                                .shape(IconButtonShape::Square)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Muted)
                                .tooltip(move |window, cx| {
                                    Tooltip::for_action("Delete", &RemoveSelectedThread, window, cx)
                                })
                                .on_click(
                                    cx.listener(move |this, _, _, cx| this.remove_thread(ix, cx)),
                                ),
                        )
                    } else {
                        None
                    })
                    .on_click(cx.listener(move |this, _, _, cx| this.confirm_entry(ix, cx))),
            )
            .into_any_element()
    }
}

impl Focusable for AcpThreadHistory {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.search_editor.focus_handle(cx)
    }
}

impl Render for AcpThreadHistory {
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
            .when(!self.history_store.read(cx).is_empty(cx), |parent| {
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

                if self.history_store.read(cx).is_empty(cx) {
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
                                self.visible_items.len(),
                                cx.processor(|this, range: Range<usize>, window, cx| {
                                    this.render_list_items(range, window, cx)
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
pub struct AcpHistoryEntryElement {
    entry: HistoryEntry,
    thread_view: WeakEntity<AcpThreadView>,
    selected: bool,
    hovered: bool,
    on_hover: Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>,
}

impl AcpHistoryEntryElement {
    pub fn new(entry: HistoryEntry, thread_view: WeakEntity<AcpThreadView>) -> Self {
        Self {
            entry,
            thread_view,
            selected: false,
            hovered: false,
            on_hover: Box::new(|_, _, _| {}),
        }
    }

    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }

    pub fn on_hover(mut self, on_hover: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_hover = Box::new(on_hover);
        self
    }
}

impl RenderOnce for AcpHistoryEntryElement {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let id = self.entry.id();
        let title = self.entry.title();
        let timestamp = self.entry.updated_at();

        let formatted_time = {
            let now = chrono::Utc::now();
            let duration = now.signed_duration_since(timestamp);

            if duration.num_days() > 0 {
                format!("{}d", duration.num_days())
            } else if duration.num_hours() > 0 {
                format!("{}h ago", duration.num_hours())
            } else if duration.num_minutes() > 0 {
                format!("{}m ago", duration.num_minutes())
            } else {
                "Just now".to_string()
            }
        };

        ListItem::new(id)
            .rounded()
            .toggle_state(self.selected)
            .spacing(ListItemSpacing::Sparse)
            .start_slot(
                h_flex()
                    .w_full()
                    .gap_2()
                    .justify_between()
                    .child(Label::new(title).size(LabelSize::Small).truncate())
                    .child(
                        Label::new(formatted_time)
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
                            let thread_view = self.thread_view.clone();
                            let entry = self.entry.clone();

                            move |_event, _window, cx| {
                                if let Some(thread_view) = thread_view.upgrade() {
                                    thread_view.update(cx, |thread_view, cx| {
                                        thread_view.delete_history_entry(entry.clone(), cx);
                                    });
                                }
                            }
                        }),
                )
            } else {
                None
            })
            .on_click({
                let thread_view = self.thread_view.clone();
                let entry = self.entry;

                move |_event, window, cx| {
                    if let Some(workspace) = thread_view
                        .upgrade()
                        .and_then(|view| view.read(cx).workspace().upgrade())
                    {
                        match &entry {
                            HistoryEntry::AcpThread(thread_metadata) => {
                                if let Some(panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
                                    panel.update(cx, |panel, cx| {
                                        panel.load_agent_thread(
                                            thread_metadata.clone(),
                                            window,
                                            cx,
                                        );
                                    });
                                }
                            }
                            HistoryEntry::TextThread(context) => {
                                if let Some(panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
                                    panel.update(cx, |panel, cx| {
                                        panel
                                            .open_saved_prompt_editor(
                                                context.path.clone(),
                                                window,
                                                cx,
                                            )
                                            .detach_and_log_err(cx);
                                    });
                                }
                            }
                        }
                    }
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
    fn format_timestamp(&self, timestamp: i64, timezone: UtcOffset) -> String {
        let timestamp = OffsetDateTime::from_unix_timestamp(timestamp).unwrap();

        match self {
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
