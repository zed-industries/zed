use std::marker::PhantomData;

use gpui2::{rems, AbsoluteLength};

use crate::prelude::*;
use crate::{Icon, IconButton, Label, Panel, PanelSide};

#[derive(Element)]
pub struct AssistantPanel<S: 'static + Send + Sync> {
    id: ElementId,
    state_type: PhantomData<S>,
    current_side: PanelSide,
}

impl<S: 'static + Send + Sync> AssistantPanel<S> {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            state_type: PhantomData,
            current_side: PanelSide::default(),
        }
    }

    pub fn side(mut self, side: PanelSide) -> Self {
        self.current_side = side;
        self
    }

    fn render(&mut self, view: &mut S, cx: &mut ViewContext<S>) -> impl IntoAnyElement<S> {
        Panel::new(self.id.clone(), cx)
            .children(vec![div()
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
                                .child(IconButton::new("menu", Icon::Menu))
                                .child(Label::new("New Conversation")),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_px()
                                .child(IconButton::new("split_message", Icon::SplitMessage))
                                .child(IconButton::new("quote", Icon::Quote))
                                .child(IconButton::new("magic_wand", Icon::MagicWand))
                                .child(IconButton::new("plus", Icon::Plus))
                                .child(IconButton::new("maximize", Icon::Maximize)),
                        ),
                )
                // Chat Body
                .child(
                    div()
                        .id("chat-body")
                        .w_full()
                        .flex()
                        .flex_col()
                        .gap_3()
                        .overflow_y_scroll()
                        .child(Label::new("Is this thing on?")),
                )
                .into_any()])
            .side(self.current_side)
            .width(AbsoluteLength::Rems(rems(32.)))
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::Story;

    use super::*;

    #[derive(Element)]
    pub struct AssistantPanelStory<S: 'static + Send + Sync> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync> AssistantPanelStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl IntoAnyElement<S> {
            Story::container(cx)
                .child(Story::title_for::<_, AssistantPanel<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(AssistantPanel::new("assistant-panel"))
        }
    }
}
