use std::{any::Any, f32::INFINITY};

use crate::{
    json::{self, ToJson, Value},
    Axis, DebugContext, Element, ElementBox, Event, EventContext, LayoutContext, PaintContext,
    SizeConstraint, Vector2FExt,
};
use pathfinder_geometry::{
    rect::RectF,
    vector::{vec2f, Vector2F},
};
use serde_json::json;

pub struct Flex {
    axis: Axis,
    children: Vec<ElementBox>,
}

impl Flex {
    pub fn new(axis: Axis) -> Self {
        Self {
            axis,
            children: Default::default(),
        }
    }

    pub fn row() -> Self {
        Self::new(Axis::Horizontal)
    }

    pub fn column() -> Self {
        Self::new(Axis::Vertical)
    }

    fn child_flex<'b>(child: &ElementBox) -> Option<f32> {
        child.metadata::<FlexParentData>().map(|data| data.flex)
    }
}

impl Extend<ElementBox> for Flex {
    fn extend<T: IntoIterator<Item = ElementBox>>(&mut self, children: T) {
        self.children.extend(children);
    }
}

impl Element for Flex {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let mut total_flex = 0.0;
        let mut fixed_space = 0.0;

        let cross_axis = self.axis.invert();
        let mut cross_axis_max: f32 = 0.0;
        for child in &mut self.children {
            if let Some(flex) = Self::child_flex(&child) {
                total_flex += flex;
            } else {
                let child_constraint = match self.axis {
                    Axis::Horizontal => SizeConstraint::new(
                        vec2f(0.0, constraint.min.y()),
                        vec2f(INFINITY, constraint.max.y()),
                    ),
                    Axis::Vertical => SizeConstraint::new(
                        vec2f(constraint.min.x(), 0.0),
                        vec2f(constraint.max.x(), INFINITY),
                    ),
                };
                let size = child.layout(child_constraint, cx);
                fixed_space += size.along(self.axis);
                cross_axis_max = cross_axis_max.max(size.along(cross_axis));
            }
        }

        let mut size = if total_flex > 0.0 {
            if constraint.max_along(self.axis).is_infinite() {
                panic!("flex contains flexible children but has an infinite constraint along the flex axis");
            }

            let mut remaining_space = constraint.max_along(self.axis) - fixed_space;
            let mut remaining_flex = total_flex;
            for child in &mut self.children {
                if let Some(flex) = Self::child_flex(&child) {
                    let child_max = if remaining_flex == 0.0 {
                        remaining_space
                    } else {
                        let space_per_flex = remaining_space / remaining_flex;
                        space_per_flex * flex
                    };
                    let child_constraint = match self.axis {
                        Axis::Horizontal => SizeConstraint::new(
                            vec2f(0.0, constraint.min.y()),
                            vec2f(child_max, constraint.max.y()),
                        ),
                        Axis::Vertical => SizeConstraint::new(
                            vec2f(constraint.min.x(), 0.0),
                            vec2f(constraint.max.x(), child_max),
                        ),
                    };
                    let child_size = child.layout(child_constraint, cx);
                    remaining_space -= child_size.along(self.axis);
                    remaining_flex -= flex;
                    cross_axis_max = cross_axis_max.max(child_size.along(cross_axis));
                }
            }

            match self.axis {
                Axis::Horizontal => vec2f(constraint.max.x() - remaining_space, cross_axis_max),
                Axis::Vertical => vec2f(cross_axis_max, constraint.max.y() - remaining_space),
            }
        } else {
            match self.axis {
                Axis::Horizontal => vec2f(fixed_space, cross_axis_max),
                Axis::Vertical => vec2f(cross_axis_max, fixed_space),
            }
        };

        if constraint.min.x().is_finite() {
            size.set_x(size.x().max(constraint.min.x()));
        }

        if constraint.min.y().is_finite() {
            size.set_y(size.y().max(constraint.min.y()));
        }

        (size, ())
    }

    fn paint(
        &mut self,
        bounds: RectF,
        _: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        let mut child_origin = bounds.origin();
        for child in &mut self.children {
            child.paint(child_origin, cx);
            match self.axis {
                Axis::Horizontal => child_origin += vec2f(child.size().x(), 0.0),
                Axis::Vertical => child_origin += vec2f(0.0, child.size().y()),
            }
        }
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool {
        let mut handled = false;
        for child in &mut self.children {
            handled = child.dispatch_event(event, cx) || handled;
        }
        handled
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &DebugContext,
    ) -> json::Value {
        json!({
            "type": "Flex",
            "bounds": bounds.to_json(),
            "axis": self.axis.to_json(),
            "children": self.children.iter().map(|child| child.debug(cx)).collect::<Vec<json::Value>>()
        })
    }
}

struct FlexParentData {
    flex: f32,
}

pub struct Expanded {
    metadata: FlexParentData,
    child: ElementBox,
}

impl Expanded {
    pub fn new(flex: f32, child: ElementBox) -> Self {
        Expanded {
            metadata: FlexParentData { flex },
            child,
        }
    }
}

impl Element for Expanded {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let size = self.child.layout(constraint, cx);
        (size, ())
    }

    fn paint(
        &mut self,
        bounds: RectF,
        _: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        self.child.paint(bounds.origin(), cx)
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool {
        self.child.dispatch_event(event, cx)
    }

    fn metadata(&self) -> Option<&dyn Any> {
        Some(&self.metadata)
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &DebugContext,
    ) -> Value {
        json!({
            "type": "Expanded",
            "flex": self.metadata.flex,
            "child": self.child.debug(cx)
        })
    }
}
