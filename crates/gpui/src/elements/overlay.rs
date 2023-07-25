use std::ops::Range;

use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json::ToJson,
    AnyElement, Axis, Element, LayoutContext, MouseRegion, PaintContext, SceneBuilder,
    SizeConstraint, View, ViewContext,
};
use serde_json::json;

pub struct Overlay<V: View> {
    child: AnyElement<V>,
    anchor_position: Option<Vector2F>,
    anchor_corner: AnchorCorner,
    fit_mode: OverlayFitMode,
    position_mode: OverlayPositionMode,
    hoverable: bool,
    z_index: Option<usize>,
}

#[derive(Copy, Clone)]
pub enum OverlayFitMode {
    SnapToWindow,
    SwitchAnchor,
    None,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OverlayPositionMode {
    Window,
    Local,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnchorCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl AnchorCorner {
    fn get_bounds(&self, anchor_position: Vector2F, size: Vector2F) -> RectF {
        match self {
            Self::TopLeft => RectF::from_points(anchor_position, anchor_position + size),
            Self::TopRight => RectF::from_points(
                anchor_position - Vector2F::new(size.x(), 0.),
                anchor_position + Vector2F::new(0., size.y()),
            ),
            Self::BottomLeft => RectF::from_points(
                anchor_position - Vector2F::new(0., size.y()),
                anchor_position + Vector2F::new(size.x(), 0.),
            ),
            Self::BottomRight => RectF::from_points(anchor_position - size, anchor_position),
        }
    }

    fn switch_axis(self, axis: Axis) -> Self {
        match axis {
            Axis::Vertical => match self {
                AnchorCorner::TopLeft => AnchorCorner::BottomLeft,
                AnchorCorner::TopRight => AnchorCorner::BottomRight,
                AnchorCorner::BottomLeft => AnchorCorner::TopLeft,
                AnchorCorner::BottomRight => AnchorCorner::TopRight,
            },
            Axis::Horizontal => match self {
                AnchorCorner::TopLeft => AnchorCorner::TopRight,
                AnchorCorner::TopRight => AnchorCorner::TopLeft,
                AnchorCorner::BottomLeft => AnchorCorner::BottomRight,
                AnchorCorner::BottomRight => AnchorCorner::BottomLeft,
            },
        }
    }
}

impl<V: View> Overlay<V> {
    pub fn new(child: impl Element<V>) -> Self {
        Self {
            child: child.into_any(),
            anchor_position: None,
            anchor_corner: AnchorCorner::TopLeft,
            fit_mode: OverlayFitMode::None,
            position_mode: OverlayPositionMode::Window,
            hoverable: false,
            z_index: None,
        }
    }

    pub fn with_anchor_position(mut self, position: Vector2F) -> Self {
        self.anchor_position = Some(position);
        self
    }

    pub fn with_anchor_corner(mut self, anchor_corner: AnchorCorner) -> Self {
        self.anchor_corner = anchor_corner;
        self
    }

    pub fn with_fit_mode(mut self, fit_mode: OverlayFitMode) -> Self {
        self.fit_mode = fit_mode;
        self
    }

    pub fn with_position_mode(mut self, position_mode: OverlayPositionMode) -> Self {
        self.position_mode = position_mode;
        self
    }

    pub fn with_hoverable(mut self, hoverable: bool) -> Self {
        self.hoverable = hoverable;
        self
    }

    pub fn with_z_index(mut self, z_index: usize) -> Self {
        self.z_index = Some(z_index);
        self
    }
}

impl<V: View> Element<V> for Overlay<V> {
    type LayoutState = Vector2F;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let constraint = if self.anchor_position.is_some() {
            SizeConstraint::new(Vector2F::zero(), cx.window_size())
        } else {
            constraint
        };
        let size = self.child.layout(constraint, view, cx);
        (Vector2F::zero(), size)
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        _: RectF,
        size: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) {
        let (anchor_position, mut bounds) = match self.position_mode {
            OverlayPositionMode::Window => {
                let anchor_position = self.anchor_position.unwrap_or_else(|| bounds.origin());
                let bounds = self.anchor_corner.get_bounds(anchor_position, *size);
                (anchor_position, bounds)
            }
            OverlayPositionMode::Local => {
                let anchor_position = self.anchor_position.unwrap_or_default();
                let bounds = self
                    .anchor_corner
                    .get_bounds(bounds.origin() + anchor_position, *size);
                (anchor_position, bounds)
            }
        };

        match self.fit_mode {
            OverlayFitMode::SnapToWindow => {
                // Snap the horizontal edges of the overlay to the horizontal edges of the window if
                // its horizontal bounds overflow
                if bounds.max_x() > cx.window_size().x() {
                    let mut lower_right = bounds.lower_right();
                    lower_right.set_x(cx.window_size().x());
                    bounds = RectF::from_points(lower_right - *size, lower_right);
                } else if bounds.min_x() < 0. {
                    let mut upper_left = bounds.origin();
                    upper_left.set_x(0.);
                    bounds = RectF::from_points(upper_left, upper_left + *size);
                }

                // Snap the vertical edges of the overlay to the vertical edges of the window if
                // its vertical bounds overflow.
                if bounds.max_y() > cx.window_size().y() {
                    let mut lower_right = bounds.lower_right();
                    lower_right.set_y(cx.window_size().y());
                    bounds = RectF::from_points(lower_right - *size, lower_right);
                } else if bounds.min_y() < 0. {
                    let mut upper_left = bounds.origin();
                    upper_left.set_y(0.);
                    bounds = RectF::from_points(upper_left, upper_left + *size);
                }
            }
            OverlayFitMode::SwitchAnchor => {
                let mut anchor_corner = self.anchor_corner;

                if bounds.max_x() > cx.window_size().x() {
                    anchor_corner = anchor_corner.switch_axis(Axis::Horizontal);
                }

                if bounds.max_y() > cx.window_size().y() {
                    anchor_corner = anchor_corner.switch_axis(Axis::Vertical);
                }

                if bounds.min_x() < 0. {
                    anchor_corner = anchor_corner.switch_axis(Axis::Horizontal)
                }

                if bounds.min_y() < 0. {
                    anchor_corner = anchor_corner.switch_axis(Axis::Vertical)
                }

                // Update bounds if needed
                if anchor_corner != self.anchor_corner {
                    bounds = anchor_corner.get_bounds(anchor_position, *size)
                }
            }
            OverlayFitMode::None => {}
        }

        scene.paint_stacking_context(None, self.z_index, |scene| {
            if self.hoverable {
                enum OverlayHoverCapture {}
                // Block hovers in lower stacking contexts
                scene.push_mouse_region(MouseRegion::new::<OverlayHoverCapture>(
                    cx.view_id(),
                    cx.view_id(),
                    bounds,
                ));
            }

            self.child.paint(
                scene,
                bounds.origin(),
                RectF::new(Vector2F::zero(), cx.window_size()),
                view,
                cx,
            );
        });
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        self.child.rect_for_text_range(range_utf16, view, cx)
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> serde_json::Value {
        json!({
            "type": "Overlay",
            "abs_position": self.anchor_position.to_json(),
            "child": self.child.debug(view, cx),
        })
    }
}
