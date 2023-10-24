use crate::prelude::*;
use crate::{Icon, IconButton, Label, Panel, PanelSide};
use gpui2::{div, rems, AbsoluteLength, IntoAnyElement};

#[derive(IntoAnyElement)]
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

    fn render<V>(mut self) -> impl IntoAnyElement<V> {
        div()

        // let color = ThemeColor::new(cx);

        // Panel::new(self.id, cx)
        //     .children(vec![div()
        //         .flex()
        //         .flex_col()
        //         .h_full()
        //         .px_2()
        //         .gap_2()
        //         // Header
        //         .child(
        //             div()
        //                 .flex()
        //                 .justify_between()
        //                 .gap_2()
        //                 .child(
        //                     div()
        //                         .flex()
        //                         .child(IconButton::new(Icon::Menu))
        //                         .child(Label::new("New Conversation")),
        //                 )
        //                 .child(
        //                     div()
        //                         .flex()
        //                         .items_center()
        //                         .gap_px()
        //                         .child(IconButton::new(Icon::SplitMessage))
        //                         .child(IconButton::new(Icon::Quote))
        //                         .child(IconButton::new(Icon::MagicWand))
        //                         .child(IconButton::new(Icon::Plus))
        //                         .child(IconButton::new(Icon::Maximize)),
        //                 ),
        //         )
        //         // Chat Body
        //         .child(
        //             div()
        //                 .id("chat-body")
        //                 .w_full()
        //                 .flex()
        //                 .flex_col()
        //                 .gap_3()
        //                 .overflow_y_scroll()
        //                 .child(Label::new("Is this thing on?")),
        //         )
        //         .into_any()])
        //     .side(self.current_side)
        //     .width(AbsoluteLength::Rems(rems(32.)))
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use gpui2::AppContext;

    use crate::Story;

    use super::*;

    #[derive(IntoAnyElement)]
    pub struct AssistantPanelStory<'a>(&'a AppContext);

    impl<'a> AssistantPanelStory<'a> {
        pub fn new() -> Self {
            Self
        }

        fn render<V>(self) -> impl IntoAnyElement<V> {
            Story::container(self.0)
                .child(Story::title_for::<_, AssistantPanel>(self.0))
                .child(Story::label(self.cx, "Default"))
                .child(AssistantPanel::new("assistant-panel"))
        }
    }
}
