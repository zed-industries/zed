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
}

impl Focusable for PromptHistoryPopover {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PromptHistoryPopover {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .key_context("AgentPromptHistory")
    }
}
