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
        let (background_color, bottom_opacity, top_opacity) = if cx.theme().appearance.is_light() {
            (cx.theme().colors().icon_muted, 0.05, 0.025)
        } else {
            (gpui::black(), 0.2, 0.1)
        };

        v_flex()
            .relative()
            .h(rems_from_px(155_f32))
            .items_center()
            .justify_center()
            .rounded_t_md()
            .overflow_hidden()
            .bg(linear_gradient(
                0.,
                linear_color_stop(background_color.opacity(bottom_opacity), 0.),
                linear_color_stop(background_color.opacity(top_opacity), 1.),
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
                svg()
                    .absolute()
                    .inset_0()
                    .size_full()
                    .path("images/delta_announcement_diagram.svg")
                    .text_color(cx.theme().colors().text),
            )
            .child(
                svg()
                    .absolute()
                    .inset_0()
                    .size_full()
                    .path("images/delta_mark.svg")
                    .text_color(gpui::rgb(0xd5442c)),
            )
    }
}
