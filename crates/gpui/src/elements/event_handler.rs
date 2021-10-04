use pathfinder_geometry::rect::RectF;
use serde_json::json;

use crate::{
    geometry::vector::Vector2F, DebugContext, Element, ElementBox, Event, EventContext,
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
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let size = self.child.layout(constraint, cx);
        (size, ())
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        self.child.paint(bounds.origin(), visible_bounds, cx);
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        bounds: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool {
        if self.child.dispatch_event(event, cx) {
            true
        } else {
            match event {
                Event::LeftMouseDown { position, .. } => {
                    if let Some(callback) = self.mouse_down.as_mut() {
                        if bounds.contains_point(*position) {
                            return callback(cx);
                        }
                    }
                    false
                }
                _ => false,
            }
        }
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &DebugContext,
    ) -> serde_json::Value {
        json!({
            "type": "EventHandler",
            "child": self.child.debug(cx),
        })
    }
}
