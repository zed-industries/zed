use super::Element;
use crate::{
    json::{self, json},
    DebugContext, PaintContext,
};
use json::ToJson;
use pathfinder_geometry::{
    rect::RectF,
    vector::{vec2f, Vector2F},
};

pub struct Canvas<F>(F)
where
    F: FnMut(RectF, &mut PaintContext);

impl<F> Canvas<F>
where
    F: FnMut(RectF, &mut PaintContext),
{
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

impl<F> Element for Canvas<F>
where
    F: FnMut(RectF, &mut PaintContext),
{
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: crate::SizeConstraint,
        _: &mut crate::LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let x = if constraint.max.x().is_finite() {
            constraint.max.x()
        } else {
            constraint.min.x()
        };
        let y = if constraint.max.y().is_finite() {
            constraint.max.y()
        } else {
            constraint.min.y()
        };
        (vec2f(x, y), ())
    }

    fn paint(
        &mut self,
        bounds: RectF,
        _: &mut Self::LayoutState,
        ctx: &mut PaintContext,
    ) -> Self::PaintState {
        self.0(bounds, ctx)
    }

    fn after_layout(
        &mut self,
        _: Vector2F,
        _: &mut Self::LayoutState,
        _: &mut crate::AfterLayoutContext,
    ) {
    }

    fn dispatch_event(
        &mut self,
        _: &crate::Event,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        _: &mut crate::EventContext,
    ) -> bool {
        false
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &DebugContext,
    ) -> json::Value {
        json!({"type": "Canvas", "bounds": bounds.to_json()})
    }
}
