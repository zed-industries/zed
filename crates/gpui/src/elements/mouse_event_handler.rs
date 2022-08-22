use super::Padding;
use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    platform::CursorStyle,
    scene::{
        ClickRegionEvent, CursorRegion, DownOutRegionEvent, DownRegionEvent, DragRegionEvent,
        HandlerSet, HoverRegionEvent, MoveRegionEvent, UpOutRegionEvent, UpRegionEvent,
    },
    DebugContext, Element, ElementBox, Event, EventContext, LayoutContext, MeasurementContext,
    MouseButton, MouseRegion, MouseState, PaintContext, RenderContext, SizeConstraint, View,
};
use serde_json::json;
use std::{any::TypeId, ops::Range};

pub struct MouseEventHandler {
    child: ElementBox,
    discriminant: (TypeId, usize),
    cursor_style: Option<CursorStyle>,
    handlers: HandlerSet,
    padding: Padding,
}

impl MouseEventHandler {
    pub fn new<Tag, V, F>(id: usize, cx: &mut RenderContext<V>, render_child: F) -> Self
    where
        Tag: 'static,
        V: View,
        F: FnOnce(MouseState, &mut RenderContext<V>) -> ElementBox,
    {
        Self {
            child: render_child(cx.mouse_state::<Tag>(id), cx),
            cursor_style: None,
            discriminant: (TypeId::of::<Tag>(), id),
            handlers: Default::default(),
            padding: Default::default(),
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

    pub fn on_move(
        mut self,
        handler: impl Fn(MoveRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_move(handler);
        self
    }

    pub fn on_down(
        mut self,
        button: MouseButton,
        handler: impl Fn(DownRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_down(button, handler);
        self
    }

    pub fn on_up(
        mut self,
        button: MouseButton,
        handler: impl Fn(UpRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_up(button, handler);
        self
    }

    pub fn on_click(
        mut self,
        button: MouseButton,
        handler: impl Fn(ClickRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_click(button, handler);
        self
    }

    pub fn on_down_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(DownOutRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_down_out(button, handler);
        self
    }

    pub fn on_up_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(UpOutRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_up_out(button, handler);
        self
    }

    pub fn on_drag(
        mut self,
        button: MouseButton,
        handler: impl Fn(DragRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_drag(button, handler);
        self
    }

    pub fn on_hover(
        mut self,
        handler: impl Fn(HoverRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_hover(handler);
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
        let hit_bounds = self.hit_bounds(visible_bounds);
        if let Some(style) = self.cursor_style {
            cx.scene.push_cursor_region(CursorRegion {
                bounds: hit_bounds,
                style,
            });
        }

        cx.scene.push_mouse_region(MouseRegion::from_handlers(
            cx.current_view_id(),
            Some(self.discriminant),
            hit_bounds,
            self.handlers.clone(),
        ));

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
