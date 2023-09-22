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

        Story::container()
            .child(Story::title(std::any::type_name::<ui::Facepile>()))
            .child(Story::label("Default"))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(facepile(vec![avatar(
                        "https://avatars.githubusercontent.com/u/1714999?v=4",
                    )]))
                    .child(facepile(vec![
                        avatar("https://avatars.githubusercontent.com/u/1714999?v=4"),
                        avatar("https://avatars.githubusercontent.com/u/482957?v=4"),
                    ]))
                    .child(facepile(vec![
                        avatar("https://avatars.githubusercontent.com/u/1714999?v=4"),
                        avatar("https://avatars.githubusercontent.com/u/482957?v=4"),
                        avatar("https://avatars.githubusercontent.com/u/1789?v=4"),
                    ])),
            )
            .child(Story::label("Rounded rectangle avatars"))
            .child({
                let shape = Shape::RoundedRectangle;

                div()
                    .flex()
                    .gap_3()
                    .child(facepile(vec![avatar(
                        "https://avatars.githubusercontent.com/u/1714999?v=4",
                    )
                    .shape(shape)]))
                    .child(facepile(vec![
                        avatar("https://avatars.githubusercontent.com/u/1714999?v=4").shape(shape),
                        avatar("https://avatars.githubusercontent.com/u/482957?v=4").shape(shape),
                    ]))
                    .child(facepile(vec![
                        avatar("https://avatars.githubusercontent.com/u/1714999?v=4").shape(shape),
                        avatar("https://avatars.githubusercontent.com/u/482957?v=4").shape(shape),
                        avatar("https://avatars.githubusercontent.com/u/1789?v=4").shape(shape),
                    ]))
            })
    }
}
