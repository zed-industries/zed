use gpui::{FocusHandle, Focusable};
use ui::{
    Color, Context, IntoElement, Label, LabelCommon, ParentElement, Render, Styled, Window, h_flex,
};

pub(crate) struct FailedState {
    focus_handle: FocusHandle,
}
impl FailedState {
    pub(super) fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Focusable for FailedState {
    fn focus_handle(&self, _: &ui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
impl Render for FailedState {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .items_center()
            .justify_center()
            .child(Label::new("Failed to spawn debugging session").color(Color::Error))
    }
}
