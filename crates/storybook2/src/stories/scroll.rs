use crate::themes::rose_pine;
use gpui3::{div, view, Context, ParentElement, Styled, View, WindowContext};

pub struct ScrollStory {
    text: View<()>,
}

impl ScrollStory {
    pub fn view(cx: &mut WindowContext) -> View<()> {
        let theme = rose_pine();

        view(cx.entity(|cx| ()), move |_, cx| {
            div()
                .id("parent")
                .bg(theme.lowest.base.default.background)
                .size_full()
                .overflow_x_scroll()
                .child(div().w_96().flex().flex_row().children((0..3).map(|ix| {
                    let bg = if ix % 2 == 0 {
                        theme.middle.positive.default.background
                    } else {
                        theme.middle.warning.default.background
                    };
                    div().bg(bg).flex_1().h_20()
                })))
        })
    }
}
