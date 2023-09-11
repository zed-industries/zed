use std::ops::Range;

use json::ToJson;
use serde_json::json;

use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json, AnyElement, Element, PaintContext, SizeConstraint, ViewContext,
};

pub struct ConstrainedBox<V> {
    child: AnyElement<V>,
    constraint: Constraint<V>,
}

pub enum Constraint<V> {
    Static(SizeConstraint),
    Dynamic(Box<dyn FnMut(SizeConstraint, &mut V, &mut ViewContext<V>) -> SizeConstraint>),
}

impl<V> ToJson for Constraint<V> {
    fn to_json(&self) -> serde_json::Value {
        match self {
            Constraint::Static(constraint) => constraint.to_json(),
            Constraint::Dynamic(_) => "dynamic".into(),
        }
    }
}

impl<V: 'static> ConstrainedBox<V> {
    pub fn new(child: impl Element<V>) -> Self {
        Self {
            child: child.into_any(),
            constraint: Constraint::Static(Default::default()),
        }
    }

    pub fn dynamically(
        mut self,
        constraint: impl 'static + FnMut(SizeConstraint, &mut V, &mut ViewContext<V>) -> SizeConstraint,
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
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> SizeConstraint {
        match &mut self.constraint {
            Constraint::Static(constraint) => *constraint,
            Constraint::Dynamic(compute_constraint) => {
                compute_constraint(input_constraint, view, cx)
            }
        }
    }
}

impl<V: 'static> Element<V> for ConstrainedBox<V> {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        mut parent_constraint: SizeConstraint,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let constraint = self.constraint(parent_constraint, view, cx);
        parent_constraint.min = parent_constraint.min.max(constraint.min);
        parent_constraint.max = parent_constraint.max.min(constraint.max);
        parent_constraint.max = parent_constraint.max.max(parent_constraint.min);
        let size = self.child.layout(parent_constraint, view, cx);
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
        cx.scene().push_layer(Some(visible_bounds));
        self.child.paint(bounds.origin(), visible_bounds, view, cx);
        cx.scene().pop_layer();
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
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
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> json::Value {
        json!({"type": "ConstrainedBox", "assigned_constraint": self.constraint.to_json(), "child": self.child.debug(view, cx)})
    }
}
