use crate::prelude::*;
use crate::{Icon, IconButton, Label, Panel, PanelSide};
use gpui::{prelude::*, rems, AbsoluteLength};

#[derive(Component)]
pub struct AssistantPanel {
    id: ElementId,
    current_side: PanelSide,
}

impl AssistantPanel {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            current_side: PanelSide::default(),
        }
    }

    pub fn side(mut self, side: PanelSide) -> Self {
        self.current_side = side;
        self
    }

    fn render<V: 'static>(self, view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
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
                .render()])
            .side(self.current_side)
            .width(AbsoluteLength::Rems(rems(32.)))
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::Story;
    use gpui::{Div, Render};
    pub struct AssistantPanelStory;

    impl Render for AssistantPanelStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, AssistantPanel>(cx))
                .child(Story::label(cx, "Default"))
                .child(AssistantPanel::new("assistant-panel"))
        }
    }
}
