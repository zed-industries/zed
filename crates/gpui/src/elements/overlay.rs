use smallvec::SmallVec;
use taffy::style::{Display, Position};

use crate::{
    point, AnyElement, Bounds, Element, ElementContext, IntoElement, LayoutId, ParentElement,
    Pixels, Point, Size, Style,
};

/// The state that the overlay element uses to track its children.
pub struct OverlayState {
    child_layout_ids: SmallVec<[LayoutId; 4]>,
    offset: Point<Pixels>,
}

/// An overlay element that can be used to display UI that
/// floats on top of other UI elements.
pub struct Overlay {
    children: SmallVec<[AnyElement; 2]>,
    anchor_corner: AnchorCorner,
    fit_mode: OverlayFitMode,
    anchor_position: Option<Point<Pixels>>,
    position_mode: OverlayPositionMode,
}

/// overlay gives you a floating element that will avoid overflowing the window bounds.
/// Its children should have no margin to avoid measurement issues.
pub fn overlay() -> Overlay {
    Overlay {
        children: SmallVec::new(),
        anchor_corner: AnchorCorner::TopLeft,
        fit_mode: OverlayFitMode::SwitchAnchor,
        anchor_position: None,
        position_mode: OverlayPositionMode::Window,
    }
}

impl Overlay {
    /// Sets which corner of the overlay should be anchored to the current position.
    pub fn anchor(mut self, anchor: AnchorCorner) -> Self {
        self.anchor_corner = anchor;
        self
    }

    /// Sets the position in window coordinates
    /// (otherwise the location the overlay is rendered is used)
    pub fn position(mut self, anchor: Point<Pixels>) -> Self {
        self.anchor_position = Some(anchor);
        self
    }

    /// Sets the position mode for this overlay. Local will have this
    /// interpret its [`Overlay::position`] as relative to the parent element.
    /// While Window will have it interpret the position as relative to the window.
    pub fn position_mode(mut self, mode: OverlayPositionMode) -> Self {
        self.position_mode = mode;
        self
    }

    /// Snap to window edge instead of switching anchor corner when an overflow would occur.
    pub fn snap_to_window(mut self) -> Self {
        self.fit_mode = OverlayFitMode::SnapToWindow;
        self
    }
}

impl ParentElement for Overlay {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl Element for Overlay {
    type BeforeLayout = OverlayState;
    type AfterLayout = ();

    fn before_layout(&mut self, cx: &mut ElementContext) -> (crate::LayoutId, Self::BeforeLayout) {
        let child_layout_ids = self
            .children
            .iter_mut()
            .map(|child| child.before_layout(cx))
            .collect::<SmallVec<_>>();

        let overlay_style = Style {
            position: Position::Absolute,
            display: Display::Flex,
            ..Style::default()
        };

        let layout_id = cx.request_layout(&overlay_style, child_layout_ids.iter().copied());

        (
            layout_id,
            OverlayState {
                child_layout_ids,
                offset: Point::default(),
            },
        )
    }

