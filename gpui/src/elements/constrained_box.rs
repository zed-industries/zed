use json::ToJson;
use serde_json::json;

use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json, AfterLayoutContext, DebugContext, Element, ElementBox, Event, EventContext,
    LayoutContext, PaintContext, SizeConstraint,
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

    pub fn with_max_width(mut self, max_width: f32) -> Self {
        self.constraint.max.set_x(max_width);
        self
    }

    pub fn with_max_height(mut self, max_height: f32) -> Self {
        self.constraint.max.set_y(max_height);
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
        ctx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        constraint.min = constraint.min.max(self.constraint.min);
        constraint.max = constraint.max.min(self.constraint.max);
        let size = self.child.layout(constraint, ctx);
        (size, ())
    }

    fn after_layout(
        &mut self,
        _: Vector2F,
        _: &mut Self::LayoutState,
        ctx: &mut AfterLayoutContext,
    ) {
        self.child.after_layout(ctx);
    }

    fn paint(
        &mut self,
        bounds: RectF,
        _: &mut Self::LayoutState,
        ctx: &mut PaintContext,
    ) -> Self::PaintState {
        self.child.paint(bounds.origin(), ctx);
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        ctx: &mut EventContext,
    ) -> bool {
        self.child.dispatch_event(event, ctx)
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        ctx: &DebugContext,
    ) -> json::Value {
        json!({"type": "ConstrainedBox", "set_constraint": self.constraint.to_json(), "child": self.child.debug(ctx)})
    }
}
