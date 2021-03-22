use crate::{
    AfterLayoutContext, Element, Event, EventContext, LayoutContext, PaintContext, SizeConstraint,
};
use pathfinder_geometry::{rect::RectF, vector::Vector2F};

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
        (constraint.max, ())
    }

    fn after_layout(&mut self, _: Vector2F, _: &mut Self::LayoutState, _: &mut AfterLayoutContext) {
    }

    fn paint(
        &mut self,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut PaintContext,
    ) -> Self::PaintState {
    }

    fn dispatch_event(
        &mut self,
        _: &Event,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        _: &mut EventContext,
    ) -> bool {
        false
    }
}
