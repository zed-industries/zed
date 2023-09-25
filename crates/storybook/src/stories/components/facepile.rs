use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use ui::prelude::*;
use ui::{facepile, Avatar};

use crate::story::Story;

#[derive(Element, Default)]
pub struct FacepileStory {}

impl FacepileStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let avatars = vec![
            Avatar::new("https://avatars.githubusercontent.com/u/1714999?v=4"),
            Avatar::new("https://avatars.githubusercontent.com/u/482957?v=4"),
            Avatar::new("https://avatars.githubusercontent.com/u/1789?v=4"),
        ];

        Story::container(cx)
            .child(Story::title_for::<_, ui::Facepile>(cx))
            .child(Story::label(cx, "Default"))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(facepile(avatars.clone().into_iter().take(1)))
                    .child(facepile(avatars.clone().into_iter().take(2)))
                    .child(facepile(avatars.clone().into_iter().take(3))),
            )
            .child(Story::label(cx, "Rounded rectangle avatars"))
            .child({
                let shape = Shape::RoundedRectangle;

                let avatars = avatars
                    .clone()
                    .into_iter()
                    .map(|avatar| avatar.shape(Shape::RoundedRectangle));

                div()
                    .flex()
                    .gap_3()
                    .child(facepile(avatars.clone().take(1)))
                    .child(facepile(avatars.clone().take(2)))
                    .child(facepile(avatars.clone().take(3)))
            })
    }
}
