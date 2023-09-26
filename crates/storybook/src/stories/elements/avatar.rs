use ui::prelude::*;
use ui::Avatar;

use crate::story::Story;

#[derive(Element, Default)]
pub struct AvatarStory {}

impl AvatarStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, ui::Avatar>(cx))
            .child(Story::label(cx, "Default"))
            .child(Avatar::new(
                "https://avatars.githubusercontent.com/u/1714999?v=4",
            ))
            .child(Story::label(cx, "Rounded rectangle"))
            .child(
                Avatar::new("https://avatars.githubusercontent.com/u/1714999?v=4")
                    .shape(Shape::RoundedRectangle),
            )
    }
}