    fn after_layout(
        &mut self,
        bounds: Bounds<Pixels>,
        before_layout: &mut Self::BeforeLayout,
        cx: &mut ElementContext,
    ) {
        if before_layout.child_layout_ids.is_empty() {
            return;
        }

        let mut child_min = point(Pixels::MAX, Pixels::MAX);
        let mut child_max = Point::default();
        for child_layout_id in &before_layout.child_layout_ids {
            let child_bounds = cx.layout_bounds(*child_layout_id);
            child_min = child_min.min(&child_bounds.origin);
            child_max = child_max.max(&child_bounds.lower_right());
        }
        let size: Size<Pixels> = (child_max - child_min).into();

        let (origin, mut desired) = self.position_mode.get_position_and_bounds(
            self.anchor_position,
            self.anchor_corner,
            size,
            bounds,
        );

        let limits = Bounds {
            origin: Point::default(),
            size: cx.viewport_size(),
        };

        if self.fit_mode == OverlayFitMode::SwitchAnchor {
            let mut anchor_corner = self.anchor_corner;

            if desired.left() < limits.left() || desired.right() > limits.right() {
                let switched = anchor_corner
                    .switch_axis(Axis::Horizontal)
                    .get_bounds(origin, size);
                if !(switched.left() < limits.left() || switched.right() > limits.right()) {
                    anchor_corner = anchor_corner.switch_axis(Axis::Horizontal);
                    desired = switched
                }
            }

            if desired.top() < limits.top() || desired.bottom() > limits.bottom() {
                let switched = anchor_corner
                    .switch_axis(Axis::Vertical)
                    .get_bounds(origin, size);
                if !(switched.top() < limits.top() || switched.bottom() > limits.bottom()) {
                    desired = switched;
                }
            }
        }

        // Snap the horizontal edges of the overlay to the horizontal edges of the window if
        // its horizontal bounds overflow, aligning to the left if it is wider than the limits.
        if desired.right() > limits.right() {
            desired.origin.x -= desired.right() - limits.right();
        }
        if desired.left() < limits.left() {
            desired.origin.x = limits.origin.x;
        }

        // Snap the vertical edges of the overlay to the vertical edges of the window if
        // its vertical bounds overflow, aligning to the top if it is taller than the limits.
        if desired.bottom() > limits.bottom() {
            desired.origin.y -= desired.bottom() - limits.bottom();
        }
        if desired.top() < limits.top() {
            desired.origin.y = limits.origin.y;
        }

        before_layout.offset = cx.element_offset() + desired.origin - bounds.origin;
        before_layout.offset = point(
            before_layout.offset.x.round(),
            before_layout.offset.y.round(),
        );

        for child in self.children.drain(..) {
            cx.defer_draw(child, before_layout.offset, 1);
        }
    }

    fn paint(
        &mut self,
        _bounds: crate::Bounds<crate::Pixels>,
        _before_layout: &mut Self::BeforeLayout,
        _after_layout: &mut Self::AfterLayout,
        _cx: &mut ElementContext,
    ) {
    }
}

impl IntoElement for Overlay {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

enum Axis {
    Horizontal,
    Vertical,
}

/// Which algorithm to use when fitting the overlay to be inside the window.
#[derive(Copy, Clone, PartialEq)]
pub enum OverlayFitMode {
    /// Snap the overlay to the window edge
    SnapToWindow,
    /// Switch which corner anchor this overlay is attached to
    SwitchAnchor,
}

/// Which algorithm to use when positioning the overlay.
#[derive(Copy, Clone, PartialEq)]
pub enum OverlayPositionMode {
    /// Position the overlay relative to the window
    Window,
    /// Position the overlay relative to its parent
    Local,
}

impl OverlayPositionMode {
    fn get_position_and_bounds(
        &self,
        anchor_position: Option<Point<Pixels>>,
        anchor_corner: AnchorCorner,
        size: Size<Pixels>,
        bounds: Bounds<Pixels>,
    ) -> (Point<Pixels>, Bounds<Pixels>) {
        match self {
            OverlayPositionMode::Window => {
                let anchor_position = anchor_position.unwrap_or(bounds.origin);
                let bounds = anchor_corner.get_bounds(anchor_position, size);
                (anchor_position, bounds)
            }
            OverlayPositionMode::Local => {
                let anchor_position = anchor_position.unwrap_or_default();
                let bounds = anchor_corner.get_bounds(bounds.origin + anchor_position, size);
                (anchor_position, bounds)
            }
        }
    }
}

/// Which corner of the overlay should be considered the anchor.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnchorCorner {
    /// The top left corner
    TopLeft,
    /// The top right corner
    TopRight,
    /// The bottom left corner
    BottomLeft,
    /// The bottom right corner
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

    /// Get the point corresponding to this anchor corner in `bounds`.
    pub fn corner(&self, bounds: Bounds<Pixels>) -> Point<Pixels> {
        match self {
            Self::TopLeft => bounds.origin,
            Self::TopRight => bounds.upper_right(),
            Self::BottomLeft => bounds.lower_left(),
            Self::BottomRight => bounds.lower_right(),
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
