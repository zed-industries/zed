use std::any::TypeId;

use super::Padding;
use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    platform::CursorStyle,
    scene::CursorRegion,
    DebugContext, Element, ElementBox, Event, EventContext, LayoutContext, MouseButton,
    MouseButtonEvent, MouseMovedEvent, MouseRegion, MouseState, PaintContext, RenderContext,
    SizeConstraint, View,
};
use serde_json::json;

pub struct MouseEventHandler {
    child: ElementBox,
    cursor_style: Option<CursorStyle>,
    region: MouseRegion,
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
            region: MouseRegion::new(0, Some((TypeId::of::<Tag>(), id)), Default::default()),
            padding: Default::default(),
        }
    }

    pub fn with_cursor_style(mut self, cursor: CursorStyle) -> Self {
        self.cursor_style = Some(cursor);
        self
    }

    pub fn on_mouse_down(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseButtonEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.region = self.region.on_down(button, handler);
        self
    }

    pub fn on_click(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseButtonEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.region = self.region.on_click(button, handler);
        self
    }

    pub fn on_mouse_down_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseButtonEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.region = self.region.on_down_out(button, handler);
        self
    }

    pub fn on_drag(
        mut self,
        button: MouseButton,
        handler: impl Fn(Vector2F, MouseMovedEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.region = self.region.on_drag(button, handler);
        self
    }

    pub fn on_hover(
        mut self,
        handler: impl Fn(bool, MouseMovedEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.region = self.region.on_hover(handler);
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

        self.region.view_id = cx.current_view_id();
        self.region.bounds = hit_bounds;
        cx.scene.push_mouse_region(self.region.clone());

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
