use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json, DebugContext, Element, ElementBox, Event, EventContext, LayoutContext, PaintContext,
    SizeConstraint,
};
use json::ToJson;

use serde_json::json;

pub struct Align {
    child: ElementBox,
    alignment: Vector2F,
}

impl Align {
    pub fn new(child: ElementBox) -> Self {
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

impl Element for Align {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        mut constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let mut size = constraint.max;
        constraint.min = Vector2F::zero();
        let child_size = self.child.layout(constraint, cx);
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
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        let my_center = bounds.size() / 2.;
        let my_target = my_center + my_center * self.alignment;

        let child_center = self.child.size() / 2.;
        let child_target = child_center + child_center * self.alignment;

        self.child.paint(
            bounds.origin() - (child_target - my_target),
            visible_bounds,
            cx,
        );
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: RectF,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool {
        self.child.dispatch_event(event, cx)
    }

    fn debug(
        &self,
        bounds: pathfinder_geometry::rect::RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &DebugContext,
    ) -> json::Value {
        json!({
            "type": "Align",
            "bounds": bounds.to_json(),
            "alignment": self.alignment.to_json(),
            "child": self.child.debug(cx),
        })
    }
}
