use crate::{Agent, agent_connection_store::AgentConnectionStore, thread_history::ThreadHistory};
use acp_thread::AgentSessionInfo;
use agent_settings::AgentSettings;
use chrono::{Datelike as _, Local, NaiveDate, TimeDelta, Utc};
use editor::Editor;
use gpui::{
    AnyElement, App, Context, Entity, EventEmitter, FocusHandle, Focusable, ListState, Render,
    SharedString, Window, list, prelude::*, px,
};
use settings::Settings;
use theme::ActiveTheme;
use ui::{HighlightedLabel, ListItem, Tab, Tooltip, WithScrollbar, prelude::*};

#[derive(Clone)]
enum ArchiveListItem {
    BucketSeparator(TimeBucket),
    Entry {
        session: AgentSessionInfo,
        highlight_positions: Vec<usize>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TimeBucket {
    Today,
    Yesterday,
    ThisWeek,
    PastWeek,
    Older,
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
        TimeBucket::Older
    }

    fn label(&self) -> &'static str {
        match self {
            TimeBucket::Today => "Today",
            TimeBucket::Yesterday => "Yesterday",
            TimeBucket::ThisWeek => "This Week",
            TimeBucket::PastWeek => "Past Week",
            TimeBucket::Older => "Older",
        }
    }
}

fn fuzzy_match_positions(query: &str, text: &str) -> Option<Vec<usize>> {
    let query = query.to_lowercase();
    let text_lower = text.to_lowercase();
    let mut positions = Vec::new();
    let mut query_chars = query.chars().peekable();
    for (i, c) in text_lower.chars().enumerate() {
        if query_chars.peek() == Some(&c) {
            positions.push(i);
            query_chars.next();
        }
    }
    if query_chars.peek().is_none() {
        Some(positions)
    } else {
        None
    }
}

pub enum ThreadsArchiveViewEvent {
    Close,
    OpenThread(AgentSessionInfo),
}

impl EventEmitter<ThreadsArchiveViewEvent> for ThreadsArchiveView {}

pub struct ThreadsArchiveView {
    agent_connection_store: Entity<AgentConnectionStore>,
    selected_agent: Agent,
    focus_handle: FocusHandle,
    list_state: ListState,
    items: Vec<ArchiveListItem>,
    selection: Option<usize>,
    filter_editor: Entity<Editor>,
    _subscriptions: Vec<gpui::Subscription>,
}

