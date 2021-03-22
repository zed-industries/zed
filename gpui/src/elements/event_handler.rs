use crate::{
    geometry::vector::Vector2F, AfterLayoutContext, Element, ElementBox, Event, EventContext,
    LayoutContext, PaintContext, SizeConstraint,
};

pub struct EventHandler {
    child: ElementBox,
    mouse_down: Option<Box<dyn FnMut(&mut EventContext) -> bool>>,
}

impl EventHandler {
    pub fn new(child: ElementBox) -> Self {
        Self {
            child,
            mouse_down: None,
        }
    }

    pub fn on_mouse_down<F>(mut self, callback: F) -> Self
    where
        F: 'static + FnMut(&mut EventContext) -> bool,
    {
        self.mouse_down = Some(Box::new(callback));
        self
    }
}

impl Element for EventHandler {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let size = self.child.layout(constraint, ctx);
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
        self.child.paint(bounds.origin(), ctx);
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        bounds: pathfinder_geometry::rect::RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        ctx: &mut EventContext,
    ) -> bool {
        if self.child.dispatch_event(event, ctx) {
            true
        } else {
            match event {
                Event::LeftMouseDown { position, .. } => {
                    if let Some(callback) = self.mouse_down.as_mut() {
                        if bounds.contains_point(*position) {
                            return callback(ctx);
                        }
                    }
                    false
                }
                _ => false,
            }
        }
    }
}
