use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json, Element, ElementBox, SceneBuilder, SizeConstraint, View, ViewContext,
};
use json::ToJson;

use serde_json::json;

pub struct Align<V: View> {
    child: ElementBox<V>,
    alignment: Vector2F,
}

impl<V: View> Align<V> {
    pub fn new(child: ElementBox<V>) -> Self {
        Self {
            child,
            alignment: Vector2F::zero(),
        }
    }

    pub fn top(mut self) -> Self {
        self.alignment.set_y(-1.0);
        self
    }

    pub fn bottom(mut self) -> Self {
        self.alignment.set_y(1.0);
        self
    }

    pub fn left(mut self) -> Self {
        self.alignment.set_x(-1.0);
        self
    }

    pub fn right(mut self) -> Self {
        self.alignment.set_x(1.0);
        self
    }
}

impl<V: View> Element<V> for Align<V> {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        view: &mut V,
        mut constraint: SizeConstraint,
        cx: &mut ViewContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let mut size = constraint.max;
        constraint.min = Vector2F::zero();
        let child_size = self.child.layout(view, constraint, cx);
        if size.x().is_infinite() {
            size.set_x(child_size.x());
        }
        if size.y().is_infinite() {
            size.set_y(child_size.y());
        }
        (size, ())
    }

    fn paint(
        &mut self,
        view: &mut V,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        cx: &mut ViewContext<V>,
    ) -> Self::PaintState {
        let my_center = bounds.size() / 2.;
        let my_target = my_center + my_center * self.alignment;

        let child_center = self.child.size() / 2.;
        let child_target = child_center + child_center * self.alignment;

        self.child.paint(
            view,
            scene,
            bounds.origin() - (child_target - my_target),
            visible_bounds,
            cx,
        );
    }

    fn rect_for_text_range(
        &self,
        view: &V,
        range_utf16: std::ops::Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        self.child.rect_for_text_range(view, range_utf16, cx)
    }

    fn debug(
        &self,
        view: &V,
        bounds: pathfinder_geometry::rect::RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &ViewContext<V>,
    ) -> json::Value {
        json!({
            "type": "Align",
            "bounds": bounds.to_json(),
            "alignment": self.alignment.to_json(),
            "child": self.child.debug(view, cx),
        })
    }
}
