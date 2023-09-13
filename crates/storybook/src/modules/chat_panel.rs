use std::marker::PhantomData;

use crate::components::icon_button;
use crate::theme::theme;
use gpui2::elements::div::ScrollState;
use gpui2::style::StyleHelpers;
use gpui2::{elements::div, IntoElement};
use gpui2::{Element, ParentElement, ViewContext};

#[derive(Element)]
pub struct ChatPanel<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
}

pub fn chat_panel<V: 'static>(scroll_state: ScrollState) -> ChatPanel<V> {
    ChatPanel {
        view_type: PhantomData,
        scroll_state,
    }
}

impl<V: 'static> ChatPanel<V> {
    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .h_full()
            .flex()
            // Header
            .child(
                div()
                    .px_2()
                    .flex()
                    .gap_2()
                    // Nav Buttons
                    .child("#gpui2"),
            )
            // Chat Body
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .overflow_y_scroll(self.scroll_state.clone())
                    .child("body"),
            )
            // Composer
            .child(
                div()
                    .px_2()
                    .flex()
                    .gap_2()
                    // Nav Buttons
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_px()
                            .child(icon_button("icons/plus.svg"))
                            .child(icon_button("icons/split.svg")),
                    ),
            )
    }
}
