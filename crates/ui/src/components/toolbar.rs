use crate::prelude::*;
use crate::theme;

#[derive(Clone)]
pub struct ToolbarItem {}

#[derive(Element)]
pub struct Toolbar<V: 'static> {
    left_items: HackyChildren<V>,
    left_items_payload: HackyChildrenPayload,
    right_items: HackyChildren<V>,
    right_items_payload: HackyChildrenPayload,
}

impl<V: 'static> Toolbar<V> {
    pub fn new(
        left_items: HackyChildren<V>,
        left_items_payload: HackyChildrenPayload,
        right_items: HackyChildren<V>,
        right_items_payload: HackyChildrenPayload,
    ) -> Self {
        Self {
            left_items,
            left_items_payload,
            right_items,
            right_items_payload,
        }
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
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
