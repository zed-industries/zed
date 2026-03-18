use std::sync::Arc;

use crate::{
    Agent, RemoveSelectedThread, agent_connection_store::AgentConnectionStore,
    thread_history::ThreadHistory, thread_metadata_store::ThreadMetadataStore,
};
use acp_thread::AgentSessionInfo;
use agent::ThreadStore;
use agent_client_protocol as acp;
use chrono::{DateTime, Datelike as _, Local, NaiveDate, TimeDelta, Utc};
use editor::Editor;
use fs::Fs;
use gpui::{
    AnyElement, App, Context, Entity, EventEmitter, FocusHandle, Focusable, ListState, Render,
    SharedString, Subscription, Task, Window, list, prelude::*, px,
};
use itertools::Itertools as _;
use menu::{Confirm, SelectFirst, SelectLast, SelectNext, SelectPrevious};
use project::{AgentId, AgentServerStore};
use theme::ActiveTheme;
use ui::{
    ButtonLike, CommonAnimationExt, ContextMenu, ContextMenuEntry, HighlightedLabel, KeyBinding,
    ListItem, PopoverMenu, PopoverMenuHandle, Tab, TintColor, Tooltip, WithScrollbar, prelude::*,
    utils::platform_title_bar_height,
};
use util::ResultExt as _;
use zed_actions::editor::{MoveDown, MoveUp};

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

fn archive_empty_state_message(
    has_history: bool,
    is_empty: bool,
    has_query: bool,
) -> Option<&'static str> {
    if !is_empty {
        None
    } else if !has_history {
        Some("This agent does not support viewing archived threads.")
    } else if has_query {
        Some("No threads match your search.")
    } else {
        Some("No archived threads yet.")
    }
}

pub enum ThreadsArchiveViewEvent {
    Close,
    Unarchive {
        agent: Agent,
        session_info: AgentSessionInfo,
    },
}

impl EventEmitter<ThreadsArchiveViewEvent> for ThreadsArchiveView {}

pub struct ThreadsArchiveView {
    agent_connection_store: Entity<AgentConnectionStore>,
    agent_server_store: Entity<AgentServerStore>,
    thread_store: Entity<ThreadStore>,
    fs: Arc<dyn Fs>,
    history: Option<Entity<ThreadHistory>>,
    _history_subscription: Subscription,
    selected_agent: Agent,
    focus_handle: FocusHandle,
    list_state: ListState,
    items: Vec<ArchiveListItem>,
    selection: Option<usize>,
    hovered_index: Option<usize>,
    filter_editor: Entity<Editor>,
    _subscriptions: Vec<gpui::Subscription>,
    selected_agent_menu: PopoverMenuHandle<ContextMenu>,
    _refresh_history_task: Task<()>,
    _update_items_task: Option<Task<()>>,
    is_loading: bool,
    has_open_project: bool,
}

impl ThreadsArchiveView {
    pub fn new(
        agent_connection_store: Entity<AgentConnectionStore>,
        agent_server_store: Entity<AgentServerStore>,
        thread_store: Entity<ThreadStore>,
        fs: Arc<dyn Fs>,
        has_open_project: bool,
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

        let mut this = Self {
            agent_connection_store,
            agent_server_store,
            thread_store,
            fs,
            history: None,
            _history_subscription: Subscription::new(|| {}),
            selected_agent: Agent::NativeAgent,
            focus_handle,
            list_state: ListState::new(0, gpui::ListAlignment::Top, px(1000.)),
            items: Vec::new(),
            selection: None,
            hovered_index: None,
            filter_editor,
            _subscriptions: vec![filter_editor_subscription],
            selected_agent_menu: PopoverMenuHandle::default(),
            _refresh_history_task: Task::ready(()),
            _update_items_task: None,
            is_loading: true,
            has_open_project,
        };
        this.set_selected_agent(Agent::NativeAgent, window, cx);
        this
    }

