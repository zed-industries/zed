use std::{any::Any, cell::Cell, f32::INFINITY, ops::Range, rc::Rc};

use crate::{
    json::{self, ToJson, Value},
    presenter::MeasurementContext,
    Axis, DebugContext, Element, ElementBox, ElementStateHandle, Event, EventContext,
    LayoutContext, MouseRegion, PaintContext, RenderContext, ScrollWheelEvent, SizeConstraint,
    Vector2FExt, View,
};
use pathfinder_geometry::{
    rect::RectF,
    vector::{vec2f, Vector2F},
};
use serde_json::json;

#[derive(Default, Clone, Copy)]
struct ScrollState {
    scroll_to: Option<usize>,
    scroll_position: f32,
}

pub struct Flex {
    axis: Axis,
    children: Vec<ElementBox>,
    scroll_state: Option<(ElementStateHandle<Rc<Cell<ScrollState>>>, usize)>,
}

impl Flex {
    pub fn new(axis: Axis) -> Self {
        Self {
            axis,
            children: Default::default(),
            scroll_state: None,
        }
    }

    pub fn row() -> Self {
        Self::new(Axis::Horizontal)
    }

    pub fn column() -> Self {
        Self::new(Axis::Vertical)
    }

    pub fn scrollable<Tag, V>(
        mut self,
        element_id: usize,
        scroll_to: Option<usize>,
        cx: &mut RenderContext<V>,
    ) -> Self
    where
        Tag: 'static,
        V: View,
    {
        let scroll_state_handle =
            cx.default_element_state::<Tag, Rc<Cell<ScrollState>>>(element_id);
        let scroll_state_cell = scroll_state_handle.read(cx);
        let mut scroll_state = scroll_state_cell.get();
        scroll_state.scroll_to = scroll_to;
        scroll_state_cell.set(scroll_state);

        self.scroll_state = Some((scroll_state_handle, cx.handle().id()));

        self
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

    fn handle_scroll(
        e: ScrollWheelEvent,
        axis: Axis,
        scroll_state: Rc<Cell<ScrollState>>,
        remaining_space: f32,
    ) -> bool {
        let precise = e.precise;
        let delta = e.delta;
        if remaining_space < 0. {
            let mut delta = match axis {
                Axis::Horizontal => {
                    if delta.x() != 0. {
                        delta.x()
                    } else {
                        delta.y()
                    }
                }
                Axis::Vertical => delta.y(),
            };
            if !precise {
                delta *= 20.;
            }

            let mut old_state = scroll_state.get();
            old_state.scroll_position -= delta;
            scroll_state.set(old_state);

            return true;
        }
        return false;
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
        let mut contains_float = false;

        let cross_axis = self.axis.invert();
        let mut cross_axis_max: f32 = 0.0;
        for child in &mut self.children {
            let metadata = child.metadata::<FlexParentData>();
            contains_float |= metadata.map_or(false, |metadata| metadata.float);

            if let Some(flex) = metadata.and_then(|metadata| metadata.flex.map(|(flex, _)| flex)) {
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

        if contains_float {
            match self.axis {
                Axis::Horizontal => size.set_x(size.x().max(constraint.max.x())),
                Axis::Vertical => size.set_y(size.y().max(constraint.max.y())),
            }
        }

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

        if let Some(scroll_state) = self.scroll_state.as_ref() {
            scroll_state.0.update(cx, |scroll_state, _| {
                if let Some(scroll_to) = scroll_state.get().scroll_to.take() {
                    let visible_start = scroll_state.get().scroll_position;
                    let visible_end = visible_start + size.along(self.axis);
                    if let Some(child) = self.children.get(scroll_to) {
                        let child_start: f32 = self.children[..scroll_to]
                            .iter()
                            .map(|c| c.size().along(self.axis))
                            .sum();
                        let child_end = child_start + child.size().along(self.axis);

                        let mut old_state = scroll_state.get();
                        if child_start < visible_start {
                            old_state.scroll_position = child_start;
                        } else if child_end > visible_end {
                            old_state.scroll_position = child_end - size.along(self.axis);
                        }
                        scroll_state.set(old_state);
                    }
                }

                let mut old_state = scroll_state.get();
                old_state.scroll_position = old_state.scroll_position.min(-remaining_space).max(0.);
                scroll_state.set(old_state);
            });
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
        let mut remaining_space = *remaining_space;

        let overflowing = remaining_space < 0.;
        if overflowing {
            cx.scene.push_layer(Some(bounds));
        }

        if let Some(scroll_state) = &self.scroll_state {
            cx.scene.push_mouse_region(
                MouseRegion::new::<Self>(scroll_state.1, 0, bounds)
                    .on_scroll({
                        let axis = self.axis;
                        let scroll_state = scroll_state.0.read(cx).clone();
                        move |e, cx| {
                            if Self::handle_scroll(
                                e.platform_event,
                                axis,
                                scroll_state.clone(),
                                remaining_space,
                            ) {
                                cx.propogate_event();
                            }
                        }
                    })
                    .on_move(|_, _| { /* Eat move events so they don't propogate */ }),
            );
        }

        let mut child_origin = bounds.origin();
        if let Some(scroll_state) = self.scroll_state.as_ref() {
            let scroll_position = scroll_state.0.read(cx).get().scroll_position;
            match self.axis {
                Axis::Horizontal => child_origin.set_x(child_origin.x() - scroll_position),
                Axis::Vertical => child_origin.set_y(child_origin.y() - scroll_position),
            }
        }

        for child in &mut self.children {
            if remaining_space > 0. {
                if let Some(metadata) = child.metadata::<FlexParentData>() {
                    if metadata.float {
                        match self.axis {
                            Axis::Horizontal => child_origin += vec2f(remaining_space, 0.0),
                            Axis::Vertical => child_origin += vec2f(0.0, remaining_space),
                        }
                        remaining_space = 0.;
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

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &MeasurementContext,
    ) -> Option<RectF> {
        self.children
            .iter()
            .find_map(|child| child.rect_for_text_range(range_utf16.clone(), cx))
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
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool {
        self.child.dispatch_event(event, cx)
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &MeasurementContext,
    ) -> Option<RectF> {
        self.child.rect_for_text_range(range_utf16, cx)
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
