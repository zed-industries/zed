use std::ops::Range;

use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{json, ToJson},
    ViewContext,
};
use crate::{Element, SizeConstraint};

#[derive(Default)]
pub struct Empty {
    collapsed: bool,
}

impl Empty {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn collapsed(mut self) -> Self {
        self.collapsed = true;
        self
    }
}

impl<V: 'static> Element<V> for Empty {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        _: &mut V,
        _: &mut ViewContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let x = if constraint.max.x().is_finite() && !self.collapsed {
            constraint.max.x()
        } else {
            constraint.min.x()
        };
        let y = if constraint.max.y().is_finite() && !self.collapsed {
            constraint.max.y()
        } else {
            constraint.min.y()
        };

        (vec2f(x, y), ())
    }

    fn paint(
        &mut self,
        _: RectF,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut V,
        _: &mut ViewContext<V>,
    ) -> Self::PaintState {
    }

    fn rect_for_text_range(
        &self,
        _: Range<usize>,
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
    ) -> serde_json::Value {
        json!({
            "type": "Empty",
            "bounds": bounds.to_json(),
        })
    }
}
