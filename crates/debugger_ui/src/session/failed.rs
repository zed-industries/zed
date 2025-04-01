use dap::client::SessionId;
use gpui::{FocusHandle, Focusable};
use ui::{
    Color, Context, IntoElement, Label, LabelCommon, ParentElement, Render, Styled, Window, h_flex,
};

pub(crate) struct FailedState {
    session_id: SessionId,
    focus_handle: FocusHandle,
}
impl FailedState {
    pub(super) fn new(session_id: SessionId, cx: &mut Context<Self>) -> Self {
        Self {
            session_id,
            focus_handle: cx.focus_handle(),
        }
    }
    pub(crate) fn session_id(&self) -> SessionId {
        self.session_id
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
