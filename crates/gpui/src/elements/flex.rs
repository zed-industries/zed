use std::{any::Any, cell::Cell, f32::INFINITY, ops::Range, rc::Rc};

use crate::{
    json::{self, ToJson, Value},
    AnyElement, Axis, Element, ElementStateHandle, LayoutContext, PaintContext, SceneBuilder,
    SizeConstraint, Vector2FExt, ViewContext,
};
use pathfinder_geometry::{
    rect::RectF,
    vector::{vec2f, Vector2F},
};
use serde_json::json;

#[derive(Default)]
struct ScrollState {
    scroll_to: Cell<Option<usize>>,
    scroll_position: Cell<f32>,
}

pub struct Flex<V> {
    axis: Axis,
    children: Vec<AnyElement<V>>,
    scroll_state: Option<(ElementStateHandle<Rc<ScrollState>>, usize)>,
    child_alignment: f32,
    spacing: f32,
}

impl<V: 'static> Flex<V> {
    pub fn new(axis: Axis) -> Self {
        Self {
            axis,
            children: Default::default(),
            scroll_state: None,
            child_alignment: -1.,
            spacing: 0.,
        }
    }

    pub fn row() -> Self {
        Self::new(Axis::Horizontal)
    }

    pub fn column() -> Self {
        Self::new(Axis::Vertical)
    }

    /// Render children centered relative to the cross-axis of the parent flex.
    ///
    /// If this is a flex row, children will be centered vertically. If this is a
    /// flex column, children will be centered horizontally.
    pub fn align_children_center(mut self) -> Self {
        self.child_alignment = 0.;
        self
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn scrollable<Tag>(
        mut self,
        element_id: usize,
        scroll_to: Option<usize>,
        cx: &mut ViewContext<V>,
    ) -> Self
    where
        Tag: 'static,
    {
        let scroll_state = cx.default_element_state::<Tag, Rc<ScrollState>>(element_id);
        scroll_state.read(cx).scroll_to.set(scroll_to);
        self.scroll_state = Some((scroll_state, cx.handle().id()));
        self
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    fn layout_flex_children(
        &mut self,
        layout_expanded: bool,
        constraint: SizeConstraint,
        remaining_space: &mut f32,
        remaining_flex: &mut f32,
        cross_axis_max: &mut f32,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) {
        let cross_axis = self.axis.invert();
        for child in self.children.iter_mut() {
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
                    let child_size = child.layout(child_constraint, view, cx);
                    *remaining_space -= child_size.along(self.axis);
                    *remaining_flex -= flex;
                    *cross_axis_max = cross_axis_max.max(child_size.along(cross_axis));
                }
            }
        }
    }
}

impl<V> Extend<AnyElement<V>> for Flex<V> {
    fn extend<T: IntoIterator<Item = AnyElement<V>>>(&mut self, children: T) {
        self.children.extend(children);
    }
}

