use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use ui::prelude::*;
use ui::{text_button, theme};

use crate::story::Story;

#[derive(Element, Default)]
pub struct TextButtonStory {}

impl TextButtonStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        Story::container()
            .child(Story::title_for::<_, ui::TextButton>())
            .child(Story::label("Default"))
            .child(div().flex().child(text_button("Click me")))
            .child(Story::label("Filled variant"))
            .child(
                div()
                    .flex()
                    .child(text_button("Click me").variant(ButtonVariant::Filled)),
            )
    }
}