    fn set_selected_agent(&mut self, agent: Agent, window: &mut Window, cx: &mut Context<Self>) {
        self.selected_agent = agent.clone();
        self.is_loading = true;
        self.reset_history_subscription();
        self.history = None;
        self.items.clear();
        self.selection = None;
        self.list_state.reset(0);
        self.reset_filter_editor_text(window, cx);

        let server = agent.server(self.fs.clone(), self.thread_store.clone());
        let connection = self
            .agent_connection_store
            .update(cx, |store, cx| store.request_connection(agent, server, cx));

        let task = connection.read(cx).wait_for_connection();
        self._refresh_history_task = cx.spawn(async move |this, cx| {
            if let Some(state) = task.await.log_err() {
                this.update(cx, |this, cx| this.set_history(state.history, cx))
                    .ok();
            }
        });

        cx.notify();
    }

    fn reset_history_subscription(&mut self) {
        self._history_subscription = Subscription::new(|| {});
    }

    fn set_history(&mut self, history: Option<Entity<ThreadHistory>>, cx: &mut Context<Self>) {
        self.reset_history_subscription();

        if let Some(history) = &history {
            self._history_subscription = cx.observe(history, |this, _, cx| {
                this.update_items(cx);
            });
            history.update(cx, |history, cx| {
                history.refresh_full_history(cx);
            });
        }
        self.history = history;
        self.is_loading = false;
        self.update_items(cx);
        cx.notify();
    }

