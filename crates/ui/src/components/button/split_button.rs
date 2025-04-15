use gpui::{
    AnyElement, App, BoxShadow, IntoElement, ParentElement, RenderOnce, Styled, Window, div, hsla,
    point, px,
};
use theme::ActiveTheme;

use crate::{ElevationIndex, h_flex};

use super::ButtonLike;

/// /// A button with two parts: a primary action on the left and a secondary action on the right.
///
/// The left side is a [`ButtonLike`] with the main action, while the right side can contain
/// any element (typically a dropdown trigger or similar).
///
/// The two sections are visually separated by a divider, but presented as a unified control.
#[derive(IntoElement)]
pub struct SplitButton {
    pub left: ButtonLike,
    pub right: AnyElement,
}

impl SplitButton {
    pub fn new(left: ButtonLike, right: AnyElement) -> Self {
        Self { left, right }
    }
}

impl RenderOnce for SplitButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .rounded_sm()
            .border_1()
            .border_color(cx.theme().colors().text_muted.alpha(0.12))
            .child(div().flex_grow().child(self.left))
            .child(
                div()
                    .h_full()
                    .w_px()
                    .bg(cx.theme().colors().text_muted.alpha(0.16)),
            )
            .child(self.right)
            .bg(ElevationIndex::Surface.on_elevation_bg(cx))
            .shadow(smallvec::smallvec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.16),
                offset: point(px(0.), px(1.)),
                blur_radius: px(0.),
                spread_radius: px(0.),
            }])
    }
}
