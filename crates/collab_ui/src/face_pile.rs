use std::ops::Range;

use gpui::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::ToJson,
    serde_json::{self, json},
    AnyElement, Axis, Element, LayoutContext, PaintContext, SceneBuilder, View, ViewContext,
};

pub(crate) struct FacePile<V: View> {
    overlap: f32,
    faces: Vec<AnyElement<V>>,
}

impl<V: View> FacePile<V> {
    pub fn new(overlap: f32) -> Self {
        Self {
            overlap,
            faces: Vec::new(),
        }
    }
}

impl<V: View> Element<V> for FacePile<V> {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        debug_assert!(constraint.max_along(Axis::Horizontal) == f32::INFINITY);

        let mut width = 0.;
        let mut max_height = 0.;
        for face in &mut self.faces {
            let layout = face.layout(constraint, view, cx);
            width += layout.x();
            max_height = f32::max(max_height, layout.y());
        }
        width -= self.overlap * self.faces.len().saturating_sub(1) as f32;

        (
            Vector2F::new(width, max_height.clamp(1., constraint.max.y())),
            (),
        )
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        _layout: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Self::PaintState {
        let visible_bounds = bounds.intersection(visible_bounds).unwrap_or_default();

        let origin_y = bounds.upper_right().y();
        let mut origin_x = bounds.upper_right().x();

        for face in self.faces.iter_mut().rev() {
            let size = face.size();
            origin_x -= size.x();
            let origin_y = origin_y + (bounds.height() - size.y()) / 2.0;
            scene.paint_layer(None, |scene| {
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
            "type": "FacePile",
            "bounds": bounds.to_json()
        })
    }
}

impl<V: View> Extend<AnyElement<V>> for FacePile<V> {
    fn extend<T: IntoIterator<Item = AnyElement<V>>>(&mut self, children: T) {
        self.faces.extend(children);
    }
}
