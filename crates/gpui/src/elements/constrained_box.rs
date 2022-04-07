use json::ToJson;
use serde_json::json;

use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json, DebugContext, Element, ElementBox, Event, EventContext, LayoutContext, PaintContext,
    SizeConstraint,
};

pub struct ConstrainedBox {
    child: ElementBox,
    constraint: SizeConstraint,
}

impl ConstrainedBox {
    pub fn new(child: ElementBox) -> Self {
        Self {
            child,
            constraint: SizeConstraint {
                min: Vector2F::zero(),
                max: Vector2F::splat(f32::INFINITY),
            },
        }
    }

    pub fn with_min_width(mut self, min_width: f32) -> Self {
        self.constraint.min.set_x(min_width);
        self
    }

    pub fn with_max_width(mut self, max_width: f32) -> Self {
        self.constraint.max.set_x(max_width);
        self
    }

    pub fn with_max_height(mut self, max_height: f32) -> Self {
        self.constraint.max.set_y(max_height);
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.constraint.min.set_x(width);
        self.constraint.max.set_x(width);
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.constraint.min.set_y(height);
        self.constraint.max.set_y(height);
        self
    }
}

impl Element for ConstrainedBox {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        mut constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        constraint.min = constraint.min.max(self.constraint.min);
        constraint.max = constraint.max.min(self.constraint.max);
        constraint.max = constraint.max.max(constraint.min);
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
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &DebugContext,
    ) -> json::Value {
        json!({"type": "ConstrainedBox", "set_constraint": self.constraint.to_json(), "child": self.child.debug(cx)})
    }
}
