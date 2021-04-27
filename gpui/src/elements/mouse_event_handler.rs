use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    AfterLayoutContext, AppContext, DebugContext, Element, ElementBox, Event, EventContext,
    LayoutContext, PaintContext, SizeConstraint, ValueHandle,
};
use serde_json::json;

pub struct MouseEventHandler {
    state: ValueHandle<MouseState>,
    child: ElementBox,
}

#[derive(Clone, Copy, Default)]
pub struct MouseState {
    hovered: bool,
    clicked: bool,
}

impl MouseEventHandler {
    pub fn new<Tag: 'static>(
        id: usize,
        ctx: &AppContext,
        render_child: impl FnOnce(MouseState) -> ElementBox,
    ) -> Self {
        let state = ctx.value::<Tag, _>(id);
        let child = state.map(ctx, |state| render_child(*state));
        Self { state, child }
    }
}

impl Element for MouseEventHandler {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        (self.child.layout(constraint, ctx), ())
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
        bounds: RectF,
        _: &mut Self::LayoutState,
        ctx: &mut PaintContext,
    ) -> Self::PaintState {
        self.child.paint(bounds.origin(), ctx);
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        bounds: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        ctx: &mut EventContext,
    ) -> bool {
        self.state.map(ctx.app, |state| match event {
            Event::MouseMoved { position } => {
                let mouse_in = bounds.contains_point(*position);
                if state.hovered != mouse_in {
                    state.hovered = mouse_in;
                    log::info!("hovered {}", state.hovered);
                    // ctx.notify();
                    true
                } else {
                    false
                }
            }
            Event::LeftMouseDown { position, .. } => {
                if bounds.contains_point(*position) {
                    log::info!("clicked");
                    state.clicked = true;
                    // ctx.notify();
                    true
                } else {
                    false
                }
            }
            Event::LeftMouseUp { .. } => {
                if state.clicked {
                    log::info!("unclicked");
                    state.clicked = false;
                    // ctx.notify();
                    true
                } else {
                    false
                }
            }
            _ => false,
        })
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        ctx: &DebugContext,
    ) -> serde_json::Value {
        json!({
            "type": "MouseEventHandler",
            "child": self.child.debug(ctx),
        })
    }
}
