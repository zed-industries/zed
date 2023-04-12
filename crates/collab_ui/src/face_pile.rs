use std::ops::Range;

use gpui::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::ToJson,
    serde_json::{self, json},
    Axis, DebugContext, Element, ElementBox, MeasurementContext, PaintContext,
};

pub(crate) struct FacePile {
    overlap: f32,
    faces: Vec<ElementBox>,
}

impl FacePile {
    pub fn new(overlap: f32) -> FacePile {
        FacePile {
            overlap,
            faces: Vec::new(),
        }
    }
}

impl Element for FacePile {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        cx: &mut gpui::LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        debug_assert!(constraint.max_along(Axis::Horizontal) == f32::INFINITY);

        let mut width = 0.;
        for face in &mut self.faces {
            width += face.layout(constraint, view, cx).x();
        }
        width -= self.overlap * self.faces.len().saturating_sub(1) as f32;

        (Vector2F::new(width, constraint.max.y()), ())
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        _layout: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        let visible_bounds = bounds.intersection(visible_bounds).unwrap_or_default();

        let origin_y = bounds.upper_right().y();
        let mut origin_x = bounds.upper_right().x();

        for face in self.faces.iter_mut().rev() {
            let size = face.size();
            origin_x -= size.x();
            cx.paint_layer(None, |cx| {
                face.paint(scene, vec2f(origin_x, origin_y), visible_bounds, view, cx);
            });
            origin_x += self.overlap;
        }

        ()
    }

    fn rect_for_text_range(
        &self,
        _: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &MeasurementContext,
    ) -> Option<RectF> {
        None
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &DebugContext,
    ) -> serde_json::Value {
        json!({
            "type": "FacePile",
            "bounds": bounds.to_json()
        })
    }
}

impl Extend<ElementBox> for FacePile {
    fn extend<T: IntoIterator<Item = ElementBox>>(&mut self, children: T) {
        self.faces.extend(children);
    }
}
