use crate::{
    geometry::vector::Vector2F, AfterLayoutContext, AppContext, Element, Event, EventContext,
    LayoutContext, MutableAppContext, PaintContext, SizeConstraint,
};

pub struct Stack {
    children: Vec<Box<dyn Element>>,
    size: Option<Vector2F>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            children: Vec::new(),
            size: None,
        }
    }
}

impl Element for Stack {
    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
        app: &AppContext,
    ) -> Vector2F {
        let mut size = constraint.min;
        for child in &mut self.children {
            size = size.max(child.layout(constraint, ctx, app));
        }
        self.size = Some(size);
        size
    }

    fn after_layout(&mut self, ctx: &mut AfterLayoutContext, app: &mut MutableAppContext) {
        for child in &mut self.children {
            child.after_layout(ctx, app);
        }
    }

    fn paint(&mut self, origin: Vector2F, ctx: &mut PaintContext, app: &AppContext) {
        for child in &mut self.children {
            child.paint(origin, ctx, app);
        }
    }

    fn dispatch_event(&self, event: &Event, ctx: &mut EventContext, app: &AppContext) -> bool {
        for child in self.children.iter().rev() {
            if child.dispatch_event(event, ctx, app) {
                return true;
            }
        }
        false
    }

    fn size(&self) -> Option<Vector2F> {
        self.size
    }
}

impl Extend<Box<dyn Element>> for Stack {
    fn extend<T: IntoIterator<Item = Box<dyn Element>>>(&mut self, children: T) {
        self.children.extend(children)
    }
}
