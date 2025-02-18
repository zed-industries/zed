use gpui::{FocusHandle, Focusable};
use ui::{div, Element, ParentElement, Render, Styled};

pub(super) struct StartingState {
    focus_handle: FocusHandle,
}

impl Focusable for StartingState {
    fn focus_handle(&self, cx: &ui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for StartingState {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<'_, Self>,
    ) -> impl ui::IntoElement {
        div().size_full().child("Starting a debug adapter")
    }
}
