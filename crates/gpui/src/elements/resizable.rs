use std::{cell::RefCell, rc::Rc};

use collections::HashMap;
use pathfinder_geometry::vector::{vec2f, Vector2F};
use serde_json::json;

use crate::{
    geometry::rect::RectF,
    platform::{CursorStyle, MouseButton},
    AnyElement, AppContext, Axis, Element, MouseRegion, PaintContext, SizeConstraint, TypeTag,
    View, ViewContext,
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

    fn relevant_component(&self, vector: Vector2F) -> f32 {
        match self.axis() {
            Axis::Horizontal => vector.x(),
            Axis::Vertical => vector.y(),
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

fn get_bounds(tag: TypeTag, cx: &AppContext) -> Option<&(RectF, RectF)>
where
{
    cx.optional_global::<ProviderMap>()
        .and_then(|map| map.0.get(&tag))
}

pub struct Resizable<V: 'static> {
    child: AnyElement<V>,
    tag: TypeTag,
    handle_side: HandleSide,
    handle_size: f32,
    on_resize: Rc<RefCell<dyn FnMut(&mut V, Option<f32>, &mut ViewContext<V>)>>,
}

const DEFAULT_HANDLE_SIZE: f32 = 4.0;

impl<V: 'static> Resizable<V> {
    pub fn new<Tag: 'static>(
        child: AnyElement<V>,
        handle_side: HandleSide,
        size: f32,
        on_resize: impl 'static + FnMut(&mut V, Option<f32>, &mut ViewContext<V>),
    ) -> Self {
        let child = match handle_side.axis() {
            Axis::Horizontal => child.constrained().with_max_width(size),
            Axis::Vertical => child.constrained().with_max_height(size),
        }
        .into_any();

        Self {
            child,
            handle_side,
            tag: TypeTag::new::<Tag>(),
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
        cx: &mut ViewContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        (self.child.layout(constraint, view, cx), constraint)
    }

    fn paint(
        &mut self,
        bounds: pathfinder_geometry::rect::RectF,
        visible_bounds: pathfinder_geometry::rect::RectF,
        constraint: &mut SizeConstraint,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Self::PaintState {
        cx.scene().push_stacking_context(None, None);

        let handle_region = self.handle_side.of_rect(bounds, self.handle_size);

        enum ResizeHandle {}
        let view_id = cx.view_id();
        cx.scene().push_mouse_region(
            MouseRegion::new::<ResizeHandle>(view_id, self.handle_side as usize, handle_region)
                .on_down(MouseButton::Left, |_, _: &mut V, _| {}) // This prevents the mouse down event from being propagated elsewhere
                .on_click(MouseButton::Left, {
                    let on_resize = self.on_resize.clone();
                    move |click, v, cx| {
                        if click.click_count == 2 {
                            on_resize.borrow_mut()(v, None, cx);
                        }
                    }
                })
                .on_drag(MouseButton::Left, {
                    let bounds = bounds.clone();
                    let side = self.handle_side;
                    let prev_size = side.relevant_component(bounds.size());
                    let min_size = side.relevant_component(constraint.min);
                    let max_size = side.relevant_component(constraint.max);
                    let on_resize = self.on_resize.clone();
                    let tag = self.tag;
                    move |event, view: &mut V, cx| {
                        if event.end {
                            return;
                        }

                        let Some((bounds, _)) = get_bounds(tag, cx) else {
                            return;
                        };

                        let new_size_raw = match side {
                            // Handle on top side of element => Element is on bottom
                            HandleSide::Top => {
                                bounds.height() + bounds.origin_y() - event.position.y()
                            }
                            // Handle on right side of element => Element is on left
                            HandleSide::Right => event.position.x() - bounds.lower_left().x(),
                            // Handle on left side of element => Element is on the right
                            HandleSide::Left => {
                                bounds.width() + bounds.origin_x() - event.position.x()
                            }
                            // Handle on bottom side of element => Element is on the top
                            HandleSide::Bottom => event.position.y() - bounds.lower_left().y(),
                        };

                        let new_size = min_size.max(new_size_raw).min(max_size).round();
                        if new_size != prev_size {
                            on_resize.borrow_mut()(view, Some(new_size), cx);
                        }
                    }
                }),
        );

        cx.scene().push_cursor_region(crate::CursorRegion {
            bounds: handle_region,
            style: match self.handle_side.axis() {
                Axis::Horizontal => CursorStyle::ResizeLeftRight,
                Axis::Vertical => CursorStyle::ResizeUpDown,
            },
        });

        cx.scene().pop_stacking_context();

        self.child.paint(bounds.origin(), visible_bounds, view, cx);
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

#[derive(Debug, Default)]
struct ProviderMap(HashMap<TypeTag, (RectF, RectF)>);

pub struct BoundsProvider<V: 'static, P> {
    child: AnyElement<V>,
    phantom: std::marker::PhantomData<P>,
}

impl<V: 'static, P: 'static> BoundsProvider<V, P> {
    pub fn new(child: AnyElement<V>) -> Self {
        Self {
            child,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<V: View, P: 'static> Element<V> for BoundsProvider<V, P> {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: crate::SizeConstraint,
        view: &mut V,
        cx: &mut crate::ViewContext<V>,
    ) -> (pathfinder_geometry::vector::Vector2F, Self::LayoutState) {
        (self.child.layout(constraint, view, cx), ())
    }

    fn paint(
        &mut self,
        bounds: pathfinder_geometry::rect::RectF,
        visible_bounds: pathfinder_geometry::rect::RectF,
        _: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut crate::PaintContext<V>,
    ) -> Self::PaintState {
        cx.update_default_global::<ProviderMap, _, _>(|map, _| {
            map.0.insert(TypeTag::new::<P>(), (bounds, visible_bounds));
        });

        self.child.paint(bounds.origin(), visible_bounds, view, cx)
    }

    fn rect_for_text_range(
        &self,
        range_utf16: std::ops::Range<usize>,
        _: pathfinder_geometry::rect::RectF,
        _: pathfinder_geometry::rect::RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &crate::ViewContext<V>,
    ) -> Option<pathfinder_geometry::rect::RectF> {
        self.child.rect_for_text_range(range_utf16, view, cx)
    }

    fn debug(
        &self,
        _: pathfinder_geometry::rect::RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &crate::ViewContext<V>,
    ) -> serde_json::Value {
        serde_json::json!({
            "type": "Provider",
            "providing": format!("{:?}", TypeTag::new::<P>()),
            "child": self.child.debug(view, cx),
        })
    }
}
