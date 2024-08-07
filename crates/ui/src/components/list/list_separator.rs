use crate::prelude::*;

#[derive(IntoElement)]
pub struct ListSeparator;

impl RenderOnce for ListSeparator {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .h_px()
            .w_full()
            .my_1p5()
            .bg(cx.theme().colors().border_variant)
    }
}
