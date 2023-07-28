use derive_more::Add;
use gpui::elements::layout_highlighted_chunks;
use gpui::{
    color::Color,
    fonts::HighlightStyle,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{json, ToJson},
    scene,
    serde_json::Value,
    text_layout::{Line, ShapedBoundary},
    AnyElement, AppContext, Element, LayoutContext, PaintContext, Quad, SceneBuilder,
    SizeConstraint, View, ViewContext,
};
use length::{Length, Rems};
use log::warn;
use optional_struct::*;
use std::{
    any::Any,
    borrow::Cow,
    f32,
    ops::{Add, Range},
    sync::Arc,
};

pub struct Node<V: View> {
    style: NodeStyle,
    children: Vec<AnyElement<V>>,
    id: Option<Cow<'static, str>>,
}

pub fn node<V: View>(child: impl Element<V>) -> Node<V> {
    Node::default().child(child)
}

pub fn column<V: View>() -> Node<V> {
    Node::default()
}

pub fn row<V: View>() -> Node<V> {
    Node {
        style: NodeStyle {
            axis: Axis3d::X,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn stack<V: View>() -> Node<V> {
    Node {
        style: NodeStyle {
            axis: Axis3d::Z,
            ..Default::default()
        },
        ..Default::default()
    }
}

impl<V: View> Default for Node<V> {
    fn default() -> Self {
        Self {
            style: Default::default(),
            children: Default::default(),
            id: None,
        }
    }
}

impl<V: View> Element<V> for Node<V> {
    type LayoutState = NodeLayout;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let layout = if let Some(axis) = self.style.axis.to_2d() {
            self.layout_xy(axis, constraint, cx.rem_pixels(), view, cx)
        } else {
            todo!()
        };

        (layout.size.max(constraint.min), layout)
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut NodeLayout,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Self::PaintState {
        let margined_bounds = RectF::from_points(
            bounds.origin() + vec2f(layout.margins.left, layout.margins.top),
            bounds.lower_right() - vec2f(layout.margins.right, layout.margins.bottom),
        );

        // Paint drop shadow
        for shadow in &self.style.shadows {
            scene.push_shadow(scene::Shadow {
                bounds: margined_bounds + shadow.offset,
                corner_radius: self.style.corner_radius,
                sigma: shadow.blur,
                color: shadow.color,
            });
        }

        // // Paint cursor style
        // if let Some(hit_bounds) = content_bounds.intersection(visible_bounds) {
        //     if let Some(style) = self.style.cursor {
        //         scene.push_cursor_region(CursorRegion {
        //             bounds: hit_bounds,
        //             style,
        //         });
        //     }
        // }

        // Render the background and/or the border.
        let Fill::Color(fill_color) = self.style.fill;
        let is_fill_visible = !fill_color.is_fully_transparent();
        if is_fill_visible || self.style.borders.is_visible() {
            eprintln!(
                "{}: paint background: {:?}",
                self.id.as_deref().unwrap_or(""),
                margined_bounds
            );

            scene.push_quad(Quad {
                bounds: margined_bounds,
                background: is_fill_visible.then_some(fill_color),
                border: scene::Border {
                    width: self.style.borders.width,
                    color: self.style.borders.color,
                    overlay: false,
                    top: self.style.borders.top,
                    right: self.style.borders.right,
                    bottom: self.style.borders.bottom,
                    left: self.style.borders.left,
                },
                corner_radius: self.style.corner_radius,
            });
        }

        if !self.children.is_empty() {
            // Account for padding first.
            let borders = &self.style.borders;
            let padded_bounds = RectF::from_points(
                margined_bounds.origin()
                    + vec2f(
                        borders.left_width() + layout.padding.left,
                        borders.top_width() + layout.padding.top,
                    ),
                margined_bounds.lower_right()
                    - vec2f(
                        layout.padding.right + borders.right_width(),
                        layout.padding.bottom + borders.bottom_width(),
                    ),
            );

            if let Some(axis) = self.style.axis.to_2d() {
                // let parent_size = padded_bounds.size();
                let mut child_origin = padded_bounds.origin();

                // Align all children together along the primary axis
                // let mut align_horizontally = false;
                // let mut align_vertically = false;
                // match axis {
                //     Axis2d::X => align_horizontally = true,
                //     Axis2d::Y => align_vertically = true,
                // }
                // align_child(
                //     &mut child_origin,
                //     parent_size,
                //     layout.content_size,
                //     self.style.align.0,
                //     align_horizontally,
                //     align_vertically,
                // );

                for child in &mut self.children {
                    // Align each child along the cross axis
                    // align_horizontally = !align_horizontally;
                    // align_vertically = !align_vertically;
                    // align_child(
                    //     &mut child_origin,
                    //     parent_size,
                    //     child.size(),
                    //     self.style.align.0,
                    //     align_horizontally,
                    //     align_vertically,
                    // );
                    //
                    child.paint(scene, child_origin, visible_bounds, view, cx);

                    // Advance along the primary axis by the size of this child
                    child_origin.set(axis, child_origin.get(axis) + child.size().get(axis));
                }
            } else {
                todo!();
            }
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
    ) -> Value {
        json!({
            "type": "Node",
            "bounds": bounds.to_json(),
            // TODO!
            // "children": self.content.iter().map(|child| child.debug(view, cx)).collect::<Vec<Value>>()
        })
    }

    fn metadata(&self) -> Option<&dyn Any> {
        Some(&self.style)
    }
}

impl<V: View> Node<V> {
    pub fn id(mut self, id: impl Into<Cow<'static, str>>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn child(mut self, child: impl Element<V>) -> Self {
        self.children.push(child.into_any());
        self
    }

    pub fn children<I, E>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Element<V>,
    {
        self.children
            .extend(children.into_iter().map(|child| child.into_any()));
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.style.size.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.style.size.height = height.into();
        self
    }

    pub fn fill(mut self, fill: impl Into<Fill>) -> Self {
        self.style.fill = fill.into();
        self
    }

    pub fn text_size(mut self, text_size: Rems) -> Self {
        self.style.text.size = Some(text_size);
        self
    }

    pub fn margins(
        mut self,
        top_bottom: impl Into<TopBottom>,
        left_right: impl Into<LeftRight>,
    ) -> Self {
        let top_bottom = top_bottom.into();
        let left_right = left_right.into();
        self.style.margins = Edges {
            top: top_bottom.top,
            bottom: top_bottom.bottom,
            left: left_right.left,
            right: left_right.right,
        };
        self
    }

    pub fn margin_top(mut self, top: Length) -> Self {
        self.style.margins.top = top;
        self
    }

    pub fn margin_bottom(mut self, bottom: Length) -> Self {
        self.style.margins.bottom = bottom;
        self
    }

    pub fn margin_left(mut self, left: impl Into<Length>) -> Self {
        self.style.margins.left = left.into();
        self
    }

    pub fn margin_right(mut self, right: impl Into<Length>) -> Self {
        self.style.margins.right = right.into();
        self
    }

    fn id_as_string(&self) -> String {
        self.id.as_deref().unwrap_or("<anonymous>").to_string()
    }

    fn log(&self, s: &str) {
        eprintln!("{}: {}", self.id_as_string(), s);
    }

    fn layout_xy(
        &mut self,
        primary_axis: Axis2d,
        constraint: SizeConstraint,
        rem_pixels: f32,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> NodeLayout {
        self.log(&format!("{:?}", constraint));

        let cross_axis = primary_axis.rotate();
        let total_flex = self.style.flex();
        let mut layout = NodeLayout {
            size: Default::default(),
            padding: self.style.padding.fixed_pixels(rem_pixels),
            margins: self.style.margins.fixed_pixels(rem_pixels),
            borders: self.style.borders.edges(),
        };
        let fixed_padding_size = layout.padding.size();
        let fixed_margin_size = layout.margins.size();
        let borders_size = layout.borders.size();
        let padded_constraint = constraint - fixed_margin_size - borders_size - fixed_padding_size;
        let mut child_constraint = SizeConstraint::default();

        dbg!(self.id_as_string());
        for axis in [Axis2d::X, Axis2d::Y] {
            let length = self.style.size.get(axis);
            dbg!(axis, length);

            match length {
                Length::Fixed(fixed_length) => {
                    // If the length is fixed, we calculate flexible padding and margins
                    // before laying out the children.
                    let fixed_length = fixed_length.to_pixels(rem_pixels);
                    let mut remaining_flex = total_flex.get(axis);
                    let mut remaining_length =
                        (padded_constraint.max.get(axis) - fixed_length).max(0.);

                    // Here we avoid the padding exceeding the fixed length by giving
                    // the padding calculation its own remaining_flex and remaining_length.
                    let mut padding_flex = self.style.padding.flex().get(axis);
                    let mut padding_length =
                        ((padding_flex / remaining_flex) * remaining_length).min(fixed_length);
                    layout.padding.compute_flex_edges(
                        &self.style.padding,
                        axis,
                        &mut padding_flex,
                        &mut padding_length,
                        rem_pixels,
                    );
                    remaining_flex -= padding_flex;
                    remaining_length -= padding_length;
                    layout.margins.compute_flex_edges(
                        &self.style.margins,
                        axis,
                        &mut remaining_flex,
                        &mut remaining_length,
                        rem_pixels,
                    );

                    dbg!(remaining_flex, remaining_length);

                    child_constraint.max.set(axis, remaining_length);
                    if axis == cross_axis {
                        child_constraint.min.set(axis, remaining_length);
                    }
                }
                Length::Auto { .. } => {
                    // If the length is flex, we calculate the content's share first.
                    // We then layout the children and determine the flexible padding
                    // and margins in a second phase.
                    let mut remaining_flex = total_flex.get(axis);
                    let mut remaining_length = dbg!(padded_constraint.max.get(axis));
                    let content_length =
                        length.flex_pixels(rem_pixels, &mut remaining_flex, &mut remaining_length);
                    dbg!(content_length);
                    child_constraint.max.set(axis, content_length);
                    if axis == cross_axis {
                        child_constraint.min.set(axis, content_length);
                    }
                }
                Length::Hug => {
                    // If hug, leave the child constraint in its default zero state.
                    // This will tell children to be as small as possible along this dimension,
                    // and we calculate the flexible padding and margins in a second phase.
                }
            }
        }

        let content_size = {
            dbg!(self.id_as_string(), "lay out children");
            // Layout fixed children using the child constraint determined above.
            let mut remaining_child_length = dbg!(child_constraint.max).get(primary_axis);
            let mut remaining_child_flex = 0.;
            let mut total_child_length = 0.;
            let mut cross_axis_max: f32 = 0.;
            child_constraint.min.set(primary_axis, 0.);
            child_constraint.max.set(primary_axis, 0.);

            for child in &mut self.children {
                // Don't lay out children that are flexible along the primary for this first pass,
                // but total up their flex for use in the second pass.
                if let Some(child_flex) = child
                    .metadata::<NodeStyle>()
                    .map(|style| style.flex().get(primary_axis))
                {
                    if child_flex > 0. {
                        remaining_child_flex += child_flex;
                        continue;
                    }
                }

                // The child is fixed along the primary axis, so perform layout.
                let child_size = child.layout(child_constraint, view, cx);
                let child_length = child_size.get(primary_axis);
                remaining_child_length -= child_length;
                total_child_length += child_length;
                cross_axis_max = cross_axis_max.max(child_size.get(cross_axis));
            }

            // Now divide the remaining length among the flexible children.
            let id = self.id_as_string();
            for child in &mut self.children {
                if let Some(child_flex) = child
                    .metadata::<NodeStyle>()
                    .map(|style| style.flex().get(primary_axis))
                {
                    if child_flex > 0. {
                        eprintln!("{}: child is flexible", id);

                        let max_child_length =
                            (child_flex / remaining_child_flex) * remaining_child_length;
                        child_constraint.max.set(primary_axis, max_child_length);

                        let child_size = child.layout(child_constraint, view, cx);
                        let child_length = child_size.get(primary_axis);
                        total_child_length += child_length;
                        remaining_child_length -= child_length;
                        remaining_child_flex -= child_flex;
                        cross_axis_max = cross_axis_max.max(child_size.get(cross_axis));
                    }
                }
            }

            match primary_axis {
                Axis2d::X => vec2f(total_child_length, cross_axis_max),
                Axis2d::Y => vec2f(cross_axis_max, total_child_length),
            }
        };

        // Now distribute remaining space to flexible padding and margins.
        dbg!(self.id_as_string());
        for axis in [Axis2d::X, Axis2d::Y] {
            dbg!(axis);
            let length = self.style.size.get(axis);

            // Finish with flexible margins and padding now that children are laid out.
            match length {
                Length::Hug => {
                    // Now that we know the size of our children, we can distribute
                    // space to flexible padding and margins.
                    let mut remaining_flex = total_flex.get(axis);
                    let mut remaining_length =
                        padded_constraint.min.get(axis) - content_size.get(axis);
                    layout.padding.compute_flex_edges(
                        &self.style.padding,
                        axis,
                        &mut remaining_flex,
                        &mut remaining_length,
                        rem_pixels,
                    );
                    layout.margins.compute_flex_edges(
                        &self.style.margins,
                        axis,
                        &mut remaining_flex,
                        &mut remaining_length,
                        rem_pixels,
                    );
                    layout.size.set(
                        axis,
                        content_size.get(axis)
                            + layout.padding.size().get(axis)
                            + layout.borders.size().get(axis)
                            + layout.margins.size().get(axis),
                    );
                }
                Length::Fixed(fixed) => {
                    // For a fixed length, we've already computed margins and padding
                    // before laying out children. Padding and border are included in the
                    // fixed length, so we just add the margins to determine the size.
                    layout.size.set(
                        axis,
                        fixed.to_pixels(rem_pixels) + layout.margins.size().get(axis),
                    )
                }
                Length::Auto { .. } => {
                    let mut remaining_flex = total_flex.get(axis);
                    let mut remaining_length = padded_constraint.max.get(axis);

                    dbg!(remaining_flex, remaining_length);

                    let flex_length =
                        length.flex_pixels(rem_pixels, &mut remaining_flex, &mut remaining_length);

                    dbg!(flex_length, remaining_flex, remaining_length);

                    layout.padding.compute_flex_edges(
                        &self.style.padding,
                        axis,
                        &mut remaining_flex,
                        &mut remaining_length,
                        rem_pixels,
                    );

                    dbg!(remaining_flex, remaining_length);

                    layout.margins.compute_flex_edges(
                        &self.style.margins,
                        axis,
                        &mut remaining_flex,
                        &mut remaining_length,
                        rem_pixels,
                    );

                    dbg!(remaining_flex, remaining_length);

                    layout.size.set(
                        axis,
                        flex_length
                            + layout.padding.size().get(axis)
                            + layout.borders.size().get(axis)
                            + layout.margins.size().get(axis),
                    )
                }
            }
        }

        self.log(&format!("{:?}", layout));

        layout
    }
}

pub struct TopBottom {
    top: Length,
    bottom: Length,
}

impl<T: Into<Length>> From<(T, T)> for TopBottom {
    fn from((top, bottom): (T, T)) -> Self {
        Self {
            top: top.into(),
            bottom: bottom.into(),
        }
    }
}

impl<T: Copy + Into<Length>> From<T> for TopBottom {
    fn from(both: T) -> Self {
        Self {
            top: both.into(),
            bottom: both.into(),
        }
    }
}

pub struct LeftRight {
    left: Length,
    right: Length,
}

impl From<(Length, Length)> for LeftRight {
    fn from((left, right): (Length, Length)) -> Self {
        Self { left, right }
    }
}

impl From<Length> for LeftRight {
    fn from(both: Length) -> Self {
        Self {
            left: both,
            right: both,
        }
    }
}

fn align_child(
    child_origin: &mut Vector2F,
    parent_size: Vector2F,
    child_size: Vector2F,
    alignment: Vector2F,
    horizontal: bool,
    vertical: bool,
) {
    let parent_center = parent_size / 2.;
    let parent_target = parent_center + parent_center * alignment;
    let child_center = child_size / 2.;
    let child_target = child_center + child_center * alignment;

    if horizontal {
        child_origin.set_x(child_origin.x() + parent_target.x() - child_target.x())
    }
    if vertical {
        child_origin.set_y(child_origin.y() + parent_target.y() - child_target.y());
    }
}

struct Interactive<Style> {
    default: Style,
    hovered: Style,
    active: Style,
    disabled: Style,
}

#[derive(Clone, Default)]
pub struct NodeStyle {
    axis: Axis3d,
    wrap: bool,
    align: Align,
    overflow_x: Overflow,
    overflow_y: Overflow,
    gap_x: Gap,
    gap_y: Gap,

    size: Size<Length>,
    margins: Edges<Length>,
    padding: Edges<Length>,
    text: OptionalTextStyle,
    opacity: f32,
    fill: Fill,
    borders: Borders,
    corner_radius: f32,
    shadows: Vec<Shadow>,
}

impl NodeStyle {
    fn flex(&self) -> Vector2F {
        self.size.flex() + self.padding.flex() + self.margins.flex()
    }
}

#[optional_struct]
struct TextStyle {
    size: Rems,
    font_family: Arc<str>,
    weight: FontWeight,
    style: FontStyle,
}

#[derive(Add, Default, Clone)]
struct Size<T> {
    width: T,
    height: T,
}

impl<T: Copy> Size<T> {
    fn get(&self, axis: Axis2d) -> T {
        match axis {
            Axis2d::X => self.width,
            Axis2d::Y => self.height,
        }
    }
}

impl Size<Length> {
    fn fixed_pixels(&self, rem_pixels: f32) -> Size<f32> {
        Size {
            width: self.width.fixed_pixels(rem_pixels),
            height: self.height.fixed_pixels(rem_pixels),
        }
    }

    pub fn fixed(&self) -> Size<Rems> {
        Size {
            width: self.width.fixed().unwrap_or_default(),
            height: self.height.fixed().unwrap_or_default(),
        }
    }

    pub fn flex(&self) -> Vector2F {
        vec2f(
            self.width.flex().unwrap_or(0.),
            self.height.flex().unwrap_or(0.),
        )
    }
}

impl Size<Rems> {
    pub fn to_pixels(&self, rem_size: f32) -> Vector2F {
        vec2f(
            self.width.to_pixels(rem_size),
            self.height.to_pixels(rem_size),
        )
    }
}

#[derive(Clone, Default, Debug)]
struct Edges<T> {
    top: T,
    bottom: T,
    left: T,
    right: T,
}

impl<T> Edges<T> {
    fn start(&self, axis: Axis2d) -> &T {
        match axis {
            Axis2d::X => &self.left,
            Axis2d::Y => &self.top,
        }
    }

    fn start_mut(&mut self, axis: Axis2d) -> &mut T {
        match axis {
            Axis2d::X => &mut self.left,
            Axis2d::Y => &mut self.top,
        }
    }

    fn end(&self, axis: Axis2d) -> &T {
        match axis {
            Axis2d::X => &self.right,
            Axis2d::Y => &self.bottom,
        }
    }

    fn end_mut(&mut self, axis: Axis2d) -> &mut T {
        match axis {
            Axis2d::X => &mut self.right,
            Axis2d::Y => &mut self.bottom,
        }
    }
}

impl Edges<f32> {
    fn size(&self) -> Vector2F {
        vec2f(self.left + self.right, self.top + self.bottom)
    }

    fn compute_flex_edges(
        &mut self,
        style_edges: &Edges<Length>,
        axis: Axis2d,
        remaining_flex: &mut f32,
        remaining_length: &mut f32,
        rem_pixels: f32,
    ) {
        *self.start_mut(axis) +=
            style_edges
                .start(axis)
                .flex_pixels(rem_pixels, remaining_flex, remaining_length);
        *self.end_mut(axis) +=
            style_edges
                .end(axis)
                .flex_pixels(rem_pixels, remaining_flex, remaining_length);
    }
}

impl Edges<Length> {
    fn fixed_pixels(&self, rem_pixels: f32) -> Edges<f32> {
        Edges {
            top: self.top.fixed_pixels(rem_pixels),
            bottom: self.bottom.fixed_pixels(rem_pixels),
            left: self.left.fixed_pixels(rem_pixels),
            right: self.right.fixed_pixels(rem_pixels),
        }
    }

    fn flex_pixels(
        &self,
        rem_pixels: f32,
        remaining_flex: &mut f32,
        remaining_length: &mut f32,
    ) -> Edges<f32> {
        Edges {
            top: self
                .top
                .flex_pixels(rem_pixels, remaining_flex, remaining_length),
            bottom: self
                .bottom
                .flex_pixels(rem_pixels, remaining_flex, remaining_length),
            left: self
                .left
                .flex_pixels(rem_pixels, remaining_flex, remaining_length),
            right: self
                .right
                .flex_pixels(rem_pixels, remaining_flex, remaining_length),
        }
    }

    // pub fn fixed(&self) -> Size<Rems> {
    //     let mut size = Size::default();
    //     size.width += self.left.fixed().unwrap_or_default();
    //     size.width += self.right.fixed().unwrap_or_default();
    //     size
    // }

    pub fn flex(&self) -> Vector2F {
        vec2f(
            self.left.flex().unwrap_or(0.) + self.right.flex().unwrap_or(0.),
            self.top.flex().unwrap_or(0.) + self.bottom.flex().unwrap_or(0.),
        )
    }
}

impl Edges<Rems> {
    pub fn to_pixels(&self, rem_size: f32) -> Edges<f32> {
        Edges {
            top: self.top.to_pixels(rem_size),
            bottom: self.bottom.to_pixels(rem_size),
            left: self.left.to_pixels(rem_size),
            right: self.right.to_pixels(rem_size),
        }
    }
}

#[derive(Clone, Default)]
struct CornerRadii {
    top_left: f32,
    top_right: f32,
    bottom_right: f32,
    bottom_left: f32,
}

#[derive(Clone)]
pub enum Fill {
    Color(Color),
}

impl From<Color> for Fill {
    fn from(value: Color) -> Self {
        Fill::Color(value)
    }
}

impl Default for Fill {
    fn default() -> Self {
        Fill::Color(Color::default())
    }
}

#[derive(Clone, Default)]
struct Borders {
    color: Color,
    width: f32,
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

impl Borders {
    fn is_visible(&self) -> bool {
        self.width > 0.
            && !self.color.is_fully_transparent()
            && (self.top || self.bottom || self.left || self.right)
    }

    fn top_width(&self) -> f32 {
        if self.top {
            self.width
        } else {
            0.
        }
    }

    fn bottom_width(&self) -> f32 {
        if self.bottom {
            self.width
        } else {
            0.
        }
    }

    fn left_width(&self) -> f32 {
        if self.left {
            self.width
        } else {
            0.
        }
    }

    fn right_width(&self) -> f32 {
        if self.right {
            self.width
        } else {
            0.
        }
    }

    fn edges(&self) -> Edges<f32> {
        let mut edges = Edges::default();
        if self.width > 0. {
            if self.top {
                edges.top = self.width;
            }
            if self.bottom {
                edges.bottom = self.width;
            }
            if self.left {
                edges.left = self.width;
            }
            if self.right {
                edges.right = self.width;
            }
        }
        edges
    }

    fn size(&self) -> Vector2F {
        let width =
            if self.left { self.width } else { 0. } + if self.right { self.width } else { 0. };
        let height =
            if self.top { self.width } else { 0. } + if self.bottom { self.width } else { 0. };

        vec2f(width, height)
    }
}

pub mod length {
    use derive_more::{Add, AddAssign, Into};

    #[derive(Add, AddAssign, Into, Clone, Copy, Default, Debug, PartialEq)]
    pub struct Rems(f32);

    pub fn rems(rems: f32) -> Rems {
        Rems(rems)
    }

    impl Rems {
        pub fn to_pixels(&self, rem_pixels: f32) -> f32 {
            self.0 * rem_pixels
        }
    }

    #[derive(Clone, Copy, Default, Debug)]
    pub enum Length {
        #[default]
        Hug,
        Fixed(Rems),
        Auto {
            flex: f32,
            min: Rems,
            max: Rems,
        },
    }

    impl From<Rems> for Length {
        fn from(value: Rems) -> Self {
            Length::Fixed(value)
        }
    }

    pub fn auto() -> Length {
        flex(1.)
    }

    pub fn flex(flex: f32) -> Length {
        Length::Auto {
            flex,
            min: Default::default(),
            max: rems(f32::INFINITY),
        }
    }

    pub fn constrained(flex: f32, min: Option<Rems>, max: Option<Rems>) -> Length {
        Length::Auto {
            flex,
            min: min.unwrap_or(Default::default()),
            max: max.unwrap_or(rems(f32::INFINITY)),
        }
    }

    impl Length {
        pub fn flex_pixels(
            &self,
            rem_pixels: f32,
            remaining_flex: &mut f32,
            remaining_length: &mut f32,
        ) -> f32 {
            match self {
                Length::Auto { flex, min, max } => {
                    let flex_length = *remaining_length / *remaining_flex;
                    let length = (flex * flex_length)
                        .clamp(min.to_pixels(rem_pixels), max.to_pixels(rem_pixels));
                    *remaining_flex -= flex;
                    *remaining_length -= length;
                    length
                }
                _ => 0.,
            }
        }

        pub fn fixed_pixels(&self, rem: f32) -> f32 {
            match self {
                Length::Fixed(rems) => rems.to_pixels(rem),
                _ => 0.,
            }
        }

        pub fn flex(&self) -> Option<f32> {
            match self {
                Length::Auto { flex, .. } => Some(*flex),
                _ => None,
            }
        }

        pub fn fixed(&self) -> Option<Rems> {
            match self {
                Length::Fixed(rems) => Some(*rems),
                _ => None,
            }
        }
    }
}

#[derive(Clone)]
struct Align(Vector2F);

impl Default for Align {
    fn default() -> Self {
        Self(vec2f(-1., -1.))
    }
}

#[derive(Clone, Copy, Default)]
enum Axis3d {
    X,
    #[default]
    Y,
    Z,
}

impl Axis3d {
    fn to_2d(self) -> Option<Axis2d> {
        match self {
            Axis3d::X => Some(Axis2d::X),
            Axis3d::Y => Some(Axis2d::Y),
            Axis3d::Z => None,
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum Axis2d {
    X,
    #[default]
    Y,
}

impl Axis2d {
    fn rotate(self) -> Self {
        match self {
            Axis2d::X => Axis2d::Y,
            Axis2d::Y => Axis2d::X,
        }
    }
}

#[derive(Clone, Copy, Default)]
enum Overflow {
    #[default]
    Visible,
    Hidden,
    Auto,
}

#[derive(Clone, Copy)]
enum Gap {
    Fixed(f32),
    Around,
    Between,
    Even,
}

impl Default for Gap {
    fn default() -> Self {
        Gap::Fixed(0.)
    }
}

#[derive(Clone, Copy, Default)]
struct Shadow {
    offset: Vector2F,
    blur: f32,
    color: Color,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    #[default]
    Normal,
    Medium,
    Semibold,
    Bold,
    ExtraBold,
    Black,
}

#[derive(Default)]
pub struct Text {
    text: Cow<'static, str>,
    highlights: Option<Box<[(Range<usize>, HighlightStyle)]>>,
    custom_runs: Option<(
        Box<[Range<usize>]>,
        Box<dyn FnMut(usize, RectF, &mut SceneBuilder, &mut AppContext)>,
    )>,
}

pub fn text<V: View>(text: impl Into<Cow<'static, str>>) -> Node<V> {
    row().child(Text {
        text: text.into(),
        ..Default::default()
    })
}

#[derive(Default, Debug)]
pub struct NodeLayout {
    size: Vector2F,
    padding: Edges<f32>,
    borders: Edges<f32>,
    margins: Edges<f32>,
}

impl<V: View> Element<V> for Text {
    type LayoutState = TextLayout;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        _: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        // Convert the string and highlight ranges into an iterator of highlighted chunks.
        let mut offset = 0;
        let mut highlight_ranges = self
            .highlights
            .as_ref()
            .map_or(Default::default(), AsRef::as_ref)
            .iter()
            .peekable();
        let chunks = std::iter::from_fn(|| {
            let result;
            if let Some((range, highlight_style)) = highlight_ranges.peek() {
                if offset < range.start {
                    result = Some((&self.text[offset..range.start], None));
                    offset = range.start;
                } else if range.end <= self.text.len() {
                    result = Some((&self.text[range.clone()], Some(*highlight_style)));
                    highlight_ranges.next();
                    offset = range.end;
                } else {
                    warn!(
                        "Highlight out of text range. Text len: {}, Highlight range: {}..{}",
                        self.text.len(),
                        range.start,
                        range.end
                    );
                    result = None;
                }
            } else if offset < self.text.len() {
                result = Some((&self.text[offset..], None));
                offset = self.text.len();
            } else {
                result = None;
            }
            result
        });

        let style = cx.text_style();

        // Perform shaping on these highlighted chunks
        let shaped_lines = layout_highlighted_chunks(
            chunks,
            &style,
            cx.text_layout_cache(),
            &cx.font_cache,
            usize::MAX,
            self.text.matches('\n').count() + 1,
        );

        // If line wrapping is enabled, wrap each of the shaped lines.
        let font_id = style.font_id;
        let mut line_count = 0;
        let mut max_line_width = 0_f32;
        let mut wrap_boundaries = Vec::new();
        let mut wrapper = cx.font_cache.line_wrapper(font_id, style.font_size);
        for (line, shaped_line) in self.text.split('\n').zip(&shaped_lines) {
            if style.soft_wrap {
                let boundaries = wrapper
                    .wrap_shaped_line(line, shaped_line, constraint.max.x())
                    .collect::<Vec<_>>();
                line_count += boundaries.len() + 1;
                wrap_boundaries.push(boundaries);
            } else {
                line_count += 1;
            }
            max_line_width = max_line_width.max(shaped_line.width());
        }

        let line_height = cx.font_cache.line_height(style.font_size);
        let size = vec2f(
            max_line_width
                .ceil()
                .max(constraint.min.x())
                .min(constraint.max.x()),
            (line_height * line_count as f32).ceil(),
        );
        (
            size,
            TextLayout {
                shaped_lines,
                wrap_boundaries,
                line_height,
            },
        )
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut Self::LayoutState,
        _: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Self::PaintState {
        dbg!(bounds);

        let mut origin = bounds.origin();
        let empty = Vec::new();
        let mut callback = |_, _, _: &mut SceneBuilder, _: &mut AppContext| {};

        let mouse_runs;
        let custom_run_callback;
        if let Some((runs, build_region)) = &mut self.custom_runs {
            mouse_runs = runs.iter();
            custom_run_callback = build_region.as_mut();
        } else {
            mouse_runs = [].iter();
            custom_run_callback = &mut callback;
        }
        let mut custom_runs = mouse_runs.enumerate().peekable();

        let mut offset = 0;
        for (ix, line) in layout.shaped_lines.iter().enumerate() {
            let wrap_boundaries = layout.wrap_boundaries.get(ix).unwrap_or(&empty);
            let boundaries = RectF::new(
                origin,
                vec2f(
                    bounds.width(),
                    (wrap_boundaries.len() + 1) as f32 * layout.line_height,
                ),
            );

            let style = cx.text_style();
            if boundaries.intersects(visible_bounds) {
                if style.soft_wrap {
                    line.paint_wrapped(
                        scene,
                        origin,
                        visible_bounds,
                        layout.line_height,
                        wrap_boundaries,
                        cx,
                    );
                } else {
                    line.paint(scene, origin, visible_bounds, layout.line_height, cx);
                }
            }

            // Paint any custom runs that intersect this line.
            let end_offset = offset + line.len();
            if let Some((custom_run_ix, custom_run_range)) = custom_runs.peek().cloned() {
                if custom_run_range.start < end_offset {
                    let mut current_custom_run = None;
                    if custom_run_range.start <= offset {
                        current_custom_run = Some((custom_run_ix, custom_run_range.end, origin));
                    }

                    let mut glyph_origin = origin;
                    let mut prev_position = 0.;
                    let mut wrap_boundaries = wrap_boundaries.iter().copied().peekable();
                    for (run_ix, glyph_ix, glyph) in
                        line.runs().iter().enumerate().flat_map(|(run_ix, run)| {
                            run.glyphs()
                                .iter()
                                .enumerate()
                                .map(move |(ix, glyph)| (run_ix, ix, glyph))
                        })
                    {
                        glyph_origin.set_x(glyph_origin.x() + glyph.position.x() - prev_position);
                        prev_position = glyph.position.x();

                        // If we've reached a soft wrap position, move down one line. If there
                        // is a custom run in-progress, paint it.
                        if wrap_boundaries
                            .peek()
                            .map_or(false, |b| b.run_ix == run_ix && b.glyph_ix == glyph_ix)
                        {
                            if let Some((run_ix, _, run_origin)) = &mut current_custom_run {
                                let bounds = RectF::from_points(
                                    *run_origin,
                                    glyph_origin + vec2f(0., layout.line_height),
                                );
                                custom_run_callback(*run_ix, bounds, scene, cx);
                                *run_origin =
                                    vec2f(origin.x(), glyph_origin.y() + layout.line_height);
                            }
                            wrap_boundaries.next();
                            glyph_origin = vec2f(origin.x(), glyph_origin.y() + layout.line_height);
                        }

                        // If we've reached the end of the current custom run, paint it.
                        if let Some((run_ix, run_end_offset, run_origin)) = current_custom_run {
                            if offset + glyph.index == run_end_offset {
                                current_custom_run.take();
                                let bounds = RectF::from_points(
                                    run_origin,
                                    glyph_origin + vec2f(0., layout.line_height),
                                );
                                custom_run_callback(run_ix, bounds, scene, cx);
                                custom_runs.next();
                            }

                            if let Some((_, run_range)) = custom_runs.peek() {
                                if run_range.start >= end_offset {
                                    break;
                                }
                                if run_range.start == offset + glyph.index {
                                    current_custom_run =
                                        Some((run_ix, run_range.end, glyph_origin));
                                }
                            }
                        }

                        // If we've reached the start of a new custom run, start tracking it.
                        if let Some((run_ix, run_range)) = custom_runs.peek() {
                            if offset + glyph.index == run_range.start {
                                current_custom_run = Some((*run_ix, run_range.end, glyph_origin));
                            }
                        }
                    }

                    // If a custom run extends beyond the end of the line, paint it.
                    if let Some((run_ix, run_end_offset, run_origin)) = current_custom_run {
                        let line_end = glyph_origin + vec2f(line.width() - prev_position, 0.);
                        let bounds = RectF::from_points(
                            run_origin,
                            line_end + vec2f(0., layout.line_height),
                        );
                        custom_run_callback(run_ix, bounds, scene, cx);
                        if end_offset == run_end_offset {
                            custom_runs.next();
                        }
                    }
                }
            }

            offset = end_offset + 1;
            origin.set_y(boundaries.max_y());
        }
    }

    fn rect_for_text_range(
        &self,
        _: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &V,
        _: &ViewContext<V>,
    ) -> Option<RectF> {
        None
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &V,
        _: &ViewContext<V>,
    ) -> Value {
        json!({
            "type": "Text",
            "bounds": bounds.to_json(),
            "text": &self.text,
        })
    }
}

pub struct TextLayout {
    shaped_lines: Vec<Line>,
    wrap_boundaries: Vec<Vec<ShapedBoundary>>,
    line_height: f32,
}

fn optional_add<T>(a: Option<T>, b: Option<T>) -> Option<T::Output>
where
    T: Add<Output = T>,
{
    match (a, b) {
        (Some(a), Some(b)) => Some(a + b),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

trait Vector2FExt {
    fn infinity() -> Self;
    fn get(self, axis: Axis2d) -> f32;
    fn set(&mut self, axis: Axis2d, value: f32);
    fn increment_x(&mut self, delta: f32) -> f32;
    fn increment_y(&mut self, delta: f32) -> f32;
}

impl Vector2FExt for Vector2F {
    fn infinity() -> Self {
        vec2f(f32::INFINITY, f32::INFINITY)
    }

    fn get(self, axis: Axis2d) -> f32 {
        match axis {
            Axis2d::X => self.x(),
            Axis2d::Y => self.y(),
        }
    }

    fn set(&mut self, axis: Axis2d, value: f32) {
        match axis {
            Axis2d::X => self.set_x(value),
            Axis2d::Y => self.set_y(value),
        }
    }

    fn increment_x(&mut self, delta: f32) -> f32 {
        self.set_x(self.x() + delta);
        self.x()
    }

    fn increment_y(&mut self, delta: f32) -> f32 {
        self.set_y(self.y() + delta);
        self.y()
    }
}

trait ElementExt<V: View> {
    fn margin_left(self, margin_left: impl Into<Length>) -> Node<V>
    where
        Self: Element<V> + Sized,
    {
        node(self).margin_left(margin_left)
    }
}

impl<V, E> ElementExt<V> for E
where
    V: View,
    E: Element<V>,
{
    fn margin_left(self, margin_left: impl Into<Length>) -> Node<V>
    where
        Self: Sized,
    {
        node(self).margin_left(margin_left)
    }
}
