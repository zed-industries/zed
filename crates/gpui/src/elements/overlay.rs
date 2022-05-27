use serde_json::json;

use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json::ToJson,
    DebugContext, Element, ElementBox, Event, EventContext, LayoutContext, MouseRegion,
    PaintContext, SizeConstraint,
};

pub struct Overlay {
    child: ElementBox,
    abs_position: Option<Vector2F>,
}

impl Overlay {
    pub fn new(child: ElementBox) -> Self {
        Self {
            child,
            abs_position: None,
        }
    }

    pub fn with_abs_position(mut self, position: Vector2F) -> Self {
        self.abs_position = Some(position);
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
        let origin = self.abs_position.unwrap_or(bounds.origin());
        let visible_bounds = RectF::new(origin, *size);
        cx.scene.push_stacking_context(None);
        cx.scene.push_mouse_region(MouseRegion {
            view_id: cx.current_view_id(),
            bounds: visible_bounds,
            ..Default::default()
        });
        self.child.paint(origin, visible_bounds, cx);
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
