use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json, AnyElement, Element, PaintContext, SizeConstraint, ViewContext,
};
use json::ToJson;

use serde_json::json;

pub struct Align<V> {
    child: AnyElement<V>,
    alignment: Vector2F,
}

impl<V> Align<V> {
    pub fn new(child: AnyElement<V>) -> Self {
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

impl<V: 'static> Element<V> for Align<V> {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        mut constraint: SizeConstraint,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let mut size = constraint.max;
        constraint.min = Vector2F::zero();
        let child_size = self.child.layout(constraint, view, cx);
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
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Self::PaintState {
        let my_center = bounds.size() / 2.;
        let my_target = my_center + my_center * self.alignment;

        let child_center = self.child.size() / 2.;
        let child_target = child_center + child_center * self.alignment;

        self.child.paint(
            bounds.origin() - (child_target - my_target),
            visible_bounds,
            view,
            cx,
        );
    }

    fn rect_for_text_range(
        &self,
        range_utf16: std::ops::Range<usize>,
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
        bounds: pathfinder_geometry::rect::RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
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
