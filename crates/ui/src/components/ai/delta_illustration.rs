use crate::prelude::*;
use gpui::{linear_color_stop, linear_gradient, svg};

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
            .h(rems_from_px(155_f32))
            .items_center()
            .justify_center()
            .rounded_t_md()
            .overflow_hidden()
            .bg(linear_gradient(
                0.,
                linear_color_stop(gpui::black().opacity(0.2), 0.),
                linear_color_stop(gpui::black().opacity(0.1), 1.),
            ))
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
                            .w(rems_from_px(220_f32))
                            .max_w_full()
                            .h(rems_from_px(220_f32 * 131. / 594.))
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
