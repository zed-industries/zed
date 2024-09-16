use gpui::Render;
use story::Story;
use strum::IntoEnumIterator;

use crate::{prelude::*, DecoratedIcon, IconDecoration};
use crate::{Icon, IconName};

pub struct IconStory;

impl Render for IconStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let icons = IconName::iter();

        Story::container(cx)
            .child(Story::title_for::<Icon>(cx))
            .child(Story::label(cx, "DecoratedIcon"))
            .child(DecoratedIcon::new(
                Icon::new(IconName::Bell).color(Color::Muted),
                IconDecoration::IndicatorDot,
            ))
            .child(
                DecoratedIcon::new(Icon::new(IconName::Bell), IconDecoration::IndicatorDot)
                    .decoration_color(Color::Accent),
            )
            .child(DecoratedIcon::new(
                Icon::new(IconName::Bell).color(Color::Muted),
                IconDecoration::Strikethrough,
            ))
            .child(
                DecoratedIcon::new(Icon::new(IconName::Bell), IconDecoration::X)
                    .decoration_color(Color::Error),
            )
            .child(Story::label(cx, "All Icons"))
            .child(div().flex().gap_3().children(icons.map(Icon::new)))
    }
}
