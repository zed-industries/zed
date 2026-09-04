use agent_client_protocol::schema::v1 as acp;
use gpui::{App, EventEmitter, FocusHandle, Focusable, IntoElement, Render, ScrollHandle, Window};
use ui::{ListItem, WithScrollbar, prelude::*};

use super::PromptHistory;

pub(super) struct PromptHistoryPopover {
    history: PromptHistory,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
}

pub(super) enum PromptHistoryPopoverEvent {
    Accepted(Vec<acp::ContentBlock>),
    Dismissed,
}

impl EventEmitter<PromptHistoryPopoverEvent> for PromptHistoryPopover {}

impl PromptHistoryPopover {
    pub(super) fn new(history: PromptHistory, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();
        scroll_handle.scroll_to_item(history.selected_index());
        Self {
            history,
            focus_handle: cx.focus_handle(),
            scroll_handle,
        }
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.history.select_previous();
        self.scroll_handle
            .scroll_to_item(self.history.selected_index());
        cx.notify();
    }

    fn select_next(&mut self, _: &menu::SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        self.history.select_next();
        self.scroll_handle
            .scroll_to_item(self.history.selected_index());
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        self.accept_selected(cx);
    }

    fn accept_selected(&self, cx: &mut Context<Self>) {
        if let Some(chunks) = self.history.selected_chunks() {
            cx.emit(PromptHistoryPopoverEvent::Accepted(chunks.to_vec()));
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(PromptHistoryPopoverEvent::Dismissed);
    }

    #[cfg(test)]
    pub(super) fn selected_preview(&self) -> Option<&str> {
        self.history
            .selected_preview()
            .map(|preview| preview.as_ref())
    }

    #[cfg(test)]
    pub(super) fn horizontal_scroll_state(&self) -> (gpui::Pixels, gpui::Pixels) {
        (
            self.scroll_handle.offset().x,
            self.scroll_handle.max_offset().x,
        )
    }
}

impl Focusable for PromptHistoryPopover {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PromptHistoryPopover {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_index = self.history.selected_index();

        v_flex()
            .debug_selector(|| "agent-prompt-history-popover".to_string())
            .track_focus(&self.focus_handle)
            .key_context("AgentPromptHistory")
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .w_full()
            .min_w_0()
            .flex_shrink_1()
            .overflow_x_hidden()
            .elevation_2(cx)
            .child(
                v_flex()
                    .id("agent-prompt-history-list")
                    .w_full()
                    .min_w_0()
                    .max_h_40()
                    .p_1()
                    .overflow_x_hidden()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .children(
                        self.history
                            .entries()
                            .iter()
                            .enumerate()
                            .map(|(index, entry)| {
                                div()
                                    .w_full()
                                    .min_w_0()
                                    .overflow_x_hidden()
                                    .debug_selector(|| {
                                        format!("agent-prompt-history-entry-{index}")
                                    })
                                    .child(
                                        ListItem::new(("agent-prompt-history-entry", index))
                                            .inset(true)
                                            .toggle_state(index == selected_index)
                                            .on_click(cx.listener(
                                                move |this, _event, _window, cx| {
                                                    this.history.select(index);
                                                    this.accept_selected(cx);
                                                },
                                            ))
                                            .child(
                                                h_flex()
                                                    .w_full()
                                                    .min_w_0()
                                                    .overflow_x_hidden()
                                                    .debug_selector(move || {
                                                        format!(
                                                            "agent-prompt-history-preview-{index}"
                                                        )
                                                    })
                                                    .child(
                                                        Label::new(entry.preview().clone())
                                                            .size(LabelSize::Small)
                                                            .flex_1()
                                                            .truncate(),
                                                    ),
                                            ),
                                    )
                            }),
                    ),
            )
            .vertical_scrollbar_for(&self.scroll_handle, window, cx)
    }
}
