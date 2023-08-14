use std::ops::Range;

use gpui::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::ToJson,
    serde_json::{self, json},
    AnyElement, Axis, Element, LayoutContext, PaintContext, SceneBuilder, ViewContext,
};

use crate::CollabTitlebarItem;

pub(crate) struct FacePile {
    overlap: f32,
    faces: Vec<AnyElement<CollabTitlebarItem>>,
}

impl FacePile {
    pub fn new(overlap: f32) -> FacePile {
        FacePile {
            overlap,
            faces: Vec::new(),
        }
    }
}

impl Element<CollabTitlebarItem> for FacePile {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        view: &mut CollabTitlebarItem,
        cx: &mut LayoutContext<CollabTitlebarItem>,
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
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        _layout: &mut Self::LayoutState,
        view: &mut CollabTitlebarItem,
        cx: &mut PaintContext<CollabTitlebarItem>,
    ) -> Self::PaintState {
        let visible_bounds = bounds.intersection(visible_bounds).unwrap_or_default();

        let origin_y = bounds.upper_right().y();
        let mut origin_x = bounds.upper_right().x();

        for face in self.faces.iter_mut().rev() {
            let size = face.size();
            origin_x -= size.x();
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
        _: &CollabTitlebarItem,
        _: &ViewContext<CollabTitlebarItem>,
    ) -> Option<RectF> {
        None
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &CollabTitlebarItem,
        _: &ViewContext<CollabTitlebarItem>,
    ) -> serde_json::Value {
        json!({
            "type": "FacePile",
            "bounds": bounds.to_json()
        })
    }
}

impl Extend<AnyElement<CollabTitlebarItem>> for FacePile {
    fn extend<T: IntoIterator<Item = AnyElement<CollabTitlebarItem>>>(&mut self, children: T) {
        self.faces.extend(children);
    }
}
