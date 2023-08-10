use std::{cell::RefCell, rc::Rc};

use pathfinder_geometry::vector::{vec2f, Vector2F};
use serde_json::json;

use crate::{
    geometry::rect::RectF,
    platform::{CursorStyle, MouseButton},
    scene::MouseDrag,
    AnyElement, Axis, Element, LayoutContext, MouseRegion, PaintContext, SceneBuilder,
    SizeConstraint, ViewContext,
};

#[derive(Copy, Clone, Debug)]
pub enum HandleSide {
    Top,
    Bottom,
    Left,
    Right,
}

impl HandleSide {
    fn axis(&self) -> Axis {
        match self {
            HandleSide::Left | HandleSide::Right => Axis::Horizontal,
            HandleSide::Top | HandleSide::Bottom => Axis::Vertical,
        }
    }

    /// 'before' is in reference to the standard english document ordering of left-to-right
    /// then top-to-bottom
    fn before_content(self) -> bool {
        match self {
            HandleSide::Left | HandleSide::Top => true,
            HandleSide::Right | HandleSide::Bottom => false,
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
            HandleSide::Top => RectF::new(bounds.origin(), vec2f(bounds.width(), handle_size)),
            HandleSide::Left => RectF::new(bounds.origin(), vec2f(handle_size, bounds.height())),
            HandleSide::Bottom => {
                let mut origin = bounds.lower_left();
                origin.set_y(origin.y() - handle_size);
                RectF::new(origin, vec2f(bounds.width(), handle_size))
            }
            HandleSide::Right => {
                let mut origin = bounds.upper_right();
                origin.set_x(origin.x() - handle_size);
                RectF::new(origin, vec2f(handle_size, bounds.height()))
            }
        }
    }
}

pub struct Resizable<V> {
    child: AnyElement<V>,
    handle_side: HandleSide,
    handle_size: f32,
    on_resize: Rc<RefCell<dyn FnMut(&mut V, f32, &mut ViewContext<V>)>>,
}

const DEFAULT_HANDLE_SIZE: f32 = 4.0;

impl<V: 'static> Resizable<V> {
    pub fn new(
        child: AnyElement<V>,
        handle_side: HandleSide,
        size: f32,
        on_resize: impl 'static + FnMut(&mut V, f32, &mut ViewContext<V>),
    ) -> Self {
        let child = match handle_side.axis() {
            Axis::Horizontal => child.constrained().with_max_width(size),
            Axis::Vertical => child.constrained().with_max_height(size),
        }
        .into_any();

        Self {
            child,
            handle_side,
            handle_size: DEFAULT_HANDLE_SIZE,
            on_resize: Rc::new(RefCell::new(on_resize)),
        }
    }

    pub fn with_handle_size(mut self, handle_size: f32) -> Self {
        self.handle_size = handle_size;
        self
    }
}

impl<V: 'static> Element<V> for Resizable<V> {
    type LayoutState = SizeConstraint;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: crate::SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        (self.child.layout(constraint, view, cx), constraint)
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: pathfinder_geometry::rect::RectF,
        visible_bounds: pathfinder_geometry::rect::RectF,
        constraint: &mut SizeConstraint,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Self::PaintState {
        scene.push_stacking_context(None, None);

        let handle_region = self.handle_side.of_rect(bounds, self.handle_size);

        enum ResizeHandle {}
        scene.push_mouse_region(
            MouseRegion::new::<ResizeHandle>(
                cx.view_id(),
                self.handle_side as usize,
                handle_region,
            )
            .on_down(MouseButton::Left, |_, _: &mut V, _| {}) // This prevents the mouse down event from being propagated elsewhere
            .on_drag(MouseButton::Left, {
                let bounds = bounds.clone();
                let side = self.handle_side;
                let prev_size = side.relevant_component(bounds.size());
                let min_size = side.relevant_component(constraint.min);
                let max_size = side.relevant_component(constraint.max);
                let on_resize = self.on_resize.clone();
                move |event, view: &mut V, cx| {
                    if event.end {
                        return;
                    }
                    let new_size = min_size
                        .max(prev_size + side.compute_delta(event))
                        .min(max_size)
                        .round();
                    if new_size != prev_size {
                        on_resize.borrow_mut()(view, new_size, cx);
                    }
                }
            }),
        );

        scene.push_cursor_region(crate::CursorRegion {
            bounds: handle_region,
            style: match self.handle_side.axis() {
                Axis::Horizontal => CursorStyle::ResizeLeftRight,
                Axis::Vertical => CursorStyle::ResizeUpDown,
            },
        });

        scene.pop_stacking_context();

        self.child
            .paint(scene, bounds.origin(), visible_bounds, view, cx);
    }

    fn rect_for_text_range(
        &self,
        range_utf16: std::ops::Range<usize>,
        _bounds: pathfinder_geometry::rect::RectF,
        _visible_bounds: pathfinder_geometry::rect::RectF,
        _layout: &Self::LayoutState,
        _paint: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<pathfinder_geometry::rect::RectF> {
        self.child.rect_for_text_range(range_utf16, view, cx)
    }

    fn debug(
        &self,
        _bounds: pathfinder_geometry::rect::RectF,
        _layout: &Self::LayoutState,
        _paint: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> serde_json::Value {
        json!({
            "child": self.child.debug(view, cx),
        })
    }
}