impl ThreadsArchiveView {
    pub fn new(
        agent_connection_store: Entity<AgentConnectionStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let filter_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search threads archive…", window, cx);
            editor
        });

        let filter_editor_subscription =
            cx.subscribe(&filter_editor, |this: &mut Self, _, event, cx| {
                if let editor::EditorEvent::BufferEdited = event {
                    this.update_items(cx);
                }
            });

        // let history_subscription = cx.observe(&history, |this, _, cx| {
        //     this.update_items(cx);
        // });

        // history.update(cx, |history, cx| {
        //     history.refresh_full_history(cx);
        // });

        let mut this = Self {
            agent_connection_store,
            selected_agent: Agent::NativeAgent,
            focus_handle,
            list_state: ListState::new(0, gpui::ListAlignment::Top, px(1000.)),
            items: Vec::new(),
            selection: None,
            filter_editor,
            _subscriptions: vec![filter_editor_subscription],
        };
        this.update_items(cx);
        this
    }

    fn history(&self, cx: &mut Context<Self>) -> Entity<ThreadHistory> {
        self.agent_connection_store
            .read(cx)
            .entry(&self.selected_agent)
            .unwrap()
            .read(cx)
            .history()
            .unwrap()
            .clone()
    }

    fn update_items(&mut self, cx: &mut Context<Self>) {
        let sessions = self.history(cx).read(cx).sessions().to_vec();
        let query = self.filter_editor.read(cx).text(cx).to_lowercase();
        let today = Local::now().naive_local().date();

        let mut items = Vec::with_capacity(sessions.len() + 5);
        let mut current_bucket: Option<TimeBucket> = None;

        for session in sessions {
            let highlight_positions = if !query.is_empty() {
                let title = session.title.as_ref().map(|t| t.as_ref()).unwrap_or("");
                match fuzzy_match_positions(&query, title) {
                    Some(positions) => positions,
                    None => continue,
                }
            } else {
                Vec::new()
            };

            let entry_bucket = session
                .updated_at
                .map(|timestamp| {
                    let entry_date = timestamp.with_timezone(&Local).naive_local().date();
                    TimeBucket::from_dates(today, entry_date)
                })
                .unwrap_or(TimeBucket::Older);

            if Some(entry_bucket) != current_bucket {
                current_bucket = Some(entry_bucket);
                items.push(ArchiveListItem::BucketSeparator(entry_bucket));
            }

            items.push(ArchiveListItem::Entry {
                session,
                highlight_positions,
            });
        }

        self.items = items;
        self.list_state = ListState::new(self.items.len(), gpui::ListAlignment::Top, px(1000.));
        cx.notify();
    }

    fn reset_filter_editor_text(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.filter_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
    }

    fn go_back(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.reset_filter_editor_text(window, cx);
        cx.emit(ThreadsArchiveViewEvent::Close);
    }

    fn open_thread(
        &mut self,
        session_info: AgentSessionInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selection = None;
        self.reset_filter_editor_text(window, cx);
        cx.emit(ThreadsArchiveViewEvent::OpenThread(session_info));
    }

    fn render_header(&self, docked_right: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let has_query = !self.filter_editor.read(cx).text(cx).is_empty();

        h_flex()
            .h(Tab::container_height(cx))
            .px_1()
            .gap_1p5()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                IconButton::new("back", IconName::ArrowLeft)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text("Back to Sidebar"))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.go_back(window, cx);
                    })),
            )
            .child(self.filter_editor.clone())
            .when(has_query, |this| {
                this.when(!docked_right, |this| this.pr_1p5()).child(
                    IconButton::new("clear_archive_filter", IconName::Close)
                        .icon_size(IconSize::Small)
                        .tooltip(Tooltip::text("Clear Search"))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.reset_filter_editor_text(window, cx);
                            this.update_items(cx);
                        })),
                )
            })
    }

    fn render_list_entry(
        &mut self,
        ix: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(item) = self.items.get(ix) else {
            return div().into_any_element();
        };

        match item {
            ArchiveListItem::BucketSeparator(bucket) => div()
                .w_full()
                .px_2()
                .pt_3()
                .pb_1()
                .child(
                    Label::new(bucket.label())
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element(),
            ArchiveListItem::Entry {
                session,
                highlight_positions,
            } => {
                let is_selected = self.selection == Some(ix);
                let title: SharedString =
                    session.title.clone().unwrap_or_else(|| "Untitled".into());
                let session_info = session.clone();
                let highlight_positions = highlight_positions.clone();

                let timestamp = session.created_at.or(session.updated_at).map(|entry_time| {
                    let now = Utc::now();
                    let duration = now.signed_duration_since(entry_time);

                    let minutes = duration.num_minutes();
                    let hours = duration.num_hours();
                    let days = duration.num_days();
                    let weeks = days / 7;
                    let months = days / 30;

                    if minutes < 60 {
                        format!("{}m", minutes.max(1))
                    } else if hours < 24 {
                        format!("{}h", hours)
                    } else if weeks < 4 {
                        format!("{}w", weeks.max(1))
                    } else {
                        format!("{}mo", months.max(1))
                    }
                });

                let id = SharedString::from(format!("archive-entry-{}", ix));

                let title_label = if highlight_positions.is_empty() {
                    Label::new(title)
                        .size(LabelSize::Small)
                        .truncate()
                        .into_any_element()
                } else {
                    HighlightedLabel::new(title, highlight_positions)
                        .size(LabelSize::Small)
                        .truncate()
                        .into_any_element()
                };

                ListItem::new(id)
                    .toggle_state(is_selected)
                    .child(
                        h_flex()
                            .min_w_0()
                            .w_full()
                            .py_1()
                            .px_0p5()
                            .gap_1()
                            .justify_between()
                            .child(title_label)
                            .when_some(timestamp, |this, ts| {
                                this.child(
                                    Label::new(ts).size(LabelSize::Small).color(Color::Muted),
                                )
                            }),
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.open_thread(session_info.clone(), window, cx);
                    }))
                    .into_any_element()
            }
        }
    }
}

impl Focusable for ThreadsArchiveView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ThreadsArchiveView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let docked_right = AgentSettings::get_global(cx).dock == settings::DockPosition::Right;

        let has_session_list = self.history(cx).read(cx).has_session_list();
        let is_empty = self.items.is_empty();
        let has_query = !self.filter_editor.read(cx).text(cx).is_empty();

        let empty_state_container = |label: SharedString| {
            v_flex()
                .flex_1()
                .justify_center()
                .items_center()
                .child(Label::new(label).size(LabelSize::Small).color(Color::Muted))
        };

        v_flex()
            .key_context("ThreadsArchiveView")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().surface_background)
            .child(self.render_header(docked_right, cx))
            .child(if !has_session_list {
                empty_state_container("Start a thread to see your archive.".into())
                    .into_any_element()
            } else if is_empty && has_query {
                empty_state_container("No threads match your search.".into()).into_any_element()
            } else if is_empty {
                empty_state_container("No archived threads yet.".into()).into_any_element()
            } else {
                v_flex()
                    .flex_1()
                    .overflow_hidden()
                    .child(
                        list(
                            self.list_state.clone(),
                            cx.processor(Self::render_list_entry),
                        )
                        .flex_1()
                        .size_full(),
                    )
                    .vertical_scrollbar_for(&self.list_state, window, cx)
                    .into_any_element()
            })
    }
}
