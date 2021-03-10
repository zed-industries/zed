use crate::{
    AfterLayoutContext, AppContext, Element, Event, EventContext, LayoutContext, MutableAppContext,
    PaintContext, SizeConstraint,
};
use pathfinder_geometry::vector::Vector2F;

pub struct ConstrainedBox {
    child: Box<dyn Element>,
    constraint: SizeConstraint,
}

impl ConstrainedBox {
    pub fn new(child: Box<dyn Element>) -> Self {
        Self {
            child,
            constraint: SizeConstraint {
                min: Vector2F::zero(),
                max: Vector2F::splat(f32::INFINITY),
            },
        }
    }

    pub fn with_max_width(mut self, max_width: f32) -> Self {
        self.constraint.max.set_x(max_width);
        self
    }

    pub fn with_max_height(mut self, max_height: f32) -> Self {
        self.constraint.max.set_y(max_height);
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.constraint.min.set_y(height);
        self.constraint.max.set_y(height);
        self
    }
}

impl Element for ConstrainedBox {
    fn layout(
        &mut self,
        mut constraint: SizeConstraint,
        ctx: &mut LayoutContext,
        app: &AppContext,
    ) -> Vector2F {
        constraint.min = constraint.min.max(self.constraint.min);
        constraint.max = constraint.max.min(self.constraint.max);
        self.child.layout(constraint, ctx, app)
    }

    fn after_layout(&mut self, ctx: &mut AfterLayoutContext, app: &mut MutableAppContext) {
        self.child.after_layout(ctx, app);
    }

    fn paint(&mut self, origin: Vector2F, ctx: &mut PaintContext, app: &AppContext) {
        self.child.paint(origin, ctx, app);
    }

    fn dispatch_event(&self, event: &Event, ctx: &mut EventContext, app: &AppContext) -> bool {
        self.child.dispatch_event(event, ctx, app)
    }

    fn size(&self) -> Option<Vector2F> {
        self.child.size()
    }
}
