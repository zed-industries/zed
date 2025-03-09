use gpui::{
    actions, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    ParentElement, Render, Styled, WeakEntity, Window,
};

pub struct IndentSizeSelector {
}

impl IndentSizeSelector {
    fn new() -> Self {
        Self {}
    }
}

impl Render for IndentSizeSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    }
}