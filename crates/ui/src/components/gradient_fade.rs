use gpui::{Hsla, Pixels, SharedString, linear_color_stop, linear_gradient, px};

use crate::prelude::*;

/// A gradient overlay that fades from a solid color to transparent.
#[derive(IntoElement)]
pub struct GradientFade {
    base_bg: Hsla,
    hover_bg: Hsla,
    active_bg: Hsla,
    width: Pixels,
    right: Pixels,
    gradient_stop: f32,
    group_name: Option<SharedString>,
}

impl GradientFade {
    pub fn new(base_bg: Hsla, hover_bg: Hsla, active_bg: Hsla) -> Self {
        Self {
            base_bg,
            hover_bg,
            active_bg,
            width: px(48.0),
            right: px(0.0),
            gradient_stop: 0.6,
            group_name: None,
        }
    }

    pub fn width(mut self, width: Pixels) -> Self {
        self.width = width;
        self
    }

    pub fn right(mut self, right: Pixels) -> Self {
        self.right = right;
        self
    }

    pub fn gradient_stop(mut self, stop: f32) -> Self {
        self.gradient_stop = stop;
        self
    }

    pub fn group_name(mut self, name: impl Into<SharedString>) -> Self {
        self.group_name = Some(name.into());
        self
    }
}

impl RenderOnce for GradientFade {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let stop = self.gradient_stop;
        let hover_bg = self.hover_bg;
        let active_bg = self.active_bg;

        div()
            .id("gradient_fade")
            .absolute()
            .top_0()
            .right(self.right)
            .w(self.width)
            .h_full()
            .bg(linear_gradient(
                90.,
                linear_color_stop(self.base_bg, stop),
                linear_color_stop(self.base_bg.opacity(0.0), 0.),
            ))
            .when_some(self.group_name.clone(), |element, group_name| {
                element.group_hover(group_name, move |s| {
                    s.bg(linear_gradient(
                        90.,
                        linear_color_stop(hover_bg, stop),
                        linear_color_stop(hover_bg.opacity(0.0), 0.),
                    ))
                })
            })
            .when_some(self.group_name, |element, group_name| {
                element.group_active(group_name, move |s| {
                    s.bg(linear_gradient(
                        90.,
                        linear_color_stop(active_bg, stop),
                        linear_color_stop(active_bg.opacity(0.0), 0.),
                    ))
                })
            })
    }
}
