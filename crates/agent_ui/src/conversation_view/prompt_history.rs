use gpui::{App, FocusHandle, Focusable, IntoElement, Render, Window};
use ui::prelude::*;

use super::PromptHistory;

pub(super) struct PromptHistoryPopover {
    history: PromptHistory,
    focus_handle: FocusHandle,
}

impl PromptHistoryPopover {
    pub(super) fn new(history: PromptHistory, cx: &mut Context<Self>) -> Self {
        Self {
            history,
            focus_handle: cx.focus_handle(),
        }
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.history.select_previous();
        cx.notify();
    }

    fn select_next(&mut self, _: &menu::SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        self.history.select_next();
        cx.notify();
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .key_context("AgentPromptHistory")
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_next))
    }
}
