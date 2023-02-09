use std::ops::Range;

use pathfinder_geometry::{rect::RectF, vector::Vector2F};
use serde_json::json;

use crate::{
    json, DebugContext, Element, ElementBox, LayoutContext, MeasurementContext, PaintContext,
    SizeConstraint,
};

pub struct Clipped {
    child: ElementBox,
}

impl Clipped {
    pub fn new(child: ElementBox) -> Self {
        Self { child }
    }
}

impl Element for Clipped {
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
        cx.scene.push_layer(Some(bounds));
        self.child.paint(bounds.origin(), visible_bounds, cx);
        cx.scene.pop_layer();
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &MeasurementContext,
    ) -> Option<RectF> {
        self.child.rect_for_text_range(range_utf16, cx)
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &DebugContext,
    ) -> json::Value {
        json!({
            "type": "Clipped",
            "child": self.child.debug(cx)
        })
    }
}
