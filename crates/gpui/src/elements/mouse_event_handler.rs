use super::Padding;
use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    platform::CursorStyle,
    platform::MouseButton,
    scene::{
        CursorRegion, HandlerSet, MouseClick, MouseClickOut, MouseDown, MouseDownOut, MouseDrag,
        MouseHover, MouseMove, MouseMoveOut, MouseScrollWheel, MouseUp, MouseUpOut,
    },
    AnyElement, Element, EventContext, LayoutContext, MouseRegion, MouseState, PaintContext,
    SizeConstraint, TypeTag, ViewContext,
};
use serde_json::json;
use std::ops::Range;

pub struct MouseEventHandler<V: 'static> {
    child: AnyElement<V>,
    region_id: usize,
    cursor_style: Option<CursorStyle>,
    handlers: HandlerSet,
    hoverable: bool,
    notify_on_hover: bool,
    notify_on_click: bool,
    above: bool,
    padding: Padding,
    tag: TypeTag,
}

/// Element which provides a render_child callback with a MouseState and paints a mouse
/// region under (or above) it for easy mouse event handling.
impl<V: 'static> MouseEventHandler<V> {
    pub fn for_child<Tag: 'static>(child: impl Element<V>, region_id: usize) -> Self {
        Self {
            child: child.into_any(),
            region_id,
            cursor_style: None,
            handlers: Default::default(),
            notify_on_hover: false,
            notify_on_click: false,
            hoverable: false,
            above: false,
            padding: Default::default(),
            tag: TypeTag::new::<Tag>(),
        }
    }

    pub fn new<Tag: 'static, E>(
        region_id: usize,
        cx: &mut ViewContext<V>,
        render_child: impl FnOnce(&mut MouseState, &mut ViewContext<V>) -> E,
    ) -> Self
    where
        E: Element<V>,
    {
        let mut mouse_state = cx.mouse_state_dynamic(TypeTag::new::<Tag>(), region_id);
        let child = render_child(&mut mouse_state, cx).into_any();
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
            above: false,
            padding: Default::default(),
            tag: TypeTag::new::<Tag>(),
        }
    }

    pub fn new_dynamic(
        tag: TypeTag,
        region_id: usize,
        cx: &mut ViewContext<V>,
        render_child: impl FnOnce(&mut MouseState, &mut ViewContext<V>) -> AnyElement<V>,
    ) -> Self {
        let mut mouse_state = cx.mouse_state_dynamic(tag, region_id);
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
            above: false,
            padding: Default::default(),
            tag,
        }
    }

    /// Modifies the MouseEventHandler to render the MouseRegion above the child element. Useful
    /// for drag and drop handling and similar events which should be captured before the child
    /// gets the opportunity
    pub fn above<Tag: 'static, D>(
        region_id: usize,
        cx: &mut ViewContext<V>,
        render_child: impl FnOnce(&mut MouseState, &mut ViewContext<V>) -> D,
    ) -> Self
    where
        D: Element<V>,
    {
        let mut handler = Self::new::<Tag, _>(region_id, cx, render_child);
        handler.above = true;
        handler
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
        handler: impl Fn(MouseMove, &mut V, &mut EventContext<V>) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_move(handler);
        self
    }

    pub fn on_move_out(
        mut self,
        handler: impl Fn(MouseMoveOut, &mut V, &mut EventContext<V>) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_move_out(handler);
        self
    }

    pub fn on_down(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseDown, &mut V, &mut EventContext<V>) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_down(button, handler);
        self
    }

    pub fn on_up(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseUp, &mut V, &mut EventContext<V>) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_up(button, handler);
        self
    }

    pub fn on_click(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseClick, &mut V, &mut EventContext<V>) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_click(button, handler);
        self
    }

    pub fn on_click_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseClickOut, &mut V, &mut EventContext<V>) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_click_out(button, handler);
        self
    }

    pub fn on_down_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseDownOut, &mut V, &mut EventContext<V>) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_down_out(button, handler);
        self
    }

    pub fn on_up_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseUpOut, &mut V, &mut EventContext<V>) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_up_out(button, handler);
        self
    }

    pub fn on_drag(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseDrag, &mut V, &mut EventContext<V>) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_drag(button, handler);
        self
    }

    pub fn on_hover(
        mut self,
        handler: impl Fn(MouseHover, &mut V, &mut EventContext<V>) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_hover(handler);
        self
    }

    pub fn on_scroll(
        mut self,
        handler: impl Fn(MouseScrollWheel, &mut V, &mut EventContext<V>) + 'static,
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

    fn paint_regions(&self, bounds: RectF, visible_bounds: RectF, cx: &mut ViewContext<V>) {
        let visible_bounds = visible_bounds.intersection(bounds).unwrap_or_default();
        let hit_bounds = self.hit_bounds(visible_bounds);

        if let Some(style) = self.cursor_style {
            cx.scene().push_cursor_region(CursorRegion {
                bounds: hit_bounds,
                style,
            });
        }
        let view_id = cx.view_id();
        cx.scene().push_mouse_region(
            MouseRegion::from_handlers(
                self.tag,
                view_id,
                self.region_id,
                hit_bounds,
                self.handlers.clone(),
            )
            .with_hoverable(self.hoverable)
            .with_notify_on_hover(self.notify_on_hover)
            .with_notify_on_click(self.notify_on_click),
        );
    }
}

impl<V: 'static> Element<V> for MouseEventHandler<V> {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        (self.child.layout(constraint, view, cx), ())
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Self::PaintState {
        if self.above {
            self.child.paint(bounds.origin(), visible_bounds, view, cx);
            cx.paint_layer(None, |cx| {
                self.paint_regions(bounds, visible_bounds, cx);
            });
        } else {
            self.paint_regions(bounds, visible_bounds, cx);
            self.child.paint(bounds.origin(), visible_bounds, view, cx);
        }
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        self.child.rect_for_text_range(range_utf16, view, cx)
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> serde_json::Value {
        json!({
            "type": "MouseEventHandler",
            "child": self.child.debug(view, cx),
        })
    }
}
