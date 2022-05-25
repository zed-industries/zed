use json::ToJson;
use serde_json::json;

use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json, DebugContext, Element, ElementBox, Event, EventContext, LayoutContext, PaintContext,
    SizeConstraint,
};

pub struct ConstrainedBox {
    child: ElementBox,
    constraint: Constraint,
}

pub enum Constraint {
    Static(SizeConstraint),
    Dynamic(Box<dyn FnMut(SizeConstraint, &mut LayoutContext) -> SizeConstraint>),
}

impl ToJson for Constraint {
    fn to_json(&self) -> serde_json::Value {
        match self {
            Constraint::Static(constraint) => constraint.to_json(),
            Constraint::Dynamic(_) => "dynamic".into(),
        }
    }
}

impl ConstrainedBox {
    pub fn new(child: ElementBox) -> Self {
        Self {
            child,
            constraint: Constraint::Static(Default::default()),
        }
    }

    pub fn dynamically(
        mut self,
        constraint: impl 'static + FnMut(SizeConstraint, &mut LayoutContext) -> SizeConstraint,
    ) -> Self {
        self.constraint = Constraint::Dynamic(Box::new(constraint));
        self
    }

    pub fn with_min_width(mut self, min_width: f32) -> Self {
        if let Constraint::Dynamic(_) = self.constraint {
            self.constraint = Constraint::Static(Default::default());
        }

        if let Constraint::Static(constraint) = &mut self.constraint {
            constraint.min.set_x(min_width);
        } else {
            unreachable!()
        }

        self
    }

    pub fn with_max_width(mut self, max_width: f32) -> Self {
        if let Constraint::Dynamic(_) = self.constraint {
            self.constraint = Constraint::Static(Default::default());
        }

        if let Constraint::Static(constraint) = &mut self.constraint {
            constraint.max.set_x(max_width);
        } else {
            unreachable!()
        }

        self
    }

    pub fn with_max_height(mut self, max_height: f32) -> Self {
        if let Constraint::Dynamic(_) = self.constraint {
            self.constraint = Constraint::Static(Default::default());
        }

        if let Constraint::Static(constraint) = &mut self.constraint {
            constraint.max.set_y(max_height);
        } else {
            unreachable!()
        }

        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        if let Constraint::Dynamic(_) = self.constraint {
            self.constraint = Constraint::Static(Default::default());
        }

        if let Constraint::Static(constraint) = &mut self.constraint {
            constraint.min.set_x(width);
            constraint.max.set_x(width);
        } else {
            unreachable!()
        }

        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        if let Constraint::Dynamic(_) = self.constraint {
            self.constraint = Constraint::Static(Default::default());
        }

        if let Constraint::Static(constraint) = &mut self.constraint {
            constraint.min.set_y(height);
            constraint.max.set_y(height);
        } else {
            unreachable!()
        }

        self
    }

    fn constraint(
        &mut self,
        input_constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> SizeConstraint {
        match &mut self.constraint {
            Constraint::Static(constraint) => *constraint,
            Constraint::Dynamic(compute_constraint) => compute_constraint(input_constraint, cx),
        }
    }
}

impl Element for ConstrainedBox {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        mut parent_constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let constraint = self.constraint(parent_constraint, cx);
        parent_constraint.min = parent_constraint.min.max(constraint.min);
        parent_constraint.max = parent_constraint.max.min(constraint.max);
        parent_constraint.max = parent_constraint.max.max(parent_constraint.min);
        let size = self.child.layout(parent_constraint, cx);
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
        json!({"type": "ConstrainedBox", "assigned_constraint": self.constraint.to_json(), "child": self.child.debug(cx)})
    }
}
