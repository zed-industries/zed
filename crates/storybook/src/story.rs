use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{rgb, Element, Hsla, ParentElement};

pub struct Story {}

impl Story {
    pub fn title<V: 'static>(title: &str) -> impl Element<V> {
        div()
            .text_2xl()
            .text_color(rgb::<Hsla>(0xffffff))
            .child(title.to_owned())
    }
}
