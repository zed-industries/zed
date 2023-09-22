use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use ui::prelude::*;
use ui::{avatar, facepile, theme};

use crate::story::Story;

#[derive(Element, Default)]
pub struct FacepileStory {}

impl FacepileStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        let avatars = vec![
            avatar("https://avatars.githubusercontent.com/u/1714999?v=4"),
            avatar("https://avatars.githubusercontent.com/u/482957?v=4"),
            avatar("https://avatars.githubusercontent.com/u/1789?v=4"),
        ];

        Story::container()
            .child(Story::title_for::<_, ui::Facepile>())
            .child(Story::label("Default"))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(facepile(avatars.clone().into_iter().take(1)))
                    .child(facepile(avatars.clone().into_iter().take(2)))
                    .child(facepile(avatars.clone().into_iter().take(3))),
            )
            .child(Story::label("Rounded rectangle avatars"))
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
