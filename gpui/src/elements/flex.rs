use crate::{
    AfterLayoutContext, AppContext, Axis, Element, Event, EventContext, LayoutContext,
    MutableAppContext, PaintContext, SizeConstraint, Vector2FExt,
};
use pathfinder_geometry::vector::{vec2f, Vector2F};
use std::any::Any;

pub struct Flex {
    axis: Axis,
    children: Vec<Box<dyn Element>>,
    size: Option<Vector2F>,
    origin: Option<Vector2F>,
}

impl Flex {
    pub fn new(axis: Axis) -> Self {
        Self {
            axis,
            children: Default::default(),
            size: None,
            origin: None,
        }
    }

    pub fn row() -> Self {
        Self::new(Axis::Horizontal)
    }

    pub fn column() -> Self {
        Self::new(Axis::Vertical)
    }

    fn child_flex<'b>(child: &dyn Element) -> Option<f32> {
        child
            .parent_data()
            .and_then(|d| d.downcast_ref::<FlexParentData>())
            .map(|data| data.flex)
    }
}

impl Extend<Box<dyn Element>> for Flex {
    fn extend<T: IntoIterator<Item = Box<dyn Element>>>(&mut self, children: T) {
        self.children.extend(children);
    }
}

impl Element for Flex {
    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
        app: &AppContext,
    ) -> Vector2F {
        let mut total_flex = 0.0;
        let mut fixed_space = 0.0;

        let cross_axis = self.axis.invert();
        let mut cross_axis_max: f32 = 0.0;
        for child in &mut self.children {
            if let Some(flex) = Self::child_flex(child.as_ref()) {
                total_flex += flex;
            } else {
                let child_constraint =
                    SizeConstraint::strict_along(cross_axis, constraint.max_along(cross_axis));
                let size = child.layout(child_constraint, ctx, app);
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
                let space_per_flex = remaining_space / remaining_flex;
                if let Some(flex) = Self::child_flex(child.as_ref()) {
                    let child_max = space_per_flex * flex;
                    let child_constraint = match self.axis {
                        Axis::Horizontal => SizeConstraint::new(
                            vec2f(0.0, constraint.max.y()),
                            vec2f(child_max, constraint.max.y()),
                        ),
                        Axis::Vertical => SizeConstraint::new(
                            vec2f(constraint.max.x(), 0.0),
                            vec2f(constraint.max.x(), child_max),
                        ),
                    };
                    let child_size = child.layout(child_constraint, ctx, app);
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

        self.size = Some(size);
        size
    }

    fn after_layout(&mut self, ctx: &mut AfterLayoutContext, app: &mut MutableAppContext) {
        for child in &mut self.children {
            child.after_layout(ctx, app);
        }
    }

    fn paint(&mut self, mut origin: Vector2F, ctx: &mut PaintContext, app: &AppContext) {
        self.origin = Some(origin);

        for child in &mut self.children {
            child.paint(origin, ctx, app);
            match self.axis {
                Axis::Horizontal => origin += vec2f(child.size().unwrap().x(), 0.0),
                Axis::Vertical => origin += vec2f(0.0, child.size().unwrap().y()),
            }
        }
    }

    fn dispatch_event(&self, event: &Event, ctx: &mut EventContext, app: &AppContext) -> bool {
        let mut handled = false;
        for child in &self.children {
            if child.dispatch_event(event, ctx, app) {
                handled = true;
            }
        }
        handled
    }

    fn size(&self) -> Option<Vector2F> {
        self.size
    }
}

struct FlexParentData {
    flex: f32,
}

pub struct Expanded {
    parent_data: FlexParentData,
    child: Box<dyn Element>,
}

impl Expanded {
    pub fn new(flex: f32, child: Box<dyn Element>) -> Self {
        Expanded {
            parent_data: FlexParentData { flex },
            child,
        }
    }
}

impl Element for Expanded {
    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
        app: &AppContext,
    ) -> Vector2F {
        self.child.layout(constraint, ctx, app)
    }

    fn after_layout(&mut self, ctx: &mut AfterLayoutContext, app: &mut MutableAppContext) {
        self.child.after_layout(ctx, app);
    }

    fn paint(&mut self, origin: Vector2F, ctx: &mut PaintContext, app: &AppContext) {
        self.child.paint(origin, ctx, app);
    }

    fn dispatch_event(&self, event: &Event, ctx: &mut EventContext, app: &AppContext) -> bool {
        self.child.dispatch_event(event, ctx, app)
    }

    fn size(&self) -> Option<Vector2F> {
        self.child.size()
    }

    fn parent_data(&self) -> Option<&dyn Any> {
        Some(&self.parent_data)
    }
}
