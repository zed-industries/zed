use super::Padding;
use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    platform::CursorStyle,
    CursorStyleHandle, DebugContext, Element, ElementBox, ElementStateContext, ElementStateHandle,
    Event, EventContext, LayoutContext, PaintContext, SizeConstraint,
};
use serde_json::json;

pub struct MouseEventHandler {
    state: ElementStateHandle<MouseState>,
    child: ElementBox,
    cursor_style: Option<CursorStyle>,
    mouse_down_handler: Option<Box<dyn FnMut(&mut EventContext)>>,
    click_handler: Option<Box<dyn FnMut(&mut EventContext)>>,
    drag_handler: Option<Box<dyn FnMut(Vector2F, &mut EventContext)>>,
    padding: Padding,
}

#[derive(Default)]
pub struct MouseState {
    pub hovered: bool,
    pub clicked: bool,
    prev_drag_position: Option<Vector2F>,
    cursor_style_handle: Option<CursorStyleHandle>,
}

impl MouseEventHandler {
    pub fn new<Tag, C, F>(id: usize, cx: &mut C, render_child: F) -> Self
    where
        Tag: 'static,
        C: ElementStateContext,
        F: FnOnce(&MouseState, &mut C) -> ElementBox,
    {
        let state_handle = cx.element_state::<Tag, _>(id);
        let child = state_handle.update(cx, |state, cx| render_child(state, cx));
        Self {
            state: state_handle,
            child,
            cursor_style: None,
            mouse_down_handler: None,
            click_handler: None,
            drag_handler: None,
            padding: Default::default(),
        }
    }

    pub fn with_cursor_style(mut self, cursor: CursorStyle) -> Self {
        self.cursor_style = Some(cursor);
        self
    }

    pub fn on_mouse_down(mut self, handler: impl FnMut(&mut EventContext) + 'static) -> Self {
        self.mouse_down_handler = Some(Box::new(handler));
        self
    }

    pub fn on_click(mut self, handler: impl FnMut(&mut EventContext) + 'static) -> Self {
        self.click_handler = Some(Box::new(handler));
        self
    }

    pub fn on_drag(mut self, handler: impl FnMut(Vector2F, &mut EventContext) + 'static) -> Self {
        self.drag_handler = Some(Box::new(handler));
        self
    }

    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }
}

impl Element for MouseEventHandler {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        (self.child.layout(constraint, cx), ())
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
        let cursor_style = self.cursor_style;
        let mouse_down_handler = self.mouse_down_handler.as_mut();
        let click_handler = self.click_handler.as_mut();
        let drag_handler = self.drag_handler.as_mut();

        let handled_in_child = self.child.dispatch_event(event, cx);

        let hit_bounds = RectF::from_points(
            bounds.origin() - vec2f(self.padding.left, self.padding.top),
            bounds.lower_right() + vec2f(self.padding.right, self.padding.bottom),
        )
        .round_out();

        self.state.update(cx, |state, cx| match event {
            Event::MouseMoved {
                position,
                left_mouse_down,
            } => {
                if !left_mouse_down {
                    let mouse_in = hit_bounds.contains_point(*position);
                    if state.hovered != mouse_in {
                        state.hovered = mouse_in;
                        if let Some(cursor_style) = cursor_style {
                            if !state.clicked {
                                if state.hovered {
                                    state.cursor_style_handle =
                                        Some(cx.set_cursor_style(cursor_style));
                                } else {
                                    state.cursor_style_handle = None;
                                }
                            }
                        }
                        cx.notify();
                        return true;
                    }
                }
                handled_in_child
            }
            Event::LeftMouseDown { position, .. } => {
                if !handled_in_child && hit_bounds.contains_point(*position) {
                    state.clicked = true;
                    state.prev_drag_position = Some(*position);
                    cx.notify();
                    if let Some(handler) = mouse_down_handler {
                        handler(cx);
                    }
                    true
                } else {
                    handled_in_child
                }
            }
            Event::LeftMouseUp { position, .. } => {
                state.prev_drag_position = None;
                if !handled_in_child && state.clicked {
                    state.clicked = false;
                    if !state.hovered {
                        state.cursor_style_handle = None;
                    }
                    cx.notify();
                    if let Some(handler) = click_handler {
                        if hit_bounds.contains_point(*position) {
                            handler(cx);
                        }
                    }
                    true
                } else {
                    handled_in_child
                }
            }
            Event::LeftMouseDragged { position, .. } => {
                if !handled_in_child && state.clicked {
                    let prev_drag_position = state.prev_drag_position.replace(*position);
                    if let Some((handler, prev_position)) = drag_handler.zip(prev_drag_position) {
                        let delta = *position - prev_position;
                        if !delta.is_zero() {
                            (handler)(delta, cx);
                        }
                    }
                    true
                } else {
                    handled_in_child
                }
            }
            _ => handled_in_child,
        })
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &DebugContext,
    ) -> serde_json::Value {
        json!({
            "type": "MouseEventHandler",
            "child": self.child.debug(cx),
        })
    }
}
