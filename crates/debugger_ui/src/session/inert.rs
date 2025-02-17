use gpui::{App, FocusHandle, Focusable};
use ui::{div, Element, ParentElement, Render, Styled};

pub(super) struct InertState {
    focus_handle: FocusHandle,
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
        div().size_full().child("No debug sessions")
    }
}
