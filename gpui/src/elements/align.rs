use crate::{
    AfterLayoutContext, AppContext, Element, Event, EventContext, LayoutContext, MutableAppContext,
    PaintContext, SizeConstraint,
};
use pathfinder_geometry::vector::{vec2f, Vector2F};

pub struct Align {
    child: Box<dyn Element>,
    alignment: Vector2F,
    size: Option<Vector2F>,
}

impl Align {
    pub fn new(child: Box<dyn Element>) -> Self {
        Self {
            child,
            alignment: Vector2F::zero(),
            size: None,
        }
    }

    pub fn top_center(mut self) -> Self {
        self.alignment = vec2f(0.0, -1.0);
        self
    }
}

impl Element for Align {
    fn layout(
        &mut self,
        mut constraint: SizeConstraint,
        ctx: &mut LayoutContext,
        app: &AppContext,
    ) -> Vector2F {
        let mut size = constraint.max;
        constraint.min = Vector2F::zero();
        let child_size = self.child.layout(constraint, ctx, app);
        if size.x().is_infinite() {
            size.set_x(child_size.x());
        }
        if size.y().is_infinite() {
            size.set_y(child_size.y());
        }
        self.size = Some(size);
        size
    }

    fn after_layout(&mut self, ctx: &mut AfterLayoutContext, app: &mut MutableAppContext) {
        self.child.after_layout(ctx, app);
    }

    fn paint(&mut self, origin: Vector2F, ctx: &mut PaintContext, app: &AppContext) {
        let self_center = self.size.unwrap() / 2.0;
        let self_target = self_center + self_center * self.alignment;
        let child_center = self.child.size().unwrap() / 2.0;
        let child_target = child_center + child_center * self.alignment;
        let origin = origin - (child_target - self_target);
        self.child.paint(origin, ctx, app);
    }

    fn dispatch_event(&self, event: &Event, ctx: &mut EventContext, app: &AppContext) -> bool {
        self.child.dispatch_event(event, ctx, app)
    }

    fn size(&self) -> Option<Vector2F> {
        self.size
    }
}
