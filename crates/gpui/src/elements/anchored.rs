use smallvec::SmallVec;

use crate::{
    Anchor, AnyElement, App, Axis, Bounds, Display, Edges, Element, GlobalElementId,
    InspectorElementId, IntoElement, LayoutId, ParentElement, Pixels, Point, Position, Size, Style,
    Window, point, px,
};

/// The state that the anchored element element uses to track its children.
pub struct AnchoredState {
    child_layout_ids: SmallVec<[LayoutId; 4]>,
}

/// An anchored element that can be used to display UI that
/// will avoid overflowing the window bounds.
pub struct Anchored {
    children: SmallVec<[AnyElement; 2]>,
    anchor: Anchor,
    fit_mode: AnchoredFitMode,
    anchor_position: Option<Point<Pixels>>,
    position_mode: AnchoredPositionMode,
    offset: Option<Point<Pixels>>,
}

/// anchored gives you an element that will avoid overflowing the window bounds.
/// Its children should have no margin to avoid measurement issues.
pub fn anchored() -> Anchored {
    Anchored {
        children: SmallVec::new(),
        anchor: Anchor::TopLeft,
        fit_mode: AnchoredFitMode::SwitchAnchor,
        anchor_position: None,
        position_mode: AnchoredPositionMode::Window,
        offset: None,
    }
}

impl Anchored {
    /// Sets which corner of the anchored element should be anchored to the current position.
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Sets the position in window coordinates
    /// (otherwise the location the anchored element is rendered is used)
    pub fn position(mut self, anchor: Point<Pixels>) -> Self {
        self.anchor_position = Some(anchor);
        self
    }

    /// Offset the final position by this amount.
    /// Useful when you want to anchor to an element but offset from it, such as in PopoverMenu.
    pub fn offset(mut self, offset: Point<Pixels>) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Sets the position mode for this anchored element. Local will have this
    /// interpret its [`Anchored::position`] as relative to the parent element.
    /// While Window will have it interpret the position as relative to the window.
    pub fn position_mode(mut self, mode: AnchoredPositionMode) -> Self {
        self.position_mode = mode;
        self
    }

    /// Snap to window edge instead of switching anchor corner when an overflow would occur.
    pub fn snap_to_window(mut self) -> Self {
        self.fit_mode = AnchoredFitMode::SnapToWindow;
        self
    }

    /// Snap to window edge and leave some margins.
    pub fn snap_to_window_with_margin(mut self, edges: impl Into<Edges<Pixels>>) -> Self {
        self.fit_mode = AnchoredFitMode::SnapToWindowWithMargin(edges.into());
        self
    }
}

impl ParentElement for Anchored {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl Element for Anchored {
    type RequestLayoutState = AnchoredState;
    type PrepaintState = ();

