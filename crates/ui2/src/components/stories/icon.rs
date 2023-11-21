#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use gpui::{Div, Render};
    use strum::IntoEnumIterator;

    use crate::Story;

    use super::*;

    pub struct IconStory;

    impl Render for IconStory {
        type Element = Div;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            let icons = Icon::iter();

            Story::container(cx)
                .child(Story::title_for::<IconElement>(cx))
                .child(Story::label(cx, "All Icons"))
                .child(div().flex().gap_3().children(icons.map(IconElement::new)))
        }
    }
}
