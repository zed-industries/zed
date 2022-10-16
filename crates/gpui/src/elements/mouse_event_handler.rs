use super::Padding;
use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    platform::CursorStyle,
    scene::{
        CursorRegion, HandlerSet, MouseClick, MouseDown, MouseDownOut, MouseDrag, MouseHover,
        MouseMove, MouseScrollWheel, MouseUp, MouseUpOut,
    },
    DebugContext, Element, ElementBox, Event, EventContext, LayoutContext, MeasurementContext,
    MouseButton, MouseRegion, MouseState, PaintContext, RenderContext, SizeConstraint, View,
};
use serde_json::json;
use std::{marker::PhantomData, ops::Range};

pub struct MouseEventHandler<Tag: 'static> {
    child: ElementBox,
    region_id: usize,
    cursor_style: Option<CursorStyle>,
    handlers: HandlerSet,
    hoverable: bool,
    notify_on_hover: bool,
    notify_on_click: bool,
    padding: Padding,
    _tag: PhantomData<Tag>,
}

impl<Tag> MouseEventHandler<Tag> {
    pub fn new<V, F>(region_id: usize, cx: &mut RenderContext<V>, render_child: F) -> Self
    where
        V: View,
        F: FnOnce(&mut MouseState, &mut RenderContext<V>) -> ElementBox,
    {
        let mut mouse_state = cx.mouse_state::<Tag>(region_id);
        let child = render_child(&mut mouse_state, cx);
        let notify_on_hover = mouse_state.accessed_hovered();
        let notify_on_click = mouse_state.accessed_clicked();
        Self {
            child,
            region_id,
            cursor_style: None,
            handlers: Default::default(),
            notify_on_hover,
            notify_on_click,
            hoverable: true,
            padding: Default::default(),
            _tag: PhantomData,
        }
    }

    pub fn with_cursor_style(mut self, cursor: CursorStyle) -> Self {
        self.cursor_style = Some(cursor);
        self
    }

    pub fn capture_all(mut self) -> Self {
        self.handlers = HandlerSet::capture_all();
        self
    }

    pub fn on_move(mut self, handler: impl Fn(MouseMove, &mut EventContext) + 'static) -> Self {
        self.handlers = self.handlers.on_move(handler);
        self
    }

    pub fn on_down(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseDown, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_down(button, handler);
        self
    }

    pub fn on_up(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseUp, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_up(button, handler);
        self
    }

    pub fn on_click(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseClick, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_click(button, handler);
        self
    }

    pub fn on_down_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseDownOut, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_down_out(button, handler);
        self
    }

    pub fn on_up_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseUpOut, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_up_out(button, handler);
        self
    }

    pub fn on_drag(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseDrag, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_drag(button, handler);
        self
    }

    pub fn on_hover(mut self, handler: impl Fn(MouseHover, &mut EventContext) + 'static) -> Self {
        self.handlers = self.handlers.on_hover(handler);
        self
    }

    pub fn on_scroll(
        mut self,
        handler: impl Fn(MouseScrollWheel, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_scroll(handler);
        self
    }

    pub fn with_hoverable(mut self, is_hoverable: bool) -> Self {
        self.hoverable = is_hoverable;
        self
    }

    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    fn hit_bounds(&self, bounds: RectF) -> RectF {
        RectF::from_points(
            bounds.origin() - vec2f(self.padding.left, self.padding.top),
            bounds.lower_right() + vec2f(self.padding.right, self.padding.bottom),
        )
        .round_out()
    }
}

impl<Tag> Element for MouseEventHandler<Tag> {
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
        let visible_bounds = visible_bounds.intersection(bounds).unwrap_or_default();
        let hit_bounds = self.hit_bounds(visible_bounds);
        if let Some(style) = self.cursor_style {
            cx.scene.push_cursor_region(CursorRegion {
                bounds: hit_bounds,
                style,
            });
        }

        cx.scene.push_mouse_region(
            MouseRegion::from_handlers::<Tag>(
                cx.current_view_id(),
                self.region_id,
                hit_bounds,
                self.handlers.clone(),
            )
            .with_hoverable(self.hoverable)
            .with_notify_on_hover(self.notify_on_hover)
            .with_notify_on_click(self.notify_on_click),
        );

        self.child.paint(bounds.origin(), visible_bounds, cx);
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: RectF,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool {
        self.child.dispatch_event(event, cx)
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
            "type": "MouseEventHandler",
            "child": self.child.debug(cx),
        })
    }
}
