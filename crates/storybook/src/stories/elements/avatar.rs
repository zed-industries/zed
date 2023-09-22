use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{rgb, Element, Hsla, IntoElement, ParentElement, ViewContext};
use ui::prelude::*;
use ui::{avatar, theme};

use crate::story::Story;

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
            .font("Zed Mono Extended")
            .fill(rgb::<Hsla>(0x282c34))
            .child(Story::title(std::any::type_name::<ui::Avatar>()))
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
