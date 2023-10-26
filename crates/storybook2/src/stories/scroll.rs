use crate::themes::rose_pine;
use gpui2::{
    div, px, view, Context, Element, ParentElement, SharedString, Styled, View, WindowContext,
};
use ui::ElementExt;

pub struct ScrollStory {
    text: View<()>,
}

impl ScrollStory {
    pub fn view(cx: &mut WindowContext) -> View<()> {
        let theme = rose_pine();

        view(cx.entity(|cx| ()), move |_, cx| checkerboard(1))
    }
}

fn checkerboard<S>(depth: usize) -> impl Element<S>
where
    S: 'static + Send + Sync,
{
    let theme = rose_pine();
    let color_1 = theme.lowest.positive.default.background;
    let color_2 = theme.lowest.warning.default.background;

    div()
        .id("parent")
        .bg(theme.lowest.base.default.background)
        .size_full()
        .overflow_scroll()
        .children((0..10).map(|row| {
            div()
                .w(px(1000.))
                .h(px(100.))
                .flex()
                .flex_row()
                .children((0..10).map(|column| {
                    let id = SharedString::from(format!("{}, {}", row, column));
                    let bg = if row % 2 == column % 2 {
                        color_1
                    } else {
                        color_2
                    };
                    div().id(id).bg(bg).size(px(100. / depth as f32)).when(
                        row >= 5 && column >= 5,
                        |d| {
                            d.overflow_scroll()
                                .child(div().size(px(50.)).bg(color_1))
                                .child(div().size(px(50.)).bg(color_2))
                                .child(div().size(px(50.)).bg(color_1))
                                .child(div().size(px(50.)).bg(color_2))
                        },
                    )
                }))
        }))
}
