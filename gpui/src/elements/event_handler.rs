use super::try_rect;
use crate::{
    geometry::vector::Vector2F, AfterLayoutContext, AppContext, Element, Event, EventContext,
    LayoutContext, MutableAppContext, PaintContext, SizeConstraint,
};
use std::cell::RefCell;

pub struct EventHandler {
    child: Box<dyn Element>,
    mouse_down: Option<RefCell<Box<dyn FnMut(&mut EventContext, &AppContext) -> bool>>>,
    origin: Option<Vector2F>,
}

impl EventHandler {
    pub fn new(child: Box<dyn Element>) -> Self {
        Self {
            child,
            mouse_down: None,
            origin: None,
        }
    }

    pub fn on_mouse_down<F>(mut self, callback: F) -> Self
    where
        F: 'static + FnMut(&mut EventContext, &AppContext) -> bool,
    {
        self.mouse_down = Some(RefCell::new(Box::new(callback)));
        self
    }
}

impl Element for EventHandler {
    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
        app: &AppContext,
    ) -> Vector2F {
        self.child.layout(constraint, ctx, app)
    }

    fn after_layout(&mut self, ctx: &mut AfterLayoutContext, app: &mut MutableAppContext) {
        self.child.after_layout(ctx, app);
    }

    fn paint(&mut self, origin: Vector2F, ctx: &mut PaintContext, app: &AppContext) {
        self.origin = Some(origin);
        self.child.paint(origin, ctx, app);
    }

    fn size(&self) -> Option<Vector2F> {
        self.child.size()
    }

    fn dispatch_event(&self, event: &Event, ctx: &mut EventContext, app: &AppContext) -> bool {
        match event {
            Event::LeftMouseDown { position, .. } => {
                if let Some(callback) = self.mouse_down.as_ref() {
                    let rect = try_rect(self.origin, self.size()).unwrap();
                    if rect.contains_point(*position) {
                        return callback.borrow_mut()(ctx, app);
                    }
                }
                false
            }
            _ => false,
        }
    }
}
