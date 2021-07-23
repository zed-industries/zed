use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    AfterLayoutContext, AppContext, DebugContext, Element, ElementBox, Event, EventContext,
    LayoutContext, PaintContext, SizeConstraint, ValueHandle,
};
use serde_json::json;

pub struct MouseEventHandler {
    state: ValueHandle<MouseState>,
    child: ElementBox,
    click_handler: Option<Box<dyn FnMut(&mut EventContext)>>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MouseState {
    pub hovered: bool,
    pub clicked: bool,
}

impl MouseEventHandler {
    pub fn new<Tag, F>(id: usize, cx: &AppContext, render_child: F) -> Self
    where
        Tag: 'static,
        F: FnOnce(MouseState) -> ElementBox,
    {
        let state_handle = cx.value::<Tag, _>(id);
        let state = state_handle.read(cx.as_ref(), |state| *state);
        let child = render_child(state);
        Self {
            state: state_handle,
            child,
            click_handler: None,
        }
    }

    pub fn on_click(mut self, handler: impl FnMut(&mut EventContext) + 'static) -> Self {
        self.click_handler = Some(Box::new(handler));
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

    fn after_layout(
        &mut self,
        _: Vector2F,
        _: &mut Self::LayoutState,
        cx: &mut AfterLayoutContext,
    ) {
        self.child.after_layout(cx);
    }

    fn paint(
        &mut self,
        bounds: RectF,
        _: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        self.child.paint(bounds.origin(), cx);
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        bounds: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool {
        let click_handler = self.click_handler.as_mut();

        let handled_in_child = self.child.dispatch_event(event, cx);

        self.state.update(cx, |state, cx| match event {
            Event::MouseMoved { position } => {
                let mouse_in = bounds.contains_point(*position);
                if state.hovered != mouse_in {
                    state.hovered = mouse_in;
                    cx.notify();
                    true
                } else {
                    handled_in_child
                }
            }
            Event::LeftMouseDown { position, .. } => {
                if !handled_in_child && bounds.contains_point(*position) {
                    state.clicked = true;
                    cx.notify();
                    true
                } else {
                    handled_in_child
                }
            }
            Event::LeftMouseUp { position, .. } => {
                if !handled_in_child && state.clicked {
                    state.clicked = false;
                    cx.notify();
                    if let Some(handler) = click_handler {
                        if bounds.contains_point(*position) {
                            handler(cx);
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