    fn update_items(&mut self, cx: &mut Context<Self>) {
        let sessions = self
            .history
            .as_ref()
            .map(|h| h.read(cx).sessions().to_vec())
            .unwrap_or_default();
        let query = self.filter_editor.read(cx).text(cx).to_lowercase();
        let today = Local::now().naive_local().date();

        self._update_items_task.take();
        let unarchived_ids_task = ThreadMetadataStore::global(cx)
            .read(cx)
            .list_sidebar_ids(cx);
        self._update_items_task = Some(cx.spawn(async move |this, cx| {
            let unarchived_session_ids = unarchived_ids_task.await.unwrap_or_default();

            let mut items = Vec::with_capacity(sessions.len() + 5);
            let mut current_bucket: Option<TimeBucket> = None;

            for session in sessions {
                // Skip sessions that are shown in the sidebar
                if unarchived_session_ids.contains(&session.session_id) {
                    continue;
                }

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

            this.update(cx, |this, cx| {
                this.list_state.reset(items.len());
                this.items = items;
                this.selection = None;
                this.hovered_index = None;
                cx.notify();
            })
            .ok();
        }));
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

    fn unarchive_thread(
        &mut self,
        session_info: AgentSessionInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selection = None;
        self.reset_filter_editor_text(window, cx);
        cx.emit(ThreadsArchiveViewEvent::Unarchive {
            agent: self.selected_agent.clone(),
            session_info,
        });
    }

    fn delete_thread(&mut self, session_id: &acp::SessionId, cx: &mut Context<Self>) {
        let Some(history) = &self.history else {
            return;
        };
        if !history.read(cx).supports_delete() {
            return;
        }
        let session_id = session_id.clone();
        history.update(cx, |history, cx| {
            history
                .delete_session(&session_id, cx)
                .detach_and_log_err(cx);
        });
    }

    fn remove_selected_thread(
        &mut self,
        _: &RemoveSelectedThread,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.selection else {
            return;
        };
        let Some(ArchiveListItem::Entry { session, .. }) = self.items.get(ix) else {
            return;
        };
        let session_id = session.session_id.clone();
        self.delete_thread(&session_id, cx);
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
    }

    fn editor_move_up(&mut self, _: &MoveUp, window: &mut Window, cx: &mut Context<Self>) {
        self.select_previous(&SelectPrevious, window, cx);
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

    fn select_previous(
        &mut self,
        _: &SelectPrevious,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let prev = match self.selection {
            Some(ix) if ix > 0 => self.find_previous_selectable(ix - 1),
            None => {
                let last = self.items.len().saturating_sub(1);
                self.find_previous_selectable(last)
            }
            _ => return,
        };
        if let Some(prev) = prev {
            self.selection = Some(prev);
            self.list_state.scroll_to_reveal_item(prev);
            cx.notify();
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
        let Some(ArchiveListItem::Entry { session, .. }) = self.items.get(ix) else {
            return;
        };

        let thread_has_project = session.work_dirs.as_ref().is_some_and(|p| !p.is_empty());
        if !thread_has_project && !self.has_open_project {
            return;
        }

        self.unarchive_thread(session.clone(), window, cx);
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
                session,
                highlight_positions,
            } => {
                let id = SharedString::from(format!("archive-entry-{}", ix));

                let is_focused = self.selection == Some(ix);
                let hovered = self.hovered_index == Some(ix);

                let project_names = session.work_dirs.as_ref().and_then(|paths| {
                    let paths_str = paths
                        .paths()
                        .iter()
                        .filter_map(|p| p.file_name())
                        .filter_map(|name| name.to_str())
                        .join(", ");
                    if paths_str.is_empty() {
                        None
                    } else {
                        Some(paths_str)
                    }
                });

                let thread_has_project = session.work_dirs.as_ref().is_some_and(|p| !p.is_empty());
                let can_unarchive = thread_has_project || self.has_open_project;

                let supports_delete = self
                    .history
                    .as_ref()
                    .map(|h| h.read(cx).supports_delete())
                    .unwrap_or(false);

                let title: SharedString =
                    session.title.clone().unwrap_or_else(|| "Untitled".into());

                let session_info = session.clone();
                let session_id_for_delete = session.session_id.clone();
                let focus_handle = self.focus_handle.clone();

                let timestamp = session
                    .created_at
                    .or(session.updated_at)
                    .map(format_history_entry_timestamp);

                let highlight_positions = highlight_positions.clone();
                let title_label = if highlight_positions.is_empty() {
                    Label::new(title).truncate().into_any_element()
                } else {
                    HighlightedLabel::new(title, highlight_positions)
                        .truncate()
                        .into_any_element()
                };

                ListItem::new(id)
                    .focused(is_focused)
                    .on_hover(cx.listener(move |this, is_hovered, _window, cx| {
                        if *is_hovered {
                            this.hovered_index = Some(ix);
                        } else if this.hovered_index == Some(ix) {
                            this.hovered_index = None;
                        }
                        cx.notify();
                    }))
                    .child(
                        v_flex()
                            .min_w_0()
                            .w_full()
                            .py_1()
                            .pl_1()
                            .child(title_label)
                            .child(
                                h_flex()
                                    .gap_1()
                                    .when_some(timestamp, |this, ts| {
                                        this.child(
                                            Label::new(ts)
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                    })
                                    .when_some(project_names, |this, project| {
                                        this.child(
                                            Label::new("•")
                                                .size(LabelSize::Small)
                                                .color(Color::Muted)
                                                .alpha(0.5),
                                        )
                                        .child(
                                            Label::new(project)
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                    }),
                            ),
                    )
                    .when(hovered || is_focused, |this| {
                        this.end_slot(
                            h_flex()
                                .pr_2p5()
                                .gap_0p5()
                                .when(can_unarchive, |this| {
                                    this.child(
                                        Button::new("unarchive-thread", "Unarchive")
                                            .style(ButtonStyle::OutlinedGhost)
                                            .label_size(LabelSize::Small)
                                            .when(is_focused, |this| {
                                                this.key_binding(
                                                    KeyBinding::for_action_in(
                                                        &menu::Confirm,
                                                        &focus_handle,
                                                        cx,
                                                    )
                                                    .map(|kb| kb.size(rems_from_px(12.))),
                                                )
                                            })
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                this.unarchive_thread(
                                                    session_info.clone(),
                                                    window,
                                                    cx,
                                                );
                                            })),
                                    )
                                })
                                .when(supports_delete, |this| {
                                    this.child(
                                        IconButton::new("delete-thread", IconName::Trash)
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
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                this.delete_thread(&session_id_for_delete, cx);
                                                cx.stop_propagation();
                                            })),
                                    )
                                }),
                        )
                    })
                    .into_any_element()
            }
        }
    }

    fn render_agent_picker(&self, cx: &mut Context<Self>) -> PopoverMenu<ContextMenu> {
        let agent_server_store = self.agent_server_store.clone();

        let (chevron_icon, icon_color) = if self.selected_agent_menu.is_deployed() {
            (IconName::ChevronUp, Color::Accent)
        } else {
            (IconName::ChevronDown, Color::Muted)
        };

        let selected_agent_icon = if let Agent::Custom { id } = &self.selected_agent {
            let store = agent_server_store.read(cx);
            let icon = store.agent_icon(&id);

            if let Some(icon) = icon {
                Icon::from_external_svg(icon)
            } else {
                Icon::new(IconName::Sparkle)
            }
            .color(Color::Muted)
            .size(IconSize::Small)
        } else {
            Icon::new(IconName::ZedAgent)
                .color(Color::Muted)
                .size(IconSize::Small)
        };

        let this = cx.weak_entity();

        PopoverMenu::new("agent_history_menu")
            .trigger(
                ButtonLike::new("selected_agent")
                    .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                    .child(
                        h_flex().gap_1().child(selected_agent_icon).child(
                            Icon::new(chevron_icon)
                                .color(icon_color)
                                .size(IconSize::XSmall),
                        ),
                    ),
            )
            .menu(move |window, cx| {
                Some(ContextMenu::build(window, cx, |menu, _window, cx| {
                    menu.item(
                        ContextMenuEntry::new("Zed Agent")
                            .icon(IconName::ZedAgent)
                            .icon_color(Color::Muted)
                            .handler({
                                let this = this.clone();
                                move |window, cx| {
                                    this.update(cx, |this, cx| {
                                        this.set_selected_agent(Agent::NativeAgent, window, cx)
                                    })
                                    .ok();
                                }
                            }),
                    )
                    .separator()
                    .map(|mut menu| {
                        let agent_server_store = agent_server_store.read(cx);
                        let registry_store = project::AgentRegistryStore::try_global(cx);
                        let registry_store_ref = registry_store.as_ref().map(|s| s.read(cx));

                        struct AgentMenuItem {
                            id: AgentId,
                            display_name: SharedString,
                        }

                        let agent_items = agent_server_store
                            .external_agents()
                            .map(|agent_id| {
                                let display_name = agent_server_store
                                    .agent_display_name(agent_id)
                                    .or_else(|| {
                                        registry_store_ref
                                            .as_ref()
                                            .and_then(|store| store.agent(agent_id))
                                            .map(|a| a.name().clone())
                                    })
                                    .unwrap_or_else(|| agent_id.0.clone());
                                AgentMenuItem {
                                    id: agent_id.clone(),
                                    display_name,
                                }
                            })
                            .sorted_unstable_by_key(|e| e.display_name.to_lowercase())
                            .collect::<Vec<_>>();

                        for item in &agent_items {
                            let mut entry = ContextMenuEntry::new(item.display_name.clone());

                            let icon_path = agent_server_store.agent_icon(&item.id).or_else(|| {
                                registry_store_ref
                                    .as_ref()
                                    .and_then(|store| store.agent(&item.id))
                                    .and_then(|a| a.icon_path().cloned())
                            });

                            if let Some(icon_path) = icon_path {
                                entry = entry.custom_icon_svg(icon_path);
                            } else {
                                entry = entry.icon(IconName::ZedAgent);
                            }

                            entry = entry.icon_color(Color::Muted).handler({
                                let this = this.clone();
                                let agent = Agent::Custom {
                                    id: item.id.clone(),
                                };
                                move |window, cx| {
                                    this.update(cx, |this, cx| {
                                        this.set_selected_agent(agent.clone(), window, cx)
                                    })
                                    .ok();
                                }
                            });

                            menu = menu.item(entry);
                        }
                        menu
                    })
                }))
            })
            .with_handle(self.selected_agent_menu.clone())
            .anchor(gpui::Corner::TopRight)
            .offset(gpui::Point {
                x: px(1.0),
                y: px(1.0),
            })
    }

    fn render_header(&self, window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_query = !self.filter_editor.read(cx).text(cx).is_empty();
        let traffic_lights = cfg!(target_os = "macos") && !window.is_fullscreen();
        let header_height = platform_title_bar_height(window);

        v_flex()
            .child(
                h_flex()
                    .h(header_height)
                    .mt_px()
                    .pb_px()
                    .when(traffic_lights, |this| {
                        this.pl(px(ui::utils::TRAFFIC_LIGHT_PADDING))
                    })
                    .pr_1p5()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_1p5()
                            .child(
                                IconButton::new("back", IconName::ArrowLeft)
                                    .icon_size(IconSize::Small)
                                    .tooltip(Tooltip::text("Back to Sidebar"))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.go_back(window, cx);
                                    })),
                            )
                            .child(Label::new("Threads Archive").size(LabelSize::Small).mb_px()),
                    )
                    .child(self.render_agent_picker(cx)),
            )
            .child(
                h_flex()
                    .h(Tab::container_height(cx))
                    .px_1p5()
                    .gap_1p5()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        h_flex().size_4().flex_none().justify_center().child(
                            Icon::new(IconName::MagnifyingGlass)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                    .child(self.filter_editor.clone())
                    .when(has_query, |this| {
                        this.child(
                            IconButton::new("clear_filter", IconName::Close)
                                .icon_size(IconSize::Small)
                                .tooltip(Tooltip::text("Clear Search"))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.reset_filter_editor_text(window, cx);
                                    this.update_items(cx);
                                })),
                        )
                    }),
            )
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