    fn id(&self) -> Option<crate::ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (crate::LayoutId, Self::RequestLayoutState) {
        let child_layout_ids = self
            .children
            .iter_mut()
            .map(|child| child.request_layout(window, cx))
            .collect::<SmallVec<_>>();

        let anchored_style = Style {
            position: Position::Absolute,
            display: Display::Flex,
            ..Style::default()
        };

        let layout_id = window.request_layout(anchored_style, child_layout_ids.iter().copied(), cx);

        (layout_id, AnchoredState { child_layout_ids })
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if request_layout.child_layout_ids.is_empty() {
            return;
        }

        let children_bounds = request_layout
            .child_layout_ids
            .iter()
            .map(|id| window.layout_bounds(*id))
            .reduce(|acc, bounds| acc.union(&bounds))
            .unwrap();

        let (origin, mut desired) = self.position_mode.get_position_and_bounds(
            self.anchor_position,
            self.anchor,
            children_bounds.size,
            bounds,
            self.offset,
        );

        let limits = Bounds {
            origin: Point::default(),
            size: window.viewport_size(),
        };

        if self.fit_mode == AnchoredFitMode::SwitchAnchor {
            let mut anchor = self.anchor;

            if desired.left() < limits.left() || desired.right() > limits.right() {
                let switched = Bounds::from_anchor_and_size(
                    anchor.other_side_along(Axis::Horizontal),
                    origin,
                    children_bounds.size,
                );
                if !(switched.left() < limits.left() || switched.right() > limits.right()) {
                    anchor = anchor.other_side_along(Axis::Horizontal);
                    desired = switched
                }
            }

            if desired.top() < limits.top() || desired.bottom() > limits.bottom() {
                let switched = Bounds::from_anchor_and_size(
                    anchor.other_side_along(Axis::Vertical),
                    origin,
                    children_bounds.size,
                );
                if !(switched.top() < limits.top() || switched.bottom() > limits.bottom()) {
                    desired = switched;
                }
            }
        }

        let client_inset = window.client_inset.unwrap_or(px(0.));
        let edges = match self.fit_mode {
            AnchoredFitMode::SnapToWindowWithMargin(edges) => edges,
            _ => Edges::default(),
        }
        .map(|edge| *edge + client_inset);

        // Snap the horizontal edges of the anchored element to the horizontal edges of the window if
        // its horizontal bounds overflow, aligning to the left if it is wider than the limits.
        if desired.right() > limits.right() {
            desired.origin.x -= desired.right() - limits.right() + edges.right;
        }
        if desired.left() < limits.left() {
            desired.origin.x = limits.origin.x + edges.left;
        }

        // Snap the vertical edges of the anchored element to the vertical edges of the window if
        // its vertical bounds overflow, aligning to the top if it is taller than the limits.
        if desired.bottom() > limits.bottom() {
            desired.origin.y -= desired.bottom() - limits.bottom() + edges.bottom;
        }
        if desired.top() < limits.top() {
            desired.origin.y = limits.origin.y + edges.top;
        }

        let offset = desired.origin - bounds.origin;
        let offset = point(offset.x.round(), offset.y.round());

        window.with_element_offset(offset, |window| {
            for child in &mut self.children {
                child.prepaint(window, cx);
            }
        })
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: crate::Bounds<crate::Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        for child in &mut self.children {
            child.paint(window, cx);
        }
    }
}

impl IntoElement for Anchored {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

/// Which algorithm to use when fitting the anchored element to be inside the window.
#[derive(Copy, Clone, PartialEq)]
pub enum AnchoredFitMode {
    /// Snap the anchored element to the window edge.
    SnapToWindow,
    /// Snap to window edge and leave some margins.
    SnapToWindowWithMargin(Edges<Pixels>),
    /// Switch which corner anchor this anchored element is attached to.
    SwitchAnchor,
}

/// Which algorithm to use when positioning the anchored element.
#[derive(Copy, Clone, PartialEq)]
pub enum AnchoredPositionMode {
    /// Position the anchored element relative to the window.
    Window,
    /// Position the anchored element relative to its parent.
    Local,
}

impl AnchoredPositionMode {
    fn get_position_and_bounds(
        &self,
        anchor_position: Option<Point<Pixels>>,
        anchor: Anchor,
        size: Size<Pixels>,
        bounds: Bounds<Pixels>,
        offset: Option<Point<Pixels>>,
    ) -> (Point<Pixels>, Bounds<Pixels>) {
        let offset = offset.unwrap_or_default();

        match self {
            AnchoredPositionMode::Window => {
                let anchor_position = anchor_position.unwrap_or(bounds.origin);
                let bounds = Bounds::from_anchor_and_size(anchor, anchor_position + offset, size);
                (anchor_position, bounds)
            }
            AnchoredPositionMode::Local => {
                let anchor_position = anchor_position.unwrap_or_default();
                let bounds = Bounds::from_anchor_and_size(
                    anchor,
                    bounds.origin + anchor_position + offset,
                    size,
                );
                (anchor_position, bounds)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Context, Pixels, PlatformInput, Point, TestAppContext, Window, deferred, div, point,
        prelude::*, px, size,
    };

    struct AnchoredTestView {
        position: Point<Pixels>,
    }

    impl Render for AnchoredTestView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().child(
                div()
                    .id("scroll-container")
                    .overflow_y_scroll()
                    .size_full()
                    .child(div().h(px(2000.)).w_full())
                    .child(
                        deferred(
                            super::anchored()
                                .snap_to_window()
                                .position(self.position)
                                .child(
                                    div()
                                        .id("menu")
                                        .debug_selector(|| "MENU".into())
                                        .w(px(200.))
                                        .h(px(300.)),
                                ),
                        )
                        .with_priority(1),
                    ),
            )
        }
    }

    #[gpui::test]
    fn test_anchored_position_without_scroll(cx: &mut TestAppContext) {
        let window = cx.open_window(size(px(800.), px(600.)), |_, _| AnchoredTestView {
            position: point(px(100.), px(100.)),
        });

        cx.run_until_parked();

        let menu_bounds = window
            .update(cx, |_, window, _| {
                window.rendered_frame.debug_bounds.get("MENU").copied()
            })
            .unwrap()
            .expect("MENU debug bounds not found");

        assert_eq!(menu_bounds.origin, point(px(100.), px(100.)));
        assert_eq!(menu_bounds.size, size(px(200.), px(300.)));
    }

    #[gpui::test]
    fn test_anchored_position_when_scrolled(cx: &mut TestAppContext) {
        let window = cx.open_window(size(px(800.), px(600.)), |_, _| AnchoredTestView {
            position: point(px(100.), px(100.)),
        });

        cx.run_until_parked();

        window
            .update(cx, |_, window, cx| {
                let event = gpui::ScrollWheelEvent {
                    position: point(px(400.), px(300.)),
                    delta: gpui::ScrollDelta::Pixels(point(px(0.), px(-1000.))),
                    ..Default::default()
                };
                window.dispatch_event(PlatformInput::ScrollWheel(event), cx);
            })
            .unwrap();

        cx.run_until_parked();

        let menu_bounds = window
            .update(cx, |_, window, _| {
                window.rendered_frame.debug_bounds.get("MENU").copied()
            })
            .unwrap()
            .expect("MENU debug bounds not found");

        assert_eq!(menu_bounds.origin, point(px(100.), px(100.)));
        assert_eq!(menu_bounds.size, size(px(200.), px(300.)));
    }

    #[gpui::test]
    fn test_anchored_snaps_to_window(cx: &mut TestAppContext) {
        let window = cx.open_window(size(px(800.), px(600.)), |_, _| AnchoredTestView {
            position: point(px(100.), px(500.)),
        });

        cx.run_until_parked();

        let menu_bounds = window
            .update(cx, |_, window, _| {
                window.rendered_frame.debug_bounds.get("MENU").copied()
            })
            .unwrap()
            .expect("MENU debug bounds not found");

        assert_eq!(menu_bounds.origin, point(px(100.), px(300.)));
        assert_eq!(menu_bounds.size, size(px(200.), px(300.)));
    }
}
