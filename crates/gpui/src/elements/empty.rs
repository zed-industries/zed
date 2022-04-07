use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{json, ToJson},
    DebugContext,
};
use crate::{Element, Event, EventContext, LayoutContext, PaintContext, SizeConstraint};

pub struct Empty;

impl Empty {
    pub fn new() -> Self {
        Self
    }
}

impl Element for Empty {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        _: &mut LayoutContext,
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
        _: RectF,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut PaintContext,
    ) -> Self::PaintState {
    }

    fn dispatch_event(
        &mut self,
        _: &Event,
        _: RectF,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        _: &mut EventContext,
    ) -> bool {
        false
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &DebugContext,
    ) -> serde_json::Value {
        json!({
            "type": "Empty",
            "bounds": bounds.to_json(),
        })
    }
}
