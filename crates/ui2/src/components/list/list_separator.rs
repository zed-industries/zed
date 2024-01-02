use gpui::Div;

use crate::prelude::*;

#[derive(IntoElement)]
pub struct ListSeparator;

impl RenderOnce for ListSeparator {
    type Output = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Output {
        div()
            .h_px()
            .w_full()
            .my_1()
            .bg(cx.theme().colors().border_variant)
    }
}
