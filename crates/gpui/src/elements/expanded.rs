use std::ops::Range;

use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json, AnyElement, Element, LayoutContext, PaintContext, SceneBuilder, SizeConstraint,
    ViewContext,
};
use serde_json::json;

pub struct Expanded<V> {
    child: AnyElement<V>,
    full_width: bool,
    full_height: bool,
}

impl<V: 'static> Expanded<V> {
    pub fn new(child: impl Element<V>) -> Self {
        Self {
            child: child.into_any(),
            full_width: true,
            full_height: true,
        }
    }

    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self.full_height = false;
        self
    }

    pub fn full_height(mut self) -> Self {
        self.full_width = false;
        self.full_height = true;
        self
    }
}

impl<V: 'static> Element<V> for Expanded<V> {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        mut constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        if self.full_width {
            constraint.min.set_x(constraint.max.x());
        }
        if self.full_height {
            constraint.min.set_y(constraint.max.y());
        }
        let size = self.child.layout(constraint, view, cx);
        (size, ())
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Self::PaintState {
        self.child
            .paint(scene, bounds.origin(), visible_bounds, view, cx);
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
    ) -> json::Value {
        json!({
            "type": "Expanded",
            "full_width": self.full_width,
            "full_height": self.full_height,
            "child": self.child.debug(view, cx)
        })
    }
}
