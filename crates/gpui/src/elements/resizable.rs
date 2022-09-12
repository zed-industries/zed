use std::{cell::Cell, rc::Rc};

use pathfinder_geometry::vector::Vector2F;
use serde_json::json;

use crate::{
    color::Color, scene::DragRegionEvent, Axis, Border, CursorStyle, Element, ElementBox,
    ElementStateHandle, MouseButton, RenderContext, View, MouseRegion,
};

use super::{ConstrainedBox, Empty, Flex, Hook, MouseEventHandler, Padding, ParentElement};

#[derive(Copy, Clone, Debug)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

impl Side {
    fn axis(&self) -> Axis {
        match self {
            Side::Left | Side::Right => Axis::Horizontal,
            Side::Top | Side::Bottom => Axis::Vertical,
        }
    }

    /// 'before' is in reference to the standard english document ordering of left-to-right
    /// then top-to-bottom
    fn before_content(self) -> bool {
        match self {
            Side::Left | Side::Top => true,
            Side::Right | Side::Bottom => false,
        }
    }

    fn resize_padding(&self, padding_size: f32) -> Padding {
        match self.axis() {
            Axis::Horizontal => Padding::horizontal(padding_size),
            Axis::Vertical => Padding::vertical(padding_size),
        }
    }

    fn relevant_component(&self, vector: Vector2F) -> f32 {
        match self.axis() {
            Axis::Horizontal => vector.x(),
            Axis::Vertical => vector.y(),
        }
    }

    fn compute_delta(&self, e: DragRegionEvent) -> f32 {
        if self.before_content() {
            self.relevant_component(e.prev_mouse_position) - self.relevant_component(e.position)
        } else {
            self.relevant_component(e.position) - self.relevant_component(e.prev_mouse_position)
        }
    }
}

struct ResizeHandleState {
    actual_dimension: Cell<f32>,
    custom_dimension: Cell<f32>,
}

pub struct Resizable {
    side: Side,
    child: ElementBox,
    state: Rc<ResizeHandleState>,
    _state_handle: ElementStateHandle<Rc<ResizeHandleState>>,
}

impl Resizable {
    pub fn new<Tag: 'static, T: View>(
        child: ElementBox,
        element_id: usize,
        side: Side,
        handle_size: f32,
        initial_size: f32,
        cx: &mut RenderContext<T>,
    ) -> Self {
        let state_handle = cx.element_state::<Tag, Rc<ResizeHandleState>>(
            element_id,
            Rc::new(ResizeHandleState {
                actual_dimension: Cell::new(initial_size),
                custom_dimension: Cell::new(initial_size),
            }),
        );

        let state = state_handle.read(cx).clone();

        let mut flex = Flex::new(side.axis());

        if side.before_content() {
            dbg!("HANDLE BEING RENDERED BEFORE");
            flex.add_child(render_resize_handle(state.clone(), side, handle_size, cx))
        }

        flex.add_child(
            Hook::new({
                let constrained = ConstrainedBox::new(child);
                match side.axis() {
                    Axis::Horizontal => constrained.with_max_width(state.custom_dimension.get()),
                    Axis::Vertical => constrained.with_max_height(state.custom_dimension.get()),
                }
                .boxed()
            })
            .on_after_layout({
                let state = state.clone();
                move |size, _| {
                    state.actual_dimension.set(side.relevant_component(size));
                }
            })
            .boxed(),
        );

        if !side.before_content() {
            dbg!("HANDLE BEING RENDERED AFTER");
            flex.add_child(render_resize_handle(state.clone(), side, handle_size, cx))
        }

        let child = flex.boxed();

        Self {
            side,
            child,
            state,
            _state_handle: state_handle,
        }
    }
}

fn render_resize_handle<T: View>(
    state: Rc<ResizeHandleState>,
    side: Side,
    padding_size: f32,
    cx: &mut RenderContext<T>,
) -> ElementBox {
    enum ResizeHandle {}
    MouseEventHandler::<ResizeHandle>::new(side as usize, cx, |_, _| {
        Empty::new()
            // Border necessary to properly add a MouseRegion
            .contained()
            .with_border(Border {
                width: 4.,
                left: true,
                color: Color::red(),
                ..Default::default()
            })
            .boxed()
    })
    .with_padding(side.resize_padding(padding_size))
    .with_cursor_style(match side.axis() {
        Axis::Horizontal => CursorStyle::ResizeLeftRight,
        Axis::Vertical => CursorStyle::ResizeUpDown,
    })
    .on_down(MouseButton::Left, |_, _| {}) // This prevents the mouse down event from being propagated elsewhere
    .on_drag(MouseButton::Left, move |e, cx| {
        let prev_width = state.actual_dimension.get();
        state
            .custom_dimension
            .set(0f32.max(prev_width + side.compute_delta(e)).round());
        cx.notify();
    })
    .boxed()
}

impl Element for Resizable {
    type LayoutState = Vector2F;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: crate::SizeConstraint,
        cx: &mut crate::LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let child_size = self.child.layout(constraint, cx);
        (child_size, child_size)
    }

    fn paint(
        &mut self,
        bounds: pathfinder_geometry::rect::RectF,
        visible_bounds: pathfinder_geometry::rect::RectF,
        child_size: &mut Self::LayoutState,
        cx: &mut crate::PaintContext,
    ) -> Self::PaintState {
        cx.scene.push_stacking_context(None);
        
        // Render a mouse region on the appropriate border (likely just bounds)
        // Use the padding in the above code to decide the size of the rect to pass to the mouse region
        // Add handlers for Down and Drag like above
        
        // Maybe try pushing a quad to visually inspect where the region gets placed
        // Push a cursor region
        cx.scene.push_mouse_region(MouseRegion::)
        
        cx.scene.pop_stacking_context();

        self.child.paint(bounds.origin(), visible_bounds, cx);
    }

    fn dispatch_event(
        &mut self,
        event: &crate::Event,
        _bounds: pathfinder_geometry::rect::RectF,
        _visible_bounds: pathfinder_geometry::rect::RectF,
        _layout: &mut Self::LayoutState,
        _paint: &mut Self::PaintState,
        cx: &mut crate::EventContext,
    ) -> bool {
        self.child.dispatch_event(event, cx)
    }

    fn rect_for_text_range(
        &self,
        range_utf16: std::ops::Range<usize>,
        _bounds: pathfinder_geometry::rect::RectF,
        _visible_bounds: pathfinder_geometry::rect::RectF,
        _layout: &Self::LayoutState,
        _paint: &Self::PaintState,
        cx: &crate::MeasurementContext,
    ) -> Option<pathfinder_geometry::rect::RectF> {
        self.child.rect_for_text_range(range_utf16, cx)
    }

    fn debug(
        &self,
        _bounds: pathfinder_geometry::rect::RectF,
        _layout: &Self::LayoutState,
        _paint: &Self::PaintState,
        cx: &crate::DebugContext,
    ) -> serde_json::Value {
        json!({
            "child": self.child.debug(cx),
        })
    }
}
