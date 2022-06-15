use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json::ToJson,
    DebugContext, Element, ElementBox, Event, EventContext, LayoutContext, MouseRegion,
    PaintContext, SizeConstraint,
};
use serde_json::json;

pub struct Overlay {
    child: ElementBox,
    abs_position: Option<Vector2F>,
    fit_mode: OverlayFitMode,
    hoverable: bool,
}

#[derive(Copy, Clone)]
pub enum OverlayFitMode {
    SnapToWindow,
    FlipAlignment,
    None,
}

impl Overlay {
    pub fn new(child: ElementBox) -> Self {
        Self {
            child,
            abs_position: None,
            fit_mode: OverlayFitMode::None,
            hoverable: false,
        }
    }

    pub fn with_abs_position(mut self, position: Vector2F) -> Self {
        self.abs_position = Some(position);
        self
    }

    pub fn fit_mode(mut self, fit_mode: OverlayFitMode) -> Self {
        self.fit_mode = fit_mode;
        self
    }

    pub fn hoverable(mut self, hoverable: bool) -> Self {
        self.hoverable = hoverable;
        self
    }
}

impl Element for Overlay {
    type LayoutState = Vector2F;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let constraint = if self.abs_position.is_some() {
            SizeConstraint::new(Vector2F::zero(), cx.window_size)
        } else {
            constraint
        };
        let size = self.child.layout(constraint, cx);
        (Vector2F::zero(), size)
    }

    fn paint(
        &mut self,
        bounds: RectF,
        _: RectF,
        size: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) {
        let mut bounds = RectF::new(self.abs_position.unwrap_or(bounds.origin()), *size);
        cx.scene.push_stacking_context(None);

        if self.hoverable {
            cx.scene.push_mouse_region(MouseRegion {
                view_id: cx.current_view_id(),
                bounds,
                ..Default::default()
            });
        }

        match self.fit_mode {
            OverlayFitMode::SnapToWindow => {
                // Snap the right edge of the overlay to the right edge of the window if
                // its horizontal bounds overflow.
                if bounds.lower_right().x() > cx.window_size.x() {
                    bounds.set_origin_x((cx.window_size.x() - bounds.width()).max(0.));
                }

                // Snap the bottom edge of the overlay to the bottom edge of the window if
                // its vertical bounds overflow.
                if bounds.lower_right().y() > cx.window_size.y() {
                    bounds.set_origin_y((cx.window_size.y() - bounds.height()).max(0.));
                }
            }
            OverlayFitMode::FlipAlignment => {
                // Right-align overlay if its horizontal bounds overflow.
                if bounds.lower_right().x() > cx.window_size.x() {
                    bounds.set_origin_x(bounds.origin_x() - bounds.width());
                }

                // Bottom-align overlay if its vertical bounds overflow.
                if bounds.lower_right().y() > cx.window_size.y() {
                    bounds.set_origin_y(bounds.origin_y() - bounds.height());
                }
            }
            OverlayFitMode::None => {}
        }

        self.child.paint(bounds.origin(), bounds, cx);
        cx.scene.pop_stacking_context();
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: RectF,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool {
        self.child.dispatch_event(event, cx)
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &DebugContext,
    ) -> serde_json::Value {
        json!({
            "type": "Overlay",
            "abs_position": self.abs_position.to_json(),
            "child": self.child.debug(cx),
        })
    }
}
