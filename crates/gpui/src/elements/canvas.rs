use std::marker::PhantomData;

use super::Element;
use crate::{
    json::{self, json},
    ViewContext,
};
use json::ToJson;
use pathfinder_geometry::{
    rect::RectF,
    vector::{vec2f, Vector2F},
};

pub struct Canvas<V, F>(F, PhantomData<V>);

impl<V, F> Canvas<V, F>
where
    F: FnMut(RectF, RectF, &mut V, &mut ViewContext<V>),
{
    pub fn new(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<V: 'static, F> Element<V> for Canvas<V, F>
where
    F: 'static + FnMut(RectF, RectF, &mut V, &mut ViewContext<V>),
{
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: crate::SizeConstraint,
        _: &mut V,
        _: &mut crate::ViewContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let x = if constraint.max.x().is_finite() {
            constraint.max.x()
        } else {
            constraint.min.x()
        };
        let y = if constraint.max.y().is_finite() {
            constraint.max.y()
        } else {
            constraint.min.y()
        };
        (vec2f(x, y), ())
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Self::PaintState {
        self.0(bounds, visible_bounds, view, cx)
    }

    fn rect_for_text_range(
        &self,
        _: std::ops::Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &V,
        _: &ViewContext<V>,
    ) -> Option<RectF> {
        None
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &V,
        _: &ViewContext<V>,
    ) -> json::Value {
        json!({"type": "Canvas", "bounds": bounds.to_json()})
    }
}
