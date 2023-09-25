use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use ui::avatar;
use ui::prelude::*;

use crate::story::Story;

#[derive(Element, Default)]
pub struct AvatarStory {}

impl AvatarStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, ui::Avatar>())
            .child(Story::label("Default"))
            .child(avatar(
                "https://avatars.githubusercontent.com/u/1714999?v=4",
            ))
            .child(Story::label("Rounded rectangle"))
            .child(
                avatar("https://avatars.githubusercontent.com/u/1714999?v=4")
                    .shape(Shape::RoundedRectangle),
            )
    }
}
