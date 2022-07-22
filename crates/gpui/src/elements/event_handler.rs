use crate::{
    geometry::vector::Vector2F, presenter::MeasurementContext, CursorRegion, DebugContext, Element,
    ElementBox, Event, EventContext, LayoutContext, MouseButton, MouseButtonEvent, MouseRegion,
    NavigationDirection, PaintContext, SizeConstraint,
};
use pathfinder_geometry::rect::RectF;
use serde_json::json;
use std::{any::TypeId, ops::Range};

pub struct EventHandler {
    child: ElementBox,
    capture_all: Option<(TypeId, usize)>,
    mouse_down: Option<Box<dyn FnMut(&mut EventContext) -> bool>>,
    right_mouse_down: Option<Box<dyn FnMut(&mut EventContext) -> bool>>,
    navigate_mouse_down: Option<Box<dyn FnMut(NavigationDirection, &mut EventContext) -> bool>>,
}

impl EventHandler {
    pub fn new(child: ElementBox) -> Self {
        Self {
            child,
            capture_all: None,
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

    pub fn capture_all<T: 'static>(mut self, id: usize) -> Self {
        self.capture_all = Some((TypeId::of::<T>(), id));
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
        if let Some(discriminant) = self.capture_all {
            cx.scene.push_stacking_context(None);
            cx.scene.push_cursor_region(CursorRegion {
                bounds: visible_bounds,
                style: Default::default(),
            });
            cx.scene.push_mouse_region(MouseRegion::handle_all(
                cx.current_view_id(),
                Some(discriminant),
                visible_bounds,
            ));
            cx.scene.pop_stacking_context();
        }
        self.child.paint(bounds.origin(), visible_bounds, cx);
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool {
        if self.capture_all.is_some() {
            return true;
        }

        if self.child.dispatch_event(event, cx) {
            true
        } else {
            match event {
                Event::MouseDown(MouseButtonEvent {
                    button: MouseButton::Left,
                    position,
                    ..
                }) => {
                    if let Some(callback) = self.mouse_down.as_mut() {
                        if visible_bounds.contains_point(*position) {
                            return callback(cx);
                        }
                    }
                    false
                }
                Event::MouseDown(MouseButtonEvent {
                    button: MouseButton::Right,
                    position,
                    ..
                }) => {
                    if let Some(callback) = self.right_mouse_down.as_mut() {
                        if visible_bounds.contains_point(*position) {
                            return callback(cx);
                        }
                    }
                    false
                }
                Event::MouseDown(MouseButtonEvent {
                    button: MouseButton::Navigate(direction),
                    position,
                    ..
                }) => {
                    if let Some(callback) = self.navigate_mouse_down.as_mut() {
                        if visible_bounds.contains_point(*position) {
                            return callback(*direction, cx);
                        }
                    }
                    false
                }
                _ => false,
            }
        }
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &MeasurementContext,
    ) -> Option<RectF> {
        self.child.rect_for_text_range(range_utf16, cx)
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
