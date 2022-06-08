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
    mouse_down: Option<Rc<dyn Fn(Vector2F, &mut EventContext)>>,
    click: Option<Rc<dyn Fn(Vector2F, usize, &mut EventContext)>>,
    right_mouse_down: Option<Rc<dyn Fn(Vector2F, &mut EventContext)>>,
    right_click: Option<Rc<dyn Fn(Vector2F, usize, &mut EventContext)>>,
    mouse_down_out: Option<Rc<dyn Fn(Vector2F, &mut EventContext)>>,
    right_mouse_down_out: Option<Rc<dyn Fn(Vector2F, &mut EventContext)>>,
    drag: Option<Rc<dyn Fn(Vector2F, &mut EventContext)>>,
    hover: Option<Rc<dyn Fn(Vector2F, bool, &mut EventContext)>>,
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
            mouse_down: None,
            click: None,
            right_mouse_down: None,
            right_click: None,
            mouse_down_out: None,
            right_mouse_down_out: None,
            drag: None,
            hover: None,
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
        self.mouse_down = Some(Rc::new(handler));
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(Vector2F, usize, &mut EventContext) + 'static,
    ) -> Self {
        self.click = Some(Rc::new(handler));
        self
    }

    pub fn on_right_mouse_down(
        mut self,
        handler: impl Fn(Vector2F, &mut EventContext) + 'static,
    ) -> Self {
        self.right_mouse_down = Some(Rc::new(handler));
        self
    }

    pub fn on_right_click(
        mut self,
        handler: impl Fn(Vector2F, usize, &mut EventContext) + 'static,
    ) -> Self {
        self.right_click = Some(Rc::new(handler));
        self
    }

    pub fn on_mouse_down_out(
        mut self,
        handler: impl Fn(Vector2F, &mut EventContext) + 'static,
    ) -> Self {
        self.mouse_down_out = Some(Rc::new(handler));
        self
    }

    pub fn on_right_mouse_down_out(
        mut self,
        handler: impl Fn(Vector2F, &mut EventContext) + 'static,
    ) -> Self {
        self.right_mouse_down_out = Some(Rc::new(handler));
        self
    }

    pub fn on_drag(mut self, handler: impl Fn(Vector2F, &mut EventContext) + 'static) -> Self {
        self.drag = Some(Rc::new(handler));
        self
    }

    pub fn on_hover(
        mut self,
        handler: impl Fn(Vector2F, bool, &mut EventContext) + 'static,
    ) -> Self {
        self.hover = Some(Rc::new(handler));
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

        cx.scene.push_mouse_region(MouseRegion {
            view_id: cx.current_view_id(),
            discriminant: Some((self.tag, self.id)),
            bounds: hit_bounds,
            hover: self.hover.clone(),
            click: self.click.clone(),
            mouse_down: self.mouse_down.clone(),
            right_click: self.right_click.clone(),
            right_mouse_down: self.right_mouse_down.clone(),
            mouse_down_out: self.mouse_down_out.clone(),
            right_mouse_down_out: self.right_mouse_down_out.clone(),
            drag: self.drag.clone(),
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
