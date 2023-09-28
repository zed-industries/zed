use std::marker::PhantomData;

use gpui2::geometry::rems;

use crate::prelude::*;
use crate::theme::theme;
use crate::{Icon, IconButton, Label, Panel, PanelSide};

#[derive(Element)]
pub struct AssistantPanel<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
    current_side: PanelSide,
}

impl<V: 'static> AssistantPanel<V> {
    pub fn new() -> Self {
        Self {
            view_type: PhantomData,
            scroll_state: ScrollState::default(),
            current_side: PanelSide::default(),
        }
    }

    pub fn side(mut self, side: PanelSide) -> Self {
        self.current_side = side;
        self
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        struct PanelPayload {
            pub scroll_state: ScrollState,
        }

        Panel::new(
            self.scroll_state.clone(),
            |_, payload| {
                let payload = payload.downcast_ref::<PanelPayload>().unwrap();

                vec![div()
                    .flex()
                    .flex_col()
                    .h_full()
                    .px_2()
                    .gap_2()
                    // Header
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .gap_2()
                            .child(
                                div()
                                    .flex()
                                    .child(IconButton::new(Icon::Menu))
                                    .child(Label::new("New Conversation")),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_px()
                                    .child(IconButton::new(Icon::SplitMessage))
                                    .child(IconButton::new(Icon::Quote))
                                    .child(IconButton::new(Icon::MagicWand))
                                    .child(IconButton::new(Icon::Plus))
                                    .child(IconButton::new(Icon::Maximize)),
                            ),
                    )
                    // Chat Body
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .overflow_y_scroll(payload.scroll_state.clone())
                            .child(Label::new("Is this thing on?")),
                    )
                    .into_any()]
            },
            Box::new(PanelPayload {
                scroll_state: self.scroll_state.clone(),
            }),
        )
        .side(self.current_side)
        .width(rems(32.))
    }
}