impl ThreadsArchiveView {
    fn empty_state_message(&self, is_empty: bool, has_query: bool) -> Option<&'static str> {
        archive_empty_state_message(self.history.is_some(), is_empty, has_query)
    }
}

impl Render for ThreadsArchiveView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.items.is_empty();
        let has_query = !self.filter_editor.read(cx).text(cx).is_empty();

        let content = if self.is_loading {
            v_flex()
                .flex_1()
                .justify_center()
                .items_center()
                .child(
                    Icon::new(IconName::LoadCircle)
                        .size(IconSize::Small)
                        .color(Color::Muted)
                        .with_rotate_animation(2),
                )
                .into_any_element()
        } else if let Some(message) = self.empty_state_message(is_empty, has_query) {
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

#[cfg(test)]
mod tests {
    use super::archive_empty_state_message;

    #[test]
    fn empty_state_message_returns_none_when_archive_has_items() {
        assert_eq!(archive_empty_state_message(false, false, false), None);
        assert_eq!(archive_empty_state_message(true, false, true), None);
    }

    #[test]
    fn empty_state_message_distinguishes_unsupported_history() {
        assert_eq!(
            archive_empty_state_message(false, true, false),
            Some("This agent does not support viewing archived threads.")
        );
        assert_eq!(
            archive_empty_state_message(false, true, true),
            Some("This agent does not support viewing archived threads.")
        );
    }

    #[test]
    fn empty_state_message_distinguishes_empty_history_and_search_results() {
        assert_eq!(
            archive_empty_state_message(true, true, false),
            Some("No archived threads yet.")
        );
        assert_eq!(
            archive_empty_state_message(true, true, true),
            Some("No threads match your search.")
        );
    }
}
