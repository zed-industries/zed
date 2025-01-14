use gpui::{
    uniform_list, AppContext, FocusHandle, FocusableView, Model, ScrollStrategy,
    UniformListScrollHandle, WeakView,
};
use time::{OffsetDateTime, UtcOffset};
use ui::{prelude::*, IconButtonShape, ListItem, ListItemSpacing, Tooltip};

use crate::thread::Thread;
use crate::thread_store::ThreadStore;
use crate::{AssistantPanel, RemoveSelectedThread};

pub struct ThreadHistory {
    focus_handle: FocusHandle,
    assistant_panel: WeakView<AssistantPanel>,
    thread_store: Model<ThreadStore>,
    scroll_handle: UniformListScrollHandle,
    selected_index: usize,
}

impl ThreadHistory {
    pub(crate) fn new(
        assistant_panel: WeakView<AssistantPanel>,
        thread_store: Model<ThreadStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            assistant_panel,
            thread_store,
            scroll_handle: UniformListScrollHandle::default(),
            selected_index: 0,
        }
    }

    pub fn select_prev(&mut self, _: &menu::SelectPrev, cx: &mut ViewContext<Self>) {
        let count = self.thread_store.read(cx).non_empty_len(cx);

        if count > 0 {
            if self.selected_index == 0 {
                self.set_selected_index(count - 1, cx);
            } else {
                self.set_selected_index(self.selected_index - 1, cx);
            }
        }
    }

    pub fn select_next(&mut self, _: &menu::SelectNext, cx: &mut ViewContext<Self>) {
        let count = self.thread_store.read(cx).non_empty_len(cx);

        if count > 0 {
            if self.selected_index == count - 1 {
                self.set_selected_index(0, cx);
            } else {
                self.set_selected_index(self.selected_index + 1, cx);
            }
        }
    }

    fn select_first(&mut self, _: &menu::SelectFirst, cx: &mut ViewContext<Self>) {
        let count = self.thread_store.read(cx).non_empty_len(cx);
        if count > 0 {
            self.set_selected_index(0, cx);
        }
    }

    fn select_last(&mut self, _: &menu::SelectLast, cx: &mut ViewContext<Self>) {
        let count = self.thread_store.read(cx).non_empty_len(cx);
        if count > 0 {
            self.set_selected_index(count - 1, cx);
        }
    }

    fn set_selected_index(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        self.selected_index = index;
        self.scroll_handle
            .scroll_to_item(index, ScrollStrategy::Top);
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        let threads = self.thread_store.update(cx, |this, cx| this.threads(cx));

        if let Some(thread) = threads.get(self.selected_index) {
            self.assistant_panel
                .update(cx, move |this, cx| {
                    let thread_id = thread.read(cx).id().clone();
                    this.open_thread(&thread_id, cx)
                })
                .ok();

            cx.notify();
        }
    }

    fn remove_selected_thread(&mut self, _: &RemoveSelectedThread, cx: &mut ViewContext<Self>) {
        let threads = self.thread_store.update(cx, |this, cx| this.threads(cx));

        if let Some(thread) = threads.get(self.selected_index) {
            self.assistant_panel
                .update(cx, |this, cx| {
                    let thread_id = thread.read(cx).id().clone();
                    this.delete_thread(&thread_id, cx);
                })
                .ok();

            cx.notify();
        }
    }
}

impl FocusableView for ThreadHistory {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ThreadHistory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let threads = self.thread_store.update(cx, |this, cx| this.threads(cx));
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
                if threads.is_empty() {
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
                            cx.view().clone(),
                            "thread-history",
                            threads.len(),
                            move |history, range, _cx| {
                                threads[range]
                                    .iter()
                                    .enumerate()
                                    .map(|(index, thread)| {
                                        h_flex().w_full().pb_1().child(PastThread::new(
                                            thread.clone(),
                                            history.assistant_panel.clone(),
                                            selected_index == index,
                                        ))
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
    thread: Model<Thread>,
    assistant_panel: WeakView<AssistantPanel>,
    selected: bool,
}

impl PastThread {
    pub fn new(
        thread: Model<Thread>,
        assistant_panel: WeakView<AssistantPanel>,
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
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let (id, summary) = {
            let thread = self.thread.read(cx);
            (thread.id().clone(), thread.summary_or_default())
        };

        let thread_timestamp = time_format::format_localized_timestamp(
            OffsetDateTime::from_unix_timestamp(self.thread.read(cx).updated_at().timestamp())
                .unwrap(),
            OffsetDateTime::now_utc(),
            self.assistant_panel
                .update(cx, |this, _cx| this.local_timezone())
                .unwrap_or(UtcOffset::UTC),
            time_format::TimestampFormat::EnhancedAbsolute,
        );

        ListItem::new(("past-thread", self.thread.entity_id()))
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
                    .gap_2()
                    .child(
                        Label::new(thread_timestamp)
                            .color(Color::Disabled)
                            .size(LabelSize::Small),
                    )
                    .child(
                        IconButton::new("delete", IconName::TrashAlt)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .tooltip(|cx| Tooltip::text("Delete Thread", cx))
                            .on_click({
                                let assistant_panel = self.assistant_panel.clone();
                                let id = id.clone();
                                move |_event, cx| {
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
                let id = id.clone();
                move |_event, cx| {
                    assistant_panel
                        .update(cx, |this, cx| {
                            this.open_thread(&id, cx);
                        })
                        .ok();
                }
            })
    }
}
