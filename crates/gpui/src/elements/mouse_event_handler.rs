use std::{any::TypeId, rc::Rc};

use super::Padding;
use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    platform::CursorStyle,
    scene::CursorRegion,
    DebugContext, Element, ElementBox, Event, EventContext, LayoutContext, MouseRegion, MouseState,
    PaintContext, RenderContext, SizeConstraint, View,
};
use serde_json::json;

pub struct MouseEventHandler {
    child: ElementBox,
    tag: TypeId,
    id: usize,
    cursor_style: Option<CursorStyle>,
    mouse_down_handler: Option<Rc<dyn Fn(Vector2F, &mut EventContext)>>,
    click_handler: Option<Rc<dyn Fn(Vector2F, usize, &mut EventContext)>>,
    drag_handler: Option<Rc<dyn Fn(Vector2F, &mut EventContext)>>,
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
            id,
            tag: TypeId::of::<Tag>(),
            child: render_child(cx.mouse_state::<Tag>(id), cx),
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

    pub fn on_mouse_down(
        mut self,
        handler: impl Fn(Vector2F, &mut EventContext) + 'static,
    ) -> Self {
        self.mouse_down_handler = Some(Rc::new(handler));
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(Vector2F, usize, &mut EventContext) + 'static,
    ) -> Self {
        self.click_handler = Some(Rc::new(handler));
        self
    }

    pub fn on_drag(mut self, handler: impl Fn(Vector2F, &mut EventContext) + 'static) -> Self {
        self.drag_handler = Some(Rc::new(handler));
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
        if let Some(style) = self.cursor_style {
            cx.scene.push_cursor_region(CursorRegion {
                bounds: self.hit_bounds(bounds),
                style,
            });
        }

        cx.scene.push_mouse_region(MouseRegion {
            view_id: cx.current_view_id(),
            tag: self.tag,
            region_id: self.id,
            bounds: self.hit_bounds(bounds),
            hover: None,
            click: self.click_handler.clone(),
            mouse_down: self.mouse_down_handler.clone(),
            drag: self.drag_handler.clone(),
        });

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
