use crate::{
    AfterLayoutContext, AppContext, Element, Event, EventContext, LayoutContext, MutableAppContext,
    PaintContext, SizeConstraint,
};
use pathfinder_geometry::vector::Vector2F;

pub struct Empty {
    size: Option<Vector2F>,
    origin: Option<Vector2F>,
}

impl Empty {
    pub fn new() -> Self {
        Self {
            size: None,
            origin: None,
        }
    }
}

impl Element for Empty {
    fn layout(
        &mut self,
        constraint: SizeConstraint,
        _: &mut LayoutContext,
        _: &AppContext,
    ) -> Vector2F {
        self.size = Some(constraint.max);
        constraint.max
    }

    fn after_layout(&mut self, _: &mut AfterLayoutContext, _: &mut MutableAppContext) {}

    fn paint(&mut self, origin: Vector2F, _: &mut PaintContext, _: &AppContext) {
        self.origin = Some(origin);
    }

    fn dispatch_event(&self, _: &Event, _: &mut EventContext, _: &AppContext) -> bool {
        false
    }

    fn size(&self) -> Option<Vector2F> {
        self.size
    }
}
