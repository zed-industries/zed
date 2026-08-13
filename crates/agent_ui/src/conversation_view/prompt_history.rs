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
        if let Some(chunks) = self.history.selected_chunks() {
            cx.emit(PromptHistoryPopoverEvent::Accepted(chunks.to_vec()));
        }
    }

    #[cfg(test)]
    pub(super) fn selected_preview(&self) -> Option<&str> {
        self.history
            .selected_preview()
            .map(|preview| preview.as_ref())
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
            .track_focus(&self.focus_handle)
            .key_context("AgentPromptHistory")
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::confirm))
            .w_full()
            .flex_shrink_1()
            .elevation_2(cx)
            .child(
                v_flex()
                    .id("agent-prompt-history-list")
                    .w_full()
                    .max_h_40()
                    .p_1()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .children(
                        self.history
                            .entries()
                            .iter()
                            .enumerate()
                            .map(|(index, entry)| {
                                ListItem::new(("agent-prompt-history-entry", index))
                                    .inset(true)
                                    .toggle_state(index == selected_index)
                                    .child(
                                        h_flex().w_full().min_w_0().child(
                                            Label::new(entry.preview().clone())
                                                .size(LabelSize::Small)
                                                .truncate(),
                                        ),
                                    )
                            }),
                    ),
            )
            .vertical_scrollbar_for(&self.scroll_handle, window, cx)
    }
}
