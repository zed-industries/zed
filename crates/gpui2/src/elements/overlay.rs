use smallvec::SmallVec;

use crate::{
    point, AnyElement, BorrowWindow, Bounds, Component, Element, LayoutId, ParentComponent, Pixels,
    Point, Size, Style,
};

pub struct OverlayState {
    child_layout_ids: SmallVec<[LayoutId; 4]>,
}

pub struct Overlay<V> {
    children: SmallVec<[AnyElement<V>; 2]>,
    anchor_corner: AnchorCorner,
    fit_mode: OverlayFitMode,
    // todo!();
    // anchor_position: Option<Vector2F>,
    // position_mode: OverlayPositionMode,
}

/// overlay gives you a floating element that will avoid overflowing the window bounds.
/// Its children should have no margin to avoid measurement issues.
pub fn overlay<V: 'static>() -> Overlay<V> {
    Overlay {
        children: SmallVec::new(),
        anchor_corner: AnchorCorner::TopLeft,
        fit_mode: OverlayFitMode::SwitchAnchor,
    }
}

impl<V> Overlay<V> {
    /// Sets which corner of the overlay should be anchored to the current position.
    pub fn anchor(mut self, anchor: AnchorCorner) -> Self {
        self.anchor_corner = anchor;
        self
    }

    /// Snap to window edge instead of switching anchor corner when an overflow would occur.
    pub fn snap_to_window(mut self) -> Self {
        self.fit_mode = OverlayFitMode::SnapToWindow;
        self
    }
}

impl<V: 'static> ParentComponent<V> for Overlay<V> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]> {
        &mut self.children
    }
}

impl<V: 'static> Component<V> for Overlay<V> {
    fn render(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V: 'static> Element<V> for Overlay<V> {
    type ElementState = OverlayState;

    fn element_id(&self) -> Option<crate::ElementId> {
        None
    }

    fn layout(
        &mut self,
        view_state: &mut V,
        _: Option<Self::ElementState>,
        cx: &mut crate::ViewContext<V>,
    ) -> (crate::LayoutId, Self::ElementState) {
        let child_layout_ids = self
            .children
            .iter_mut()
            .map(|child| child.layout(view_state, cx))
            .collect::<SmallVec<_>>();
        let layout_id = cx.request_layout(&Style::default(), child_layout_ids.iter().copied());

        (layout_id, OverlayState { child_layout_ids })
    }

    fn paint(
        &mut self,
        bounds: crate::Bounds<crate::Pixels>,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut crate::ViewContext<V>,
    ) {
        if element_state.child_layout_ids.is_empty() {
            return;
        }

        let mut child_min = point(Pixels::MAX, Pixels::MAX);
        let mut child_max = Point::default();
        for child_layout_id in &element_state.child_layout_ids {
            let child_bounds = cx.layout_bounds(*child_layout_id);
            child_min = child_min.min(&child_bounds.origin);
            child_max = child_max.max(&child_bounds.lower_right());
        }
        let size: Size<Pixels> = (child_max - child_min).into();
        let origin = bounds.origin;

        let mut desired = self.anchor_corner.get_bounds(origin, size);
        let limits = Bounds {
            origin: Point::zero(),
            size: cx.viewport_size(),
        };

        match self.fit_mode {
            OverlayFitMode::SnapToWindow => {
                // Snap the horizontal edges of the overlay to the horizontal edges of the window if
                // its horizontal bounds overflow
                if desired.right() > limits.right() {
                    desired.origin.x -= desired.right() - limits.right();
                } else if desired.left() < limits.left() {
                    desired.origin.x = limits.origin.x;
                }

                // Snap the vertical edges of the overlay to the vertical edges of the window if
                // its vertical bounds overflow.
                if desired.bottom() > limits.bottom() {
                    desired.origin.y -= desired.bottom() - limits.bottom();
                } else if desired.top() < limits.top() {
                    desired.origin.y = limits.origin.y;
                }
            }
            OverlayFitMode::SwitchAnchor => {
                let mut anchor_corner = self.anchor_corner;

                if desired.left() < limits.left() || desired.right() > limits.right() {
                    anchor_corner = anchor_corner.switch_axis(Axis::Horizontal);
                }

                if bounds.top() < limits.top() || bounds.bottom() > limits.bottom() {
                    anchor_corner = anchor_corner.switch_axis(Axis::Vertical);
                }

                // Update bounds if needed
                if anchor_corner != self.anchor_corner {
                    desired = anchor_corner.get_bounds(origin, size)
                }
            }
            OverlayFitMode::None => {}
        }

        cx.with_element_offset(desired.origin - bounds.origin, |cx| {
            for child in &mut self.children {
                child.paint(view_state, cx);
            }
        })
    }
}

enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone)]
pub enum OverlayFitMode {
    SnapToWindow,
    SwitchAnchor,
    None,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnchorCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl AnchorCorner {
    fn get_bounds(&self, origin: Point<Pixels>, size: Size<Pixels>) -> Bounds<Pixels> {
        let origin = match self {
            Self::TopLeft => origin,
            Self::TopRight => Point {
                x: origin.x - size.width,
                y: origin.y,
            },
            Self::BottomLeft => Point {
                x: origin.x,
                y: origin.y - size.height,
            },
            Self::BottomRight => Point {
                x: origin.x - size.width,
                y: origin.y - size.height,
            },
        };

        Bounds { origin, size }
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
