use crate::prelude::*;
use crate::theme;

#[derive(Clone)]
pub struct ToolbarItem {}

#[derive(Element)]
pub struct Toolbar<S: 'static + Send + Sync> {
    left_items: HackyChildren<S>,
    left_items_payload: HackyChildrenPayload,
    right_items: HackyChildren<S>,
    right_items_payload: HackyChildrenPayload,
}

impl<S: 'static + Send + Sync> Toolbar<S> {
    pub fn new(
        left_items: HackyChildren<S>,
        left_items_payload: HackyChildrenPayload,
        right_items: HackyChildren<S>,
        right_items_payload: HackyChildrenPayload,
    ) -> Self {
        Self {
            left_items,
            left_items_payload,
            right_items,
            right_items_payload,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let theme = theme(cx);

        div()
            .fill(theme.highest.base.default.background)
            .p_2()
            .flex()
            .justify_between()
            .child(
                div()
                    .flex()
                    .children_any((self.left_items)(cx, self.left_items_payload.as_ref())),
            )
            .child(
                div()
                    .flex()
                    .children_any((self.right_items)(cx, self.right_items_payload.as_ref())),
            )
    }
}
