use std::ops::Range;

use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json,
    presenter::MeasurementContext,
    DebugContext, Element, ElementBox, LayoutContext, PaintContext, SizeConstraint,
};
use serde_json::json;

pub struct Expanded {
    child: ElementBox,
    full_width: bool,
    full_height: bool,
}

impl Expanded {
    pub fn new(child: ElementBox) -> Self {
        Self {
            child,
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

impl Element for Expanded {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        mut constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        if self.full_width {
            constraint.min.set_x(constraint.max.x());
        }
        if self.full_height {
            constraint.min.set_y(constraint.max.y());
        }
        let size = self.child.layout(constraint, cx);
        (size, ())
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        self.child.paint(bounds.origin(), visible_bounds, cx);
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
            "type": "Expanded",
            "full_width": self.full_width,
            "full_height": self.full_height,
            "child": self.child.debug(cx)
        })
    }
}
