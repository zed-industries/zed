use crate::prelude::*;
use gpui::svg;

#[derive(IntoElement)]
pub struct DeltaIllustration;

impl DeltaIllustration {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for DeltaIllustration {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .relative()
            .h(rems_from_px(200_f32))
            .items_center()
            .justify_center()
            .rounded_t_md()
            .overflow_hidden()
            .bg(gpui::black().opacity(0.2))
            .child(
                svg()
                    .absolute()
                    .inset_0()
                    .size_full()
                    .path("images/delta_announcement_grid.svg")
                    .text_color(cx.theme().colors().border),
            )
            .child(
                h_flex()
                    .absolute()
                    .inset_0()
                    .justify_center()
                    .px(rems_from_px(64_f32))
                    .child(
                        div()
                            .relative()
                            .w(rems_from_px(320_f32))
                            .max_w_full()
                            .h(rems_from_px(320_f32 * 131. / 594.))
                            .flex_none()
                            .child(
                                svg()
                                    .absolute()
                                    .inset_0()
                                    .size_full()
                                    .path("images/delta_wordmark.svg")
                                    .text_color(cx.theme().colors().text),
                            )
                            .child(
                                svg()
                                    .absolute()
                                    .inset_0()
                                    .size_full()
                                    .path("images/delta_mark.svg")
                                    .text_color(gpui::rgb(0xd5442c)),
                            ),
                    ),
            )
    }
}
