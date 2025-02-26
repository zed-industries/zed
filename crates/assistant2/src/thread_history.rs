use assistant_context_editor::SavedContextMetadata;
use gpui::{
    uniform_list, App, Entity, FocusHandle, Focusable, ScrollStrategy, UniformListScrollHandle,
    WeakEntity,
};
use time::{OffsetDateTime, UtcOffset};
use ui::{prelude::*, IconButtonShape, ListItem, ListItemSpacing, Tooltip};

use crate::history_store::{HistoryEntry, HistoryStore};
use crate::thread_store::SavedThreadMetadata;
use crate::{AssistantPanel, RemoveSelectedThread};

pub struct ThreadHistory {
    focus_handle: FocusHandle,
    assistant_panel: WeakEntity<AssistantPanel>,
    history_store: Entity<HistoryStore>,
    scroll_handle: UniformListScrollHandle,
    selected_index: usize,
}

impl ThreadHistory {
    pub(crate) fn new(
        assistant_panel: WeakEntity<AssistantPanel>,
        history_store: Entity<HistoryStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            assistant_panel,
            history_store,
            scroll_handle: UniformListScrollHandle::default(),
            selected_index: 0,
        }
    }

    pub fn select_prev(
        &mut self,
        _: &menu::SelectPrev,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self
            .history_store
            .update(cx, |this, cx| this.entry_count(cx));
        if count > 0 {
            if self.selected_index == 0 {
                self.set_selected_index(count - 1, window, cx);
            } else {
                self.set_selected_index(self.selected_index - 1, window, cx);
            }
        }
    }

    pub fn select_next(
        &mut self,
        _: &menu::SelectNext,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self
            .history_store
            .update(cx, |this, cx| this.entry_count(cx));
        if count > 0 {
            if self.selected_index == count - 1 {
                self.set_selected_index(0, window, cx);
            } else {
                self.set_selected_index(self.selected_index + 1, window, cx);
            }
        }
    }

    fn select_first(&mut self, _: &menu::SelectFirst, window: &mut Window, cx: &mut Context<Self>) {
        let count = self
            .history_store
            .update(cx, |this, cx| this.entry_count(cx));
        if count > 0 {
            self.set_selected_index(0, window, cx);
        }
    }

    fn select_last(&mut self, _: &menu::SelectLast, window: &mut Window, cx: &mut Context<Self>) {
        let count = self
            .history_store
            .update(cx, |this, cx| this.entry_count(cx));
        if count > 0 {
            self.set_selected_index(count - 1, window, cx);
        }
    }

    fn set_selected_index(&mut self, index: usize, _window: &mut Window, cx: &mut Context<Self>) {
        self.selected_index = index;
        self.scroll_handle
            .scroll_to_item(index, ScrollStrategy::Top);
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let entries = self.history_store.update(cx, |this, cx| this.entries(cx));

        if let Some(entry) = entries.get(self.selected_index) {
            match entry {
                HistoryEntry::Thread(thread) => {
                    self.assistant_panel
                        .update(cx, move |this, cx| this.open_thread(&thread.id, window, cx))
                        .ok();
                }
                HistoryEntry::Context(context) => {
                    self.assistant_panel
                        .update(cx, move |this, cx| {
                            this.open_saved_prompt_editor(context.path.clone(), window, cx)
                        })
                        .ok();
                }
            }

            cx.notify();
        }
    }

    fn remove_selected_thread(
        &mut self,
        _: &RemoveSelectedThread,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let entries = self.history_store.update(cx, |this, cx| this.entries(cx));

        if let Some(entry) = entries.get(self.selected_index) {
            match entry {
                HistoryEntry::Thread(thread) => {
                    self.assistant_panel
                        .update(cx, |this, cx| {
                            this.delete_thread(&thread.id, cx);
                        })
                        .ok();
                }
                HistoryEntry::Context(_context) => {}
            }

            cx.notify();
        }
    }
}

impl Focusable for ThreadHistory {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ThreadHistory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let history_entries = self.history_store.update(cx, |this, cx| this.entries(cx));
        let selected_index = self.selected_index;

        v_flex()
            .id("thread-history-container")
            .key_context("ThreadHistory")
            .track_focus(&self.focus_handle)
            .overflow_y_scroll()
            .size_full()
            .p_1()
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::remove_selected_thread))
            .map(|history| {
                if history_entries.is_empty() {
                    history
                        .justify_center()
                        .child(
                            h_flex().w_full().justify_center().child(
                                Label::new("You don't have any past threads yet.")
                                    .size(LabelSize::Small),
                            ),
                        )
                } else {
                    history.child(
                        uniform_list(
                            cx.entity().clone(),
                            "thread-history",
                            history_entries.len(),
                            move |history, range, _window, _cx| {
                                history_entries[range]
                                    .iter()
                                    .enumerate()
                                    .map(|(index, entry)| {
                                        h_flex().w_full().pb_1().child(match entry {
                                            HistoryEntry::Thread(thread) => PastThread::new(
                                                thread.clone(),
                                                history.assistant_panel.clone(),
                                                selected_index == index,
                                            )
                                            .into_any_element(),
                                            HistoryEntry::Context(context) => PastContext::new(
                                                context.clone(),
                                                history.assistant_panel.clone(),
                                                selected_index == index,
                                            )
                                            .into_any_element(),
                                        })
                                    })
                                    .collect()
                            },
                        )
                        .track_scroll(self.scroll_handle.clone())
                        .flex_grow(),
                    )
                }
            })
    }
}

#[derive(IntoElement)]
pub struct PastThread {
    thread: SavedThreadMetadata,
    assistant_panel: WeakEntity<AssistantPanel>,
    selected: bool,
}

impl PastThread {
    pub fn new(
        thread: SavedThreadMetadata,
        assistant_panel: WeakEntity<AssistantPanel>,
        selected: bool,
    ) -> Self {
        Self {
            thread,
            assistant_panel,
            selected,
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
            .outlined()
            .toggle_state(self.selected)
            .start_slot(
                Icon::new(IconName::MessageCircle)
                    .size(IconSize::Small)
                    .color(Color::Muted),
            )
            .spacing(ListItemSpacing::Sparse)
            .child(Label::new(summary).size(LabelSize::Small).text_ellipsis())
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
                            .tooltip(Tooltip::text("Delete Thread"))
                            .on_click({
                                let assistant_panel = self.assistant_panel.clone();
                                let id = self.thread.id.clone();
                                move |_event, _window, cx| {
                                    assistant_panel
                                        .update(cx, |this, cx| {
                                            this.delete_thread(&id, cx);
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
                            this.open_thread(&id, window, cx).detach_and_log_err(cx);
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
}

impl PastContext {
    pub fn new(
        context: SavedContextMetadata,
        assistant_panel: WeakEntity<AssistantPanel>,
        selected: bool,
    ) -> Self {
        Self {
            context,
            assistant_panel,
            selected,
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
        .outlined()
        .toggle_state(self.selected)
        .start_slot(
            Icon::new(IconName::Code)
                .size(IconSize::Small)
                .color(Color::Muted),
        )
        .spacing(ListItemSpacing::Sparse)
        .child(Label::new(summary).size(LabelSize::Small).text_ellipsis())
        .end_slot(
            h_flex().gap_1p5().child(
                Label::new(context_timestamp)
                    .color(Color::Muted)
                    .size(LabelSize::XSmall),
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
