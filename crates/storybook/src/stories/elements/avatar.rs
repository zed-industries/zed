use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{rgb, Element, Hsla, IntoElement, ParentElement, ViewContext};
use ui::prelude::*;
use ui::{avatar, theme};

#[derive(Element, Default)]
pub struct AvatarStory {}

impl AvatarStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .size_full()
            .flex()
            .flex_col()
            .pt_2()
            .px_4()
            .child(
                div()
                    .text_2xl()
                    .text_color(rgb::<Hsla>(0xffffff))
                    .child(std::any::type_name::<ui::Avatar>()),
            )
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(avatar(
                        "https://avatars.githubusercontent.com/u/1714999?v=4",
                    ))
                    .child(
                        avatar("https://avatars.githubusercontent.com/u/1714999?v=4")
                            .shape(Shape::RoundedRectangle),
                    ),
            )
    }
}
