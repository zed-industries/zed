use gpui::{App, FocusHandle, Focusable};
use ui::{div, Context, Element, InteractiveElement, ParentElement, Render, Styled};

pub(super) struct InertState {
    focus_handle: FocusHandle,
}

impl InertState {
    pub(super) fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}
impl Focusable for InertState {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for InertState {
    fn render(
        &mut self,
        _window: &mut ui::Window,
        _cx: &mut ui::Context<'_, Self>,
    ) -> impl ui::IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .child("No debug sessions")
    }
}
