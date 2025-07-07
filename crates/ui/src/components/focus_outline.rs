use gpui::{
    AnyElement, IntoElement, ParentElement, Pixels, RenderOnce, Styled, px, transparent_black,
};
use smallvec::SmallVec;
use theme::ActiveTheme;

use crate::{h_flex, utils::CornerSolver};

/// An outline is a stylistic focus indicator that draws a ring around
/// an element with some space between the element and ring.
#[derive(IntoElement)]
pub struct FocusOutline {
    corner_radius: Pixels,
    border_width: Pixels,
    padding: Pixels,
    focused: bool,
    active: bool,
    children: SmallVec<[AnyElement; 2]>,
}

impl FocusOutline {
    pub fn new(child_corner_radius: Pixels, focused: bool, offset: Pixels) -> Self {
        let ring_width = px(1.);
        let corner_radius =
            CornerSolver::parent_radius(child_corner_radius, ring_width, offset, px(0.));
        Self {
            corner_radius,
            border_width: ring_width,
            padding: offset,
            focused,
            active: false,
            children: SmallVec::new(),
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

impl RenderOnce for FocusOutline {
    fn render(self, _window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        let border_color = if self.focused && self.active {
            cx.theme().colors().border_focused.opacity(0.48)
        } else if self.focused {
            cx.theme().colors().border_variant
        } else {
            transparent_black()
        };

        h_flex()
            .border(self.border_width)
            .border_color(border_color)
            .rounded(self.corner_radius)
            .p(self.padding)
            .children(self.children)
    }
}

impl ParentElement for FocusOutline {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
