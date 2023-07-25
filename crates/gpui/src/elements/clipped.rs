use std::ops::Range;

use pathfinder_geometry::{rect::RectF, vector::Vector2F};
use serde_json::json;

use crate::{
    json, AnyElement, Element, LayoutContext, PaintContext, SceneBuilder, SizeConstraint, View,
    ViewContext,
};

pub struct Clipped<V: View> {
    child: AnyElement<V>,
}

impl<V: View> Clipped<V> {
    pub fn new(child: AnyElement<V>) -> Self {
        Self { child }
    }
}

impl<V: View> Element<V> for Clipped<V> {
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
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Self::PaintState {
        scene.paint_layer(Some(bounds), |scene| {
            self.child
                .paint(scene, bounds.origin(), visible_bounds, view, cx)
        })
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
            "type": "Clipped",
            "child": self.child.debug(view, cx)
        })
    }
}