impl<V: 'static> Element<V> for Flex<V> {
    type LayoutState = f32;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let mut total_flex = None;
        let mut fixed_space = self.children.len().saturating_sub(1) as f32 * self.spacing;
        let mut contains_float = false;

        let cross_axis = self.axis.invert();
        let mut cross_axis_max: f32 = 0.0;
        for child in self.children.iter_mut() {
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
                let size = child.layout(child_constraint, view, cx);
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
                view,
                cx,
            );
            self.layout_flex_children(
                true,
                constraint,
                &mut remaining_space,
                &mut remaining_flex,
                &mut cross_axis_max,
                view,
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
            scroll_state.0.update(cx.view_context(), |scroll_state, _| {
                if let Some(scroll_to) = scroll_state.scroll_to.take() {
                    let visible_start = scroll_state.scroll_position.get();
                    let visible_end = visible_start + size.along(self.axis);
                    if let Some(child) = self.children.get(scroll_to) {
                        let child_start: f32 = self.children[..scroll_to]
                            .iter()
                            .map(|c| c.size().along(self.axis))
                            .sum();
                        let child_end = child_start + child.size().along(self.axis);
                        if child_start < visible_start {
                            scroll_state.scroll_position.set(child_start);
                        } else if child_end > visible_end {
                            scroll_state
                                .scroll_position
                                .set(child_end - size.along(self.axis));
                        }
                    }
                }

                scroll_state.scroll_position.set(
                    scroll_state
                        .scroll_position
                        .get()
                        .min(-remaining_space)
                        .max(0.),
                );
            });
        }

        (size, remaining_space)
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        remaining_space: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Self::PaintState {
        let visible_bounds = bounds.intersection(visible_bounds).unwrap_or_default();

        let mut remaining_space = *remaining_space;
        let overflowing = remaining_space < 0.;
        if overflowing {
            scene.push_layer(Some(visible_bounds));
        }

        if let Some(scroll_state) = &self.scroll_state {
            scene.push_mouse_region(
                crate::MouseRegion::new::<Self>(scroll_state.1, 0, bounds)
                    .on_scroll({
                        let scroll_state = scroll_state.0.read(cx).clone();
                        let axis = self.axis;
                        move |e, _: &mut V, cx| {
                            if remaining_space < 0. {
                                let scroll_delta = e.delta.raw();

                                let mut delta = match axis {
                                    Axis::Horizontal => {
                                        if scroll_delta.x().abs() >= scroll_delta.y().abs() {
                                            scroll_delta.x()
                                        } else {
                                            scroll_delta.y()
                                        }
                                    }
                                    Axis::Vertical => scroll_delta.y(),
                                };
                                if !e.delta.precise() {
                                    delta *= 20.;
                                }

                                scroll_state
                                    .scroll_position
                                    .set(scroll_state.scroll_position.get() - delta);

                                cx.notify();
                            } else {
                                cx.propagate_event();
                            }
                        }
                    })
                    .on_move(|_, _: &mut V, _| { /* Capture move events */ }),
            )
        }

        let mut child_origin = bounds.origin();
        if let Some(scroll_state) = self.scroll_state.as_ref() {
            let scroll_position = scroll_state.0.read(cx).scroll_position.get();
            match self.axis {
                Axis::Horizontal => child_origin.set_x(child_origin.x() - scroll_position),
                Axis::Vertical => child_origin.set_y(child_origin.y() - scroll_position),
            }
        }

        for child in self.children.iter_mut() {
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

            // We use the child_alignment f32 to determine a point along the cross axis of the
            // overall flex element and each child. We then align these points. So 0 would center
            // each child relative to the overall height/width of the flex. -1 puts children at
            // the start. 1 puts children at the end.
            let aligned_child_origin = {
                let cross_axis = self.axis.invert();
                let my_center = bounds.size().along(cross_axis) / 2.;
                let my_target = my_center + my_center * self.child_alignment;

                let child_center = child.size().along(cross_axis) / 2.;
                let child_target = child_center + child_center * self.child_alignment;

                let mut aligned_child_origin = child_origin;
                match self.axis {
                    Axis::Horizontal => aligned_child_origin
                        .set_y(aligned_child_origin.y() - (child_target - my_target)),
                    Axis::Vertical => aligned_child_origin
                        .set_x(aligned_child_origin.x() - (child_target - my_target)),
                }

                aligned_child_origin
            };

            child.paint(scene, aligned_child_origin, visible_bounds, view, cx);

            match self.axis {
                Axis::Horizontal => child_origin += vec2f(child.size().x() + self.spacing, 0.0),
                Axis::Vertical => child_origin += vec2f(0.0, child.size().y() + self.spacing),
            }
        }

        if overflowing {
            scene.pop_layer();
        }
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
        self.children
            .iter()
            .find_map(|child| child.rect_for_text_range(range_utf16.clone(), view, cx))
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> json::Value {
        json!({
            "type": "Flex",
            "bounds": bounds.to_json(),
            "axis": self.axis.to_json(),
            "children": self.children.iter().map(|child| child.debug(view, cx)).collect::<Vec<json::Value>>()
        })
    }
}

struct FlexParentData {
    flex: Option<(f32, bool)>,
    float: bool,
}

pub struct FlexItem<V> {
    metadata: FlexParentData,
    child: AnyElement<V>,
}

impl<V: 'static> FlexItem<V> {
    pub fn new(child: impl Element<V>) -> Self {
        FlexItem {
            metadata: FlexParentData {
                flex: None,
                float: false,
            },
            child: child.into_any(),
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

impl<V: 'static> Element<V> for FlexItem<V> {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let size = self.child.layout(constraint, view, cx);
        (size, ())
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Self::PaintState {
        self.child
            .paint(scene, bounds.origin(), visible_bounds, view, cx)
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

    fn metadata(&self) -> Option<&dyn Any> {
        Some(&self.metadata)
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Value {
        json!({
            "type": "Flexible",
            "flex": self.metadata.flex,
            "child": self.child.debug(view, cx)
        })
    }
}
