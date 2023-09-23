use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use ui::{input, theme};

use crate::story::Story;

#[derive(Element, Default)]
pub struct InputStory {}

impl InputStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        Story::container()
            .child(Story::title_for::<_, ui::Input>())
            .child(Story::label("Default"))
            .child(div().flex().child(input("Search")))
    }
}
