use crate::agent_connection_store::AgentConnectionStore;

use crate::thread_metadata_store::{ThreadMetadata, ThreadMetadataStore};
use crate::{Agent, RemoveSelectedThread};

use agent::ThreadStore;
use agent_client_protocol as acp;
use agent_settings::AgentSettings;
use chrono::{DateTime, Datelike as _, Local, NaiveDate, TimeDelta, Utc};
use editor::Editor;
use fs::Fs;
use gpui::{
    AnyElement, App, Context, Entity, EventEmitter, FocusHandle, Focusable, ListState, Render,
    SharedString, Subscription, Task, WeakEntity, Window, list, prelude::*, px,
};
use itertools::Itertools as _;
use menu::{Confirm, SelectFirst, SelectLast, SelectNext, SelectPrevious};
use project::{AgentId, AgentServerStore};
use settings::Settings as _;
use theme::ActiveTheme;
use ui::ThreadItem;
use ui::{
    Divider, KeyBinding, Tooltip, WithScrollbar, prelude::*, utils::platform_title_bar_height,
};

use zed_actions::agents_sidebar::FocusSidebarFilter;
use zed_actions::editor::{MoveDown, MoveUp};

#[derive(Clone)]
enum ArchiveListItem {
    BucketSeparator(TimeBucket),
    Entry {
        thread: ThreadMetadata,
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
    Unarchive { thread: ThreadMetadata },
}

impl EventEmitter<ThreadsArchiveViewEvent> for ThreadsArchiveView {}

pub struct ThreadsArchiveView {
    _history_subscription: Subscription,
    focus_handle: FocusHandle,
    list_state: ListState,
    items: Vec<ArchiveListItem>,
    selection: Option<usize>,
    hovered_index: Option<usize>,
    preserve_selection_on_next_update: bool,
    filter_editor: Entity<Editor>,
    _subscriptions: Vec<gpui::Subscription>,
    _refresh_history_task: Task<()>,
    agent_connection_store: WeakEntity<AgentConnectionStore>,
    agent_server_store: WeakEntity<AgentServerStore>,
}

impl ThreadsArchiveView {
    pub fn new(
        agent_connection_store: WeakEntity<AgentConnectionStore>,
        agent_server_store: WeakEntity<AgentServerStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let filter_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search archive…", window, cx);
            editor
        });

        let filter_editor_subscription =
            cx.subscribe(&filter_editor, |this: &mut Self, _, event, cx| {
                if let editor::EditorEvent::BufferEdited = event {
                    this.update_items(cx);
                }
            });

        let filter_focus_handle = filter_editor.read(cx).focus_handle(cx);
        cx.on_focus_in(
            &filter_focus_handle,
            window,
            |this: &mut Self, _window, cx| {
                if this.selection.is_some() {
                    this.selection = None;
                    cx.notify();
                }
            },
        )
        .detach();

        let thread_metadata_store_subscription = cx.observe(
            &ThreadMetadataStore::global(cx),
            |this: &mut Self, _, cx| {
                this.update_items(cx);
            },
        );

        cx.on_focus_out(&focus_handle, window, |this: &mut Self, _, _window, cx| {
            this.selection = None;
            cx.notify();
        })
        .detach();

        let mut this = Self {
            _history_subscription: Subscription::new(|| {}),
            focus_handle,
            list_state: ListState::new(0, gpui::ListAlignment::Top, px(1000.)),
            items: Vec::new(),
            selection: None,
            hovered_index: None,
            preserve_selection_on_next_update: false,
            filter_editor,
            _subscriptions: vec![
                filter_editor_subscription,
                thread_metadata_store_subscription,
            ],
            _refresh_history_task: Task::ready(()),
            agent_connection_store,
            agent_server_store,
        };

