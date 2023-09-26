use std::marker::PhantomData;

use gpui2::elements::div::ScrollState;
use gpui2::{elements::div, IntoElement};
use gpui2::{Element, ParentElement, ViewContext};

use crate::prelude::*;
use crate::theme::theme;
use crate::{IconAsset, IconButton};

#[derive(Element)]
pub struct ChatPanel<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
}

impl<V: 'static> ChatPanel<V> {
    pub fn new(scroll_state: ScrollState) -> Self {
        Self {
            view_type: PhantomData,
            scroll_state,
        }
    }

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
                            .child(IconButton::new(IconAsset::Plus))
                            .child(IconButton::new(IconAsset::Split)),
                    ),
            )
    }
}
