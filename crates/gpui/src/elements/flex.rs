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

    fn layout_flex_children(
        &mut self,
        layout_expanded: bool,
        constraint: SizeConstraint,
        remaining_space: &mut f32,
        remaining_flex: &mut f32,
        cross_axis_max: &mut f32,
        cx: &mut LayoutContext,
    ) {
        let cross_axis = self.axis.invert();
        for child in &mut self.children {
            if let Some(metadata) = child.metadata::<FlexParentData>() {
                if let Some((flex, expanded)) = metadata.flex {
                    if expanded != layout_expanded {
                        continue;
                    }

                    let child_max = if *remaining_flex == 0.0 {
                        *remaining_space
                    } else {
                        let space_per_flex = *remaining_space / *remaining_flex;
                        space_per_flex * flex
                    };
                    let child_min = if expanded { child_max } else { 0. };
                    let child_constraint = match self.axis {
                        Axis::Horizontal => SizeConstraint::new(
                            vec2f(child_min, constraint.min.y()),
                            vec2f(child_max, constraint.max.y()),
                        ),
                        Axis::Vertical => SizeConstraint::new(
                            vec2f(constraint.min.x(), child_min),
                            vec2f(constraint.max.x(), child_max),
                        ),
                    };
                    let child_size = child.layout(child_constraint, cx);
                    *remaining_space -= child_size.along(self.axis);
                    *remaining_flex -= flex;
                    *cross_axis_max = cross_axis_max.max(child_size.along(cross_axis));
                }
            }
        }
    }
}

impl Extend<ElementBox> for Flex {
    fn extend<T: IntoIterator<Item = ElementBox>>(&mut self, children: T) {
        self.children.extend(children);
    }
}

impl Element for Flex {
    type LayoutState = f32;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let mut total_flex = None;
        let mut fixed_space = 0.0;

        let cross_axis = self.axis.invert();
        let mut cross_axis_max: f32 = 0.0;
        for child in &mut self.children {
            if let Some(flex) = child
                .metadata::<FlexParentData>()
                .and_then(|metadata| metadata.flex.map(|(flex, _)| flex))
            {
                *total_flex.get_or_insert(0.) += flex;
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

        let mut remaining_space = constraint.max_along(self.axis) - fixed_space;
        let mut size = if let Some(mut remaining_flex) = total_flex {
            if remaining_space.is_infinite() {
                panic!("flex contains flexible children but has an infinite constraint along the flex axis");
            }

            self.layout_flex_children(
                false,
                constraint,
                &mut remaining_space,
                &mut remaining_flex,
                &mut cross_axis_max,
                cx,
            );
            self.layout_flex_children(
                true,
                constraint,
                &mut remaining_space,
                &mut remaining_flex,
                &mut cross_axis_max,
                cx,
            );

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

        if size.x() > constraint.max.x() {
            size.set_x(constraint.max.x());
        }
        if size.y() > constraint.max.y() {
            size.set_y(constraint.max.y());
        }

        (size, remaining_space)
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        remaining_space: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        let overflowing = *remaining_space < 0.;
        if overflowing {
            cx.scene.push_layer(Some(bounds));
        }
        let mut child_origin = bounds.origin();
        for child in &mut self.children {
            if *remaining_space > 0. {
                if let Some(metadata) = child.metadata::<FlexParentData>() {
                    if metadata.float {
                        match self.axis {
                            Axis::Horizontal => child_origin += vec2f(*remaining_space, 0.0),
                            Axis::Vertical => child_origin += vec2f(0.0, *remaining_space),
                        }
                        *remaining_space = 0.;
                    }
                }
            }
            child.paint(child_origin, visible_bounds, cx);
            match self.axis {
                Axis::Horizontal => child_origin += vec2f(child.size().x(), 0.0),
                Axis::Vertical => child_origin += vec2f(0.0, child.size().y()),
            }
        }
        if overflowing {
            cx.scene.pop_layer();
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
    flex: Option<(f32, bool)>,
    float: bool,
}

pub struct FlexItem {
    metadata: FlexParentData,
    child: ElementBox,
}

impl FlexItem {
    pub fn new(child: ElementBox) -> Self {
        FlexItem {
            metadata: FlexParentData {
                flex: None,
                float: false,
            },
            child,
        }
    }

    pub fn flex(mut self, flex: f32, expanded: bool) -> Self {
        self.metadata.flex = Some((flex, expanded));
        self
    }

    pub fn float(mut self) -> Self {
        self.metadata.float = true;
        self
    }
}

impl Element for FlexItem {
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
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        self.child.paint(bounds.origin(), visible_bounds, cx)
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
            "type": "Flexible",
            "flex": self.metadata.flex,
            "child": self.child.debug(cx)
        })
    }
}
