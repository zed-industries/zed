use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use ui::Label;

use crate::story::Story;

#[derive(Element, Default)]
pub struct LabelStory {}

impl LabelStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, Label>(cx))
            .child(Story::label(cx, "Default"))
            .child(Label::new("Hello, world!"))
            .child(Story::label(cx, "Highlighted"))
            .child(Label::new("Hello, world!").with_highlights(vec![0, 1, 2, 7, 8, 12]))
    }
}
