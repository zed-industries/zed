use acp_thread::{AgentSessionInfo, AgentSessionList, AgentSessionListRequest};
use agent_client_protocol as acp;
use chrono::{Datelike as _, Local, NaiveDate, TimeDelta, Utc};
use editor::{Editor, EditorEvent};
use fuzzy::StringMatchCandidate;
use gpui::{
    App, Entity, EventEmitter, FocusHandle, Focusable, ScrollStrategy, Task,
    UniformListScrollHandle, Window, actions, uniform_list,
};
use std::{fmt::Display, ops::Range, rc::Rc};
use text::Bias;
use time::{OffsetDateTime, UtcOffset};
use ui::{
    HighlightedLabel, IconButtonShape, ListItem, ListItemSpacing, Tab, Tooltip, WithScrollbar,
    prelude::*,
};

const DEFAULT_TITLE: &SharedString = &SharedString::new_static("New Thread");

fn thread_title(entry: &AgentSessionInfo) -> &SharedString {
    entry
        .title
        .as_ref()
        .filter(|title| !title.is_empty())
        .unwrap_or(DEFAULT_TITLE)
}

actions!(
    agents,
    [
        /// Removes all thread history.
        RemoveHistory,
        /// Removes the currently selected thread.
        RemoveSelectedThread,
    ]
);

pub struct AcpThreadHistory {
    session_list: Option<Rc<dyn AgentSessionList>>,
    sessions: Vec<AgentSessionInfo>,
    scroll_handle: UniformListScrollHandle,
    selected_index: usize,
    hovered_index: Option<usize>,
    search_editor: Entity<Editor>,
    search_query: SharedString,
    visible_items: Vec<ListItemType>,
    local_timezone: UtcOffset,
    confirming_delete_history: bool,
    _update_task: Task<()>,
    _watch_task: Option<Task<()>>,
    _subscriptions: Vec<gpui::Subscription>,
}

enum ListItemType {
    BucketSeparator(TimeBucket),
    Entry {
        entry: AgentSessionInfo,
        format: EntryTimeFormat,
    },
    SearchResult {
        entry: AgentSessionInfo,
        positions: Vec<usize>,
    },
}

impl ListItemType {
    fn history_entry(&self) -> Option<&AgentSessionInfo> {
        match self {
            ListItemType::Entry { entry, .. } => Some(entry),
            ListItemType::SearchResult { entry, .. } => Some(entry),
            _ => None,
        }
    }
}

#[allow(dead_code)]
pub enum ThreadHistoryEvent {
    Open(AgentSessionInfo),
}

impl EventEmitter<ThreadHistoryEvent> for AcpThreadHistory {}

impl AcpThreadHistory {
    pub fn new(
        session_list: Option<Rc<dyn AgentSessionList>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let search_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search threads...", window, cx);
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

        let scroll_handle = UniformListScrollHandle::default();

        let mut this = Self {
            session_list: None,
            sessions: Vec::new(),
            scroll_handle,
            selected_index: 0,
            hovered_index: None,
            visible_items: Default::default(),
            search_editor,
            local_timezone: UtcOffset::from_whole_seconds(
                chrono::Local::now().offset().local_minus_utc(),
            )
            .unwrap(),
            search_query: SharedString::default(),
            confirming_delete_history: false,
            _subscriptions: vec![search_editor_subscription],
            _update_task: Task::ready(()),
            _watch_task: None,
        };
        this.set_session_list(session_list, cx);
        this
    }

