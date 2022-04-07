use pathfinder_geometry::rect::RectF;
use serde_json::json;

use crate::{
    geometry::vector::Vector2F, DebugContext, Element, ElementBox, Event, EventContext,
    LayoutContext, NavigationDirection, PaintContext, SizeConstraint,
};

pub struct EventHandler {
    child: ElementBox,
    capture: Option<Box<dyn FnMut(&Event, RectF, &mut EventContext) -> bool>>,
    mouse_down: Option<Box<dyn FnMut(&mut EventContext) -> bool>>,
    right_mouse_down: Option<Box<dyn FnMut(&mut EventContext) -> bool>>,
    navigate_mouse_down: Option<Box<dyn FnMut(NavigationDirection, &mut EventContext) -> bool>>,
}

impl EventHandler {
    pub fn new(child: ElementBox) -> Self {
        Self {
            child,
            capture: None,
            mouse_down: None,
            right_mouse_down: None,
            navigate_mouse_down: None,
        }
    }

    pub fn on_mouse_down<F>(mut self, callback: F) -> Self
    where
        F: 'static + FnMut(&mut EventContext) -> bool,
    {
        self.mouse_down = Some(Box::new(callback));
        self
    }

    pub fn on_right_mouse_down<F>(mut self, callback: F) -> Self
    where
        F: 'static + FnMut(&mut EventContext) -> bool,
    {
        self.right_mouse_down = Some(Box::new(callback));
        self
    }

    pub fn on_navigate_mouse_down<F>(mut self, callback: F) -> Self
    where
        F: 'static + FnMut(NavigationDirection, &mut EventContext) -> bool,
    {
        self.navigate_mouse_down = Some(Box::new(callback));
        self
    }

    pub fn capture<F>(mut self, callback: F) -> Self
    where
        F: 'static + FnMut(&Event, RectF, &mut EventContext) -> bool,
    {
        self.capture = Some(Box::new(callback));
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
        _: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool {
        if let Some(capture) = self.capture.as_mut() {
            if capture(event, bounds, cx) {
                return true;
            }
        }

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
                Event::RightMouseDown { position, .. } => {
                    if let Some(callback) = self.right_mouse_down.as_mut() {
                        if bounds.contains_point(*position) {
                            return callback(cx);
                        }
                    }
                    false
                }
                Event::NavigateMouseDown {
                    position,
                    direction,
                    ..
                } => {
                    if let Some(callback) = self.navigate_mouse_down.as_mut() {
                        if bounds.contains_point(*position) {
                            return callback(*direction, cx);
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
