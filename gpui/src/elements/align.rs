use crate::{
    AfterLayoutContext, Element, ElementBox, Event, EventContext, LayoutContext, PaintContext,
    SizeConstraint,
};
use pathfinder_geometry::vector::{vec2f, Vector2F};

pub struct Align {
    child: ElementBox,
    alignment: Vector2F,
}

impl Align {
    pub fn new(child: ElementBox) -> Self {
        Self {
            child,
            alignment: Vector2F::zero(),
        }
    }

    pub fn top_center(mut self) -> Self {
        self.alignment = vec2f(0.0, -1.0);
        self
    }
}

impl Element for Align {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        mut constraint: SizeConstraint,
        ctx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let mut size = constraint.max;
        constraint.min = Vector2F::zero();
        let child_size = self.child.layout(constraint, ctx);
        if size.x().is_infinite() {
            size.set_x(child_size.x());
        }
        if size.y().is_infinite() {
            size.set_y(child_size.y());
        }
        (size, ())
    }

    fn after_layout(
        &mut self,
        _: Vector2F,
        _: &mut Self::LayoutState,
        ctx: &mut AfterLayoutContext,
    ) {
        self.child.after_layout(ctx);
    }

    fn paint(
        &mut self,
        bounds: pathfinder_geometry::rect::RectF,
        _: &mut Self::LayoutState,
        ctx: &mut PaintContext,
    ) -> Self::PaintState {
        let my_center = bounds.size() / 2.;
        let my_target = my_center + my_center * self.alignment;

        let child_center = self.child.size() / 2.;
        let child_target = child_center + child_center * self.alignment;

        self.child
            .paint(bounds.origin() - (child_target - my_target), ctx);
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: pathfinder_geometry::rect::RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        ctx: &mut EventContext,
    ) -> bool {
        self.child.dispatch_event(event, ctx)
    }
}
