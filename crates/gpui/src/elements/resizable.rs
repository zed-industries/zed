use std::{cell::Cell, rc::Rc};

use pathfinder_geometry::vector::{vec2f, Vector2F};
use serde_json::json;

use crate::{
    geometry::rect::RectF, scene::MouseDrag, Axis, CursorStyle, Element, ElementBox,
    ElementStateHandle, MouseButton, MouseRegion, RenderContext, View,
};

use super::{ConstrainedBox, Hook};

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

    fn relevant_component(&self, vector: Vector2F) -> f32 {
        match self.axis() {
            Axis::Horizontal => vector.x(),
            Axis::Vertical => vector.y(),
        }
    }

    fn compute_delta(&self, e: MouseDrag) -> f32 {
        if self.before_content() {
            self.relevant_component(e.prev_mouse_position) - self.relevant_component(e.position)
        } else {
            self.relevant_component(e.position) - self.relevant_component(e.prev_mouse_position)
        }
    }

    fn of_rect(&self, bounds: RectF, handle_size: f32) -> RectF {
        match self {
            Side::Top => RectF::new(bounds.origin(), vec2f(bounds.width(), handle_size)),
            Side::Left => RectF::new(bounds.origin(), vec2f(handle_size, bounds.height())),
            Side::Bottom => {
                let mut origin = bounds.lower_left();
                origin.set_y(origin.y() - handle_size);
                RectF::new(origin, vec2f(bounds.width(), handle_size))
            }
            Side::Right => {
                let mut origin = bounds.upper_right();
                origin.set_x(origin.x() - handle_size);
                RectF::new(origin, vec2f(handle_size, bounds.height()))
            }
        }
    }
}

struct ResizeHandleState {
    actual_dimension: Cell<f32>,
    custom_dimension: Cell<f32>,
}

pub struct Resizable {
    side: Side,
    handle_size: f32,
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

        let child = Hook::new({
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
        .boxed();

        Self {
            side,
            child,
            handle_size,
            state,
            _state_handle: state_handle,
        }
    }

    pub fn current_size(&self) -> f32 {
        self.state.actual_dimension.get()
    }
}

impl Element for Resizable {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: crate::SizeConstraint,
        cx: &mut crate::LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        (self.child.layout(constraint, cx), ())
    }

    fn paint(
        &mut self,
        bounds: pathfinder_geometry::rect::RectF,
        visible_bounds: pathfinder_geometry::rect::RectF,
        _child_size: &mut Self::LayoutState,
        cx: &mut crate::PaintContext,
    ) -> Self::PaintState {
        cx.scene.push_stacking_context(None, None);

        let handle_region = self.side.of_rect(bounds, self.handle_size);

        enum ResizeHandle {}
        cx.scene.push_mouse_region(
            MouseRegion::new::<ResizeHandle>(
                cx.current_view_id(),
                self.side as usize,
                handle_region,
            )
            .on_down(MouseButton::Left, |_, _| {}) // This prevents the mouse down event from being propagated elsewhere
            .on_drag(MouseButton::Left, {
                let state = self.state.clone();
                let side = self.side;
                move |e, cx| {
                    let prev_width = state.actual_dimension.get();
                    state
                        .custom_dimension
                        .set(0f32.max(prev_width + side.compute_delta(e)).round());
                    cx.notify();
                }
            }),
        );

        cx.scene.push_cursor_region(crate::CursorRegion {
            bounds: handle_region,
            style: match self.side.axis() {
                Axis::Horizontal => CursorStyle::ResizeLeftRight,
                Axis::Vertical => CursorStyle::ResizeUpDown,
            },
        });

        cx.scene.pop_stacking_context();

        self.child.paint(bounds.origin(), visible_bounds, cx);
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
