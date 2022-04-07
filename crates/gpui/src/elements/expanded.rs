use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json, DebugContext, Element, ElementBox, Event, EventContext, LayoutContext, PaintContext,
    SizeConstraint,
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

    pub fn to_full_width(mut self) -> Self {
        self.full_width = true;
        self.full_height = false;
        self
    }

    pub fn to_full_height(mut self) -> Self {
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
        json!({
            "type": "Expanded",
            "full_width": self.full_width,
            "full_height": self.full_height,
            "child": self.child.debug(cx)
        })
    }
}