        this.update_items(cx);
        this
    }

    pub fn has_selection(&self) -> bool {
        self.selection.is_some()
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub fn focus_filter_editor(&self, window: &mut Window, cx: &mut App) {
        let handle = self.filter_editor.read(cx).focus_handle(cx);
        handle.focus(window, cx);
    }

    fn update_items(&mut self, cx: &mut Context<Self>) {
        let sessions = ThreadMetadataStore::global(cx)
            .read(cx)
            .archived_entries()
            .sorted_by_cached_key(|t| t.created_at.unwrap_or(t.updated_at))
            .rev()
            .cloned()
            .collect::<Vec<_>>();

        let query = self.filter_editor.read(cx).text(cx).to_lowercase();
        let today = Local::now().naive_local().date();

        let mut items = Vec::with_capacity(sessions.len() + 5);
        let mut current_bucket: Option<TimeBucket> = None;

        for session in sessions {
            let highlight_positions = if !query.is_empty() {
                match fuzzy_match_positions(&query, &session.title) {
                    Some(positions) => positions,
                    None => continue,
                }
            } else {
                Vec::new()
            };

            let entry_bucket = {
                let entry_date = session
                    .created_at
                    .unwrap_or(session.updated_at)
                    .with_timezone(&Local)
                    .naive_local()
                    .date();
                TimeBucket::from_dates(today, entry_date)
            };

            if Some(entry_bucket) != current_bucket {
                current_bucket = Some(entry_bucket);
                items.push(ArchiveListItem::BucketSeparator(entry_bucket));
            }

            items.push(ArchiveListItem::Entry {
                thread: session,
                highlight_positions,
            });
        }

        let preserve = self.preserve_selection_on_next_update;
        self.preserve_selection_on_next_update = false;

        let saved_scroll = if preserve {
            Some(self.list_state.logical_scroll_top())
        } else {
            None
        };

        self.list_state.reset(items.len());
        self.items = items;
        self.hovered_index = None;

        if let Some(scroll_top) = saved_scroll {
            self.list_state.scroll_to(scroll_top);

            if let Some(ix) = self.selection {
                let next = self.find_next_selectable(ix).or_else(|| {
                    ix.checked_sub(1)
                        .and_then(|i| self.find_previous_selectable(i))
                });
                self.selection = next;
                if let Some(next) = next {
                    self.list_state.scroll_to_reveal_item(next);
                }
            }
        } else {
            self.selection = None;
        }

        cx.notify();
    }

    fn reset_filter_editor_text(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.filter_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
    }

    fn unarchive_thread(
        &mut self,
        thread: ThreadMetadata,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selection = None;
        self.reset_filter_editor_text(window, cx);
        cx.emit(ThreadsArchiveViewEvent::Unarchive { thread });
    }

    fn is_selectable_item(&self, ix: usize) -> bool {
        matches!(self.items.get(ix), Some(ArchiveListItem::Entry { .. }))
    }

    fn find_next_selectable(&self, start: usize) -> Option<usize> {
        (start..self.items.len()).find(|&i| self.is_selectable_item(i))
    }

    fn find_previous_selectable(&self, start: usize) -> Option<usize> {
        (0..=start).rev().find(|&i| self.is_selectable_item(i))
    }

    fn editor_move_down(&mut self, _: &MoveDown, window: &mut Window, cx: &mut Context<Self>) {
        self.select_next(&SelectNext, window, cx);
        if self.selection.is_some() {
            self.focus_handle.focus(window, cx);
        }
    }

    fn editor_move_up(&mut self, _: &MoveUp, window: &mut Window, cx: &mut Context<Self>) {
        self.select_previous(&SelectPrevious, window, cx);
        if self.selection.is_some() {
            self.focus_handle.focus(window, cx);
        }
    }

    fn select_next(&mut self, _: &SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        let next = match self.selection {
            Some(ix) => self.find_next_selectable(ix + 1),
            None => self.find_next_selectable(0),
        };
        if let Some(next) = next {
            self.selection = Some(next);
            self.list_state.scroll_to_reveal_item(next);
            cx.notify();
        }
    }

    fn select_previous(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        match self.selection {
            Some(ix) => {
                if let Some(prev) = (ix > 0)
                    .then(|| self.find_previous_selectable(ix - 1))
                    .flatten()
                {
                    self.selection = Some(prev);
                    self.list_state.scroll_to_reveal_item(prev);
                } else {
                    self.selection = None;
                    self.focus_filter_editor(window, cx);
                }
                cx.notify();
            }
            None => {
                let last = self.items.len().saturating_sub(1);
                if let Some(prev) = self.find_previous_selectable(last) {
                    self.selection = Some(prev);
                    self.list_state.scroll_to_reveal_item(prev);
                    cx.notify();
                }
            }
        }
    }

    fn select_first(&mut self, _: &SelectFirst, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(first) = self.find_next_selectable(0) {
            self.selection = Some(first);
            self.list_state.scroll_to_reveal_item(first);
            cx.notify();
        }
    }

    fn select_last(&mut self, _: &SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        let last = self.items.len().saturating_sub(1);
        if let Some(last) = self.find_previous_selectable(last) {
            self.selection = Some(last);
            self.list_state.scroll_to_reveal_item(last);
            cx.notify();
        }
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let Some(ix) = self.selection else { return };
        let Some(ArchiveListItem::Entry { thread, .. }) = self.items.get(ix) else {
            return;
        };

        if thread.folder_paths.is_empty() {
            return;
        }

        self.unarchive_thread(thread.clone(), window, cx);
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
                .px_2p5()
                .pt_3()
                .pb_1()
                .child(
                    Label::new(bucket.label())
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element(),
            ArchiveListItem::Entry {
                thread,
                highlight_positions,
            } => {
                let id = SharedString::from(format!("archive-entry-{}", ix));

                let is_focused = self.selection == Some(ix);
                let is_hovered = self.hovered_index == Some(ix);

                let focus_handle = self.focus_handle.clone();

                let timestamp =
                    format_history_entry_timestamp(thread.created_at.unwrap_or(thread.updated_at));

                let icon_from_external_svg = self
                    .agent_server_store
                    .upgrade()
                    .and_then(|store| store.read(cx).agent_icon(&thread.agent_id));

                let icon = if thread.agent_id.as_ref() == agent::ZED_AGENT_ID.as_ref() {
                    IconName::ZedAgent
                } else {
                    IconName::Sparkle
                };

                ThreadItem::new(id, thread.title.clone())
                    .icon(icon)
                    .when_some(icon_from_external_svg, |this, svg| {
                        this.custom_icon_from_external_svg(svg)
                    })
                    .timestamp(timestamp)
                    .highlight_positions(highlight_positions.clone())
                    .project_paths(thread.folder_paths.paths_owned())
                    .focused(is_focused)
                    .hovered(is_hovered)
                    .on_hover(cx.listener(move |this, is_hovered, _window, cx| {
                        if *is_hovered {
                            this.hovered_index = Some(ix);
                        } else if this.hovered_index == Some(ix) {
                            this.hovered_index = None;
                        }
                        cx.notify();
                    }))
                    .action_slot(
                        IconButton::new("delete-thread", IconName::Trash)
                            .style(ButtonStyle::Filled)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .tooltip({
                                move |_window, cx| {
                                    Tooltip::for_action_in(
                                        "Delete Thread",
                                        &RemoveSelectedThread,
                                        &focus_handle,
                                        cx,
                                    )
                                }
                            })
                            .on_click({
                                let agent = thread.agent_id.clone();
                                let session_id = thread.session_id.clone();
                                cx.listener(move |this, _, _, cx| {
                                    this.delete_thread(session_id.clone(), agent.clone(), cx);
                                    cx.stop_propagation();
                                })
                            }),
                    )
                    .tooltip(move |_, cx| Tooltip::for_action("Restore Thread", &menu::Confirm, cx))
                    .on_click({
                        let thread = thread.clone();
                        cx.listener(move |this, _, window, cx| {
                            this.unarchive_thread(thread.clone(), window, cx);
                        })
                    })
                    .into_any_element()
            }
        }
    }

    fn remove_selected_thread(
        &mut self,
        _: &RemoveSelectedThread,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.selection else { return };
        let Some(ArchiveListItem::Entry { thread, .. }) = self.items.get(ix) else {
            return;
        };

        self.preserve_selection_on_next_update = true;
        self.delete_thread(thread.session_id.clone(), thread.agent_id.clone(), cx);
    }

    fn delete_thread(
        &mut self,
        session_id: acp::SessionId,
        agent: AgentId,
        cx: &mut Context<Self>,
    ) {
        ThreadMetadataStore::global(cx)
            .update(cx, |store, cx| store.delete(session_id.clone(), cx));

        let agent = Agent::from(agent);

        let Some(agent_connection_store) = self.agent_connection_store.upgrade() else {
            return;
        };
        let fs = <dyn Fs>::global(cx);

        let task = agent_connection_store.update(cx, |store, cx| {
            store
                .request_connection(agent.clone(), agent.server(fs, ThreadStore::global(cx)), cx)
                .read(cx)
                .wait_for_connection()
        });
        cx.spawn(async move |_this, cx| {
            let state = task.await?;
            let task = cx.update(|cx| {
                if let Some(list) = state.connection.session_list(cx) {
                    list.delete_session(&session_id, cx)
                } else {
                    Task::ready(Ok(()))
                }
            });
            task.await
        })
        .detach_and_log_err(cx);
    }

    fn render_header(&self, window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_query = !self.filter_editor.read(cx).text(cx).is_empty();
        let sidebar_on_left = matches!(
            AgentSettings::get_global(cx).sidebar_side(),
            settings::SidebarSide::Left
        );
        let traffic_lights =
            cfg!(target_os = "macos") && !window.is_fullscreen() && sidebar_on_left;
        let header_height = platform_title_bar_height(window);
        let show_focus_keybinding =
            self.selection.is_some() && !self.filter_editor.focus_handle(cx).is_focused(window);

        h_flex()
            .h(header_height)
            .mt_px()
            .pb_px()
            .map(|this| {
                if traffic_lights {
                    this.pl(px(ui::utils::TRAFFIC_LIGHT_PADDING))
                } else {
                    this.pl_1p5()
                }
            })
            .pr_1p5()
            .gap_1()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .when(traffic_lights, |this| {
                this.child(Divider::vertical().color(ui::DividerColor::Border))
            })
            .child(
                h_flex()
                    .ml_1()
                    .min_w_0()
                    .w_full()
                    .gap_1()
                    .child(
                        Icon::new(IconName::MagnifyingGlass)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(self.filter_editor.clone()),
            )
            .when(show_focus_keybinding, |this| {
                this.child(KeyBinding::for_action(&FocusSidebarFilter, cx))
            })
            .when(has_query, |this| {
                this.child(
                    IconButton::new("clear-filter", IconName::Close)
                        .icon_size(IconSize::Small)
                        .tooltip(Tooltip::text("Clear Search"))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.reset_filter_editor_text(window, cx);
                            this.update_items(cx);
                        })),
                )
            })
    }
}

pub fn format_history_entry_timestamp(entry_time: DateTime<Utc>) -> String {
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
        format!("{}h", hours.max(1))
    } else if days < 7 {
        format!("{}d", days.max(1))
    } else if weeks < 4 {
        format!("{}w", weeks.max(1))
    } else {
        format!("{}mo", months.max(1))
    }
}

impl Focusable for ThreadsArchiveView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ThreadsArchiveView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.items.is_empty();
        let has_query = !self.filter_editor.read(cx).text(cx).is_empty();

        let content = if is_empty {
            let message = if has_query {
                "No threads match your search."
            } else {
                "No archived or hidden threads yet."
            };

            v_flex()
                .flex_1()
                .justify_center()
                .items_center()
                .child(
                    Label::new(message)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element()
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
        };

        v_flex()
            .key_context("ThreadsArchiveView")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::editor_move_down))
            .on_action(cx.listener(Self::editor_move_up))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::remove_selected_thread))
            .size_full()
            .child(self.render_header(window, cx))
            .child(content)
    }
}