    fn update_visible_items(&mut self, preserve_selected_item: bool, cx: &mut Context<Self>) {
        let entries = self.sessions.clone();
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
                    new_visible_items
                        .iter()
                        .position(|visible_entry| {
                            visible_entry
                                .history_entry()
                                .is_some_and(|entry| entry.session_id == history_entry.session_id)
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

    pub(crate) fn set_session_list(
        &mut self,
        session_list: Option<Rc<dyn AgentSessionList>>,
        cx: &mut Context<Self>,
    ) {
        if let (Some(current), Some(next)) = (&self.session_list, &session_list)
            && Rc::ptr_eq(current, next)
        {
            return;
        }

        self.session_list = session_list;
        self.sessions.clear();
        self.visible_items.clear();
        self.selected_index = 0;
        self.refresh_sessions(false, cx);

        self._watch_task = self.session_list.as_ref().and_then(|session_list| {
            let mut rx = session_list.watch(cx)?;
            Some(cx.spawn(async move |this, cx| {
                while let Ok(()) = rx.recv().await {
                    this.update(cx, |this, cx| {
                        this.refresh_sessions(true, cx);
                    })
                    .ok();
                }
            }))
        });
    }

    fn refresh_sessions(&mut self, preserve_selected_item: bool, cx: &mut Context<Self>) {
        let Some(session_list) = self.session_list.clone() else {
            self.update_visible_items(preserve_selected_item, cx);
            return;
        };

        self._update_task = cx.spawn(async move |this, cx| {
            let mut cursor: Option<String> = None;
            let mut is_first_page = true;

            loop {
                let request = AgentSessionListRequest {
                    cursor: cursor.clone(),
                    ..Default::default()
                };
                let task = cx.update(|cx| session_list.list_sessions(request, cx));
                let response = match task.await {
                    Ok(response) => response,
                    Err(error) => {
                        log::error!("Failed to load session history: {error:#}");
                        return;
                    }
                };

                let acp_thread::AgentSessionListResponse {
                    sessions: page_sessions,
                    next_cursor,
                    ..
                } = response;

                this.update(cx, |this, cx| {
                    if is_first_page {
                        this.sessions = page_sessions;
                    } else {
                        this.sessions.extend(page_sessions);
                    }
                    this.update_visible_items(preserve_selected_item, cx);
                })
                .ok();

                is_first_page = false;
                match next_cursor {
                    Some(next_cursor) => {
                        if cursor.as_ref() == Some(&next_cursor) {
                            log::warn!(
                                "Session list pagination returned the same cursor; stopping to avoid a loop."
                            );
                            break;
                        }
                        cursor = Some(next_cursor);
                    }
                    None => break,
                }
            }
        });
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }

    pub(crate) fn session_for_id(&self, session_id: &acp::SessionId) -> Option<AgentSessionInfo> {
        self.sessions
            .iter()
            .find(|entry| &entry.session_id == session_id)
            .cloned()
    }

    #[allow(dead_code)]
    pub(crate) fn sessions(&self) -> &[AgentSessionInfo] {
        &self.sessions
    }

    fn add_list_separators(
        &self,
        entries: Vec<AgentSessionInfo>,
        cx: &App,
    ) -> Task<Vec<ListItemType>> {
        cx.background_spawn(async move {
            let mut items = Vec::with_capacity(entries.len() + 1);
            let mut bucket = None;
            let today = Local::now().naive_local().date();

            for entry in entries.into_iter() {
                let entry_bucket = entry
                    .updated_at
                    .map(|timestamp| {
                        let entry_date = timestamp.with_timezone(&Local).naive_local().date();
                        TimeBucket::from_dates(today, entry_date)
                    })
                    .unwrap_or(TimeBucket::All);

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
        entries: Vec<AgentSessionInfo>,
        cx: &App,
    ) -> Task<Vec<ListItemType>> {
        let query = self.search_query.clone();
        cx.background_spawn({
            let executor = cx.background_executor().clone();
            async move {
                let mut candidates = Vec::with_capacity(entries.len());

                for (idx, entry) in entries.iter().enumerate() {
                    candidates.push(StringMatchCandidate::new(idx, thread_title(entry)));
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

    fn selected_history_entry(&self) -> Option<&AgentSessionInfo> {
        self.get_history_entry(self.selected_index)
    }

    fn get_history_entry(&self, visible_items_ix: usize) -> Option<&AgentSessionInfo> {
        self.visible_items.get(visible_items_ix)?.history_entry()
    }

    fn set_selected_index(&mut self, mut index: usize, bias: Bias, cx: &mut Context<Self>) {
        if self.visible_items.is_empty() {
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
        let Some(session_list) = self.session_list.as_ref() else {
            return;
        };
        let task = session_list.delete_session(&entry.session_id, cx);
        task.detach_and_log_err(cx);
    }

    fn remove_history(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(session_list) = self.session_list.as_ref() {
            session_list.delete_sessions(cx).detach_and_log_err(cx);
        }
        self.confirming_delete_history = false;
        cx.notify();
    }

    fn prompt_delete_history(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.confirming_delete_history = true;
        cx.notify();
    }

    fn cancel_delete_history(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.confirming_delete_history = false;
        cx.notify();
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
        entry: &AgentSessionInfo,
        format: EntryTimeFormat,
        ix: usize,
        highlight_positions: Vec<usize>,
        cx: &Context<Self>,
    ) -> AnyElement {
        let selected = ix == self.selected_index;
        let hovered = Some(ix) == self.hovered_index;
        let display_text = match (format, entry.updated_at) {
            (EntryTimeFormat::DateAndTime, Some(entry_time)) => {
                let now = Utc::now();
                let duration = now.signed_duration_since(entry_time);
                let days = duration.num_days();

                format!("{}d", days)
            }
            (EntryTimeFormat::TimeOnly, Some(entry_time)) => {
                format.format_timestamp(entry_time.timestamp(), self.local_timezone)
            }
            (_, None) => "—".to_string(),
        };

        let title = thread_title(entry).clone();
        let full_date = entry
            .updated_at
            .map(|time| {
                EntryTimeFormat::DateAndTime.format_timestamp(time.timestamp(), self.local_timezone)
            })
            .unwrap_or_else(|| "Unknown".to_string());

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
                                HighlightedLabel::new(thread_title(entry), highlight_positions)
                                    .size(LabelSize::Small)
                                    .truncate(),
                            )
                            .child(
                                Label::new(display_text)
                                    .color(Color::Muted)
                                    .size(LabelSize::XSmall),
                            ),
                    )
                    .tooltip(move |_, cx| {
                        Tooltip::with_meta(title.clone(), None, full_date.clone(), cx)
                    })
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
                                .tooltip(move |_window, cx| {
                                    Tooltip::for_action("Delete", &RemoveSelectedThread, cx)
                                })
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.remove_thread(ix, cx);
                                    cx.stop_propagation()
                                })),
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_no_history = self.is_empty();

        v_flex()
            .key_context("ThreadHistory")
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::remove_selected_thread))
            .on_action(cx.listener(|this, _: &RemoveHistory, window, cx| {
                this.remove_history(window, cx);
            }))
            .child(
                h_flex()
                    .h(Tab::container_height(cx))
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
            .child({
                let view = v_flex()
                    .id("list-container")
                    .relative()
                    .overflow_hidden()
                    .flex_grow();

                if has_no_history {
                    view.justify_center().items_center().child(
                        Label::new("You don't have any past threads yet.")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                } else if self.search_produced_no_matches() {
                    view.justify_center()
                        .items_center()
                        .child(Label::new("No threads match your search.").size(LabelSize::Small))
                } else {
                    view.child(
                        uniform_list(
                            "thread-history",
                            self.visible_items.len(),
                            cx.processor(|this, range: Range<usize>, window, cx| {
                                this.render_list_items(range, window, cx)
                            }),
                        )
                        .p_1()
                        .pr_4()
                        .track_scroll(&self.scroll_handle)
                        .flex_grow(),
                    )
                    .vertical_scrollbar_for(&self.scroll_handle, window, cx)
                }
            })
            .when(!has_no_history, |this| {
                this.child(
                    h_flex()
                        .p_2()
                        .border_t_1()
                        .border_color(cx.theme().colors().border_variant)
                        .when(!self.confirming_delete_history, |this| {
                            this.child(
                                Button::new("delete_history", "Delete All History")
                                    .full_width()
                                    .style(ButtonStyle::Outlined)
                                    .label_size(LabelSize::Small)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.prompt_delete_history(window, cx);
                                    })),
                            )
                        })
                        .when(self.confirming_delete_history, |this| {
                            this.w_full()
                                .gap_2()
                                .flex_wrap()
                                .justify_between()
                                .child(
                                    h_flex()
                                        .flex_wrap()
                                        .gap_1()
                                        .child(
                                            Label::new("Delete all threads?")
                                                .size(LabelSize::Small),
                                        )
                                        .child(
                                            Label::new("You won't be able to recover them later.")
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        ),
                                )
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .child(
                                            Button::new("cancel_delete", "Cancel")
                                                .label_size(LabelSize::Small)
                                                .on_click(cx.listener(|this, _, window, cx| {
                                                    this.cancel_delete_history(window, cx);
                                                })),
                                        )
                                        .child(
                                            Button::new("confirm_delete", "Delete")
                                                .style(ButtonStyle::Tinted(ui::TintColor::Error))
                                                .color(Color::Error)
                                                .label_size(LabelSize::Small)
                                                .on_click(cx.listener(|_, _, window, cx| {
                                                    window.dispatch_action(
                                                        Box::new(RemoveHistory),
                                                        cx,
                                                    );
                                                })),
                                        ),
                                )
                        }),
                )
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
            EntryTimeFormat::TimeOnly => time_format::format_time(timestamp.to_offset(timezone)),
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
