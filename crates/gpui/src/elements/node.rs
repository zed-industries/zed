use super::layout_highlighted_chunks;
use crate::{
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
use derive_more::Add;
use length::{Length, Rems};
use log::warn;
use optional_struct::*;
use std::{any::Any, borrow::Cow, f32, ops::Range, sync::Arc};

pub struct Node<V: View> {
    style: NodeStyle,
    content: Vec<AnyElement<V>>,
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
        content: Default::default(),
    }
}

pub fn stack<V: View>() -> Node<V> {
    Node {
        style: NodeStyle {
            axis: Axis3d::Z,
            ..Default::default()
        },
        content: Default::default(),
    }
}

impl<V: View> Default for Node<V> {
    fn default() -> Self {
        Self {
            style: Default::default(),
            content: Default::default(),
        }
    }
}

impl<V: View> Node<V> {
    pub fn child(mut self, child: impl Element<V>) -> Self {
        self.content.push(child.into_any());
        self
    }

    pub fn children<I, E>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Element<V>,
    {
        self.content
            .extend(children.into_iter().map(|child| child.into_any()));
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.style.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.style.height = height.into();
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
        self.style.margin = Edges {
            top: top_bottom.top,
            bottom: top_bottom.bottom,
            left: left_right.left,
            right: left_right.right,
        };
        self
    }

    pub fn margin_top(mut self, top: Length) -> Self {
        self.style.margin.top = top;
        self
    }

    pub fn margin_bottom(mut self, bottom: Length) -> Self {
        self.style.margin.bottom = bottom;
        self
    }

    pub fn margin_left(mut self, left: Length) -> Self {
        self.style.margin.left = left;
        self
    }

    pub fn margin_right(mut self, right: Length) -> Self {
        self.style.margin.right = right;
        self
    }

    fn layout_2d_children(
        &mut self,
        axis: Axis2d,
        size: Vector2F,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> Vector2F {
        let mut total_flex: Option<f32> = None;
        let mut total_size = 0.0;
        let mut cross_axis_max: f32 = 0.0;

        // First pass: Layout non-flex children only
        for child in &mut self.content {
            let child_flex = child.metadata::<NodeStyle>().and_then(|style| match axis {
                Axis2d::X => style.width.flex(),
                Axis2d::Y => style.height.flex(),
            });

            if let Some(child_flex) = child_flex {
                *total_flex.get_or_insert(0.) += child_flex;
            } else {
                match axis {
                    Axis2d::X => {
                        let child_constraint =
                            SizeConstraint::new(Vector2F::zero(), vec2f(f32::INFINITY, size.y()));
                        let child_size = child.layout(child_constraint, view, cx);
                        cross_axis_max = cross_axis_max.max(child_size.y());
                        total_size += child_size.x();
                    }
                    Axis2d::Y => {
                        let child_constraint =
                            SizeConstraint::new(Vector2F::zero(), vec2f(size.x(), f32::INFINITY));
                        let child_size = child.layout(child_constraint, view, cx);
                        cross_axis_max = cross_axis_max.max(child_size.x());
                        total_size += child_size.y();
                    }
                }
            }
        }

        let remaining_space = match axis {
            Axis2d::X => size.x() - total_size,
            Axis2d::Y => size.y() - total_size,
        };

        // Second pass: Layout flexible children
        if let Some(total_flex) = total_flex {
            if total_flex > 0. {
                let space_per_flex = remaining_space.max(0.) / total_flex;

                for child in &mut self.content {
                    let child_flex = child.metadata::<NodeStyle>().and_then(|style| match axis {
                        Axis2d::X => style.width.flex(),
                        Axis2d::Y => style.height.flex(),
                    });
                    if let Some(child_flex) = child_flex {
                        let child_max = space_per_flex * child_flex;
                        let mut child_constraint = SizeConstraint::new(Vector2F::zero(), size);
                        match axis {
                            Axis2d::X => {
                                child_constraint.min.set_x(0.0);
                                child_constraint.max.set_x(child_max);
                            }
                            Axis2d::Y => {
                                child_constraint.min.set_y(0.0);
                                child_constraint.max.set_y(child_max);
                            }
                        }

                        let child_size = child.layout(child_constraint, view, cx);
                        cross_axis_max = match axis {
                            Axis2d::X => {
                                total_size += child_size.x();
                                cross_axis_max.max(child_size.y())
                            }
                            Axis2d::Y => {
                                total_size += child_size.y();
                                cross_axis_max.max(child_size.x())
                            }
                        };
                    }
                }
            }
        }

        let size = match axis {
            Axis2d::X => vec2f(total_size, cross_axis_max),
            Axis2d::Y => vec2f(cross_axis_max, total_size),
        };
        size
    }

    fn paint_2d_children(
        &mut self,
        scene: &mut SceneBuilder,
        axis: Axis2d,
        bounds: RectF,
        visible_bounds: RectF,
        size_of_children: &mut Vector2F,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) {
        let parent_size = bounds.size();
        let mut child_origin = bounds.origin();

        // Align all children together along the primary axis
        let mut align_horizontally = false;
        let mut align_vertically = false;
        match axis {
            Axis2d::X => align_horizontally = true,
            Axis2d::Y => align_vertically = true,
        }
        align_child(
            &mut child_origin,
            parent_size,
            *size_of_children,
            self.style.align.0,
            align_horizontally,
            align_vertically,
        );

        for child in &mut self.content {
            // Align each child along the cross axis
            align_horizontally = !align_horizontally;
            align_vertically = !align_vertically;
            align_child(
                &mut child_origin,
                parent_size,
                child.size(),
                self.style.align.0,
                align_horizontally,
                align_vertically,
            );

            child.paint(scene, child_origin, visible_bounds, view, cx);

            // Advance along the primary axis by the size of this child
            match axis {
                Axis2d::X => child_origin.set_x(child_origin.x() + child.size().x()),
                Axis2d::Y => child_origin.set_y(child_origin.y() + child.size().y()),
            }
        }
    }

    // fn layout_stacked_children(
    //     &mut self,
    //     constraint: SizeConstraint,
    //     view: &mut V,
    //     cx: &mut LayoutContext<V>,
    // ) -> Vector2F {
    //     let mut size = Vector2F::zero();

    //     for child in &mut self.children {
    //         let child_size = child.layout(constraint, view, cx);
    //         size.set_x(size.x().max(child_size.x()));
    //         size.set_y(size.y().max(child_size.y()));
    //     }

    //     size
    // }

    fn inset_size(&self, rem_size: f32) -> Vector2F {
        self.padding_size(rem_size) + self.border_size() + self.margin_size(rem_size)
    }

    fn margin_size(&self, rem_size: f32) -> Vector2F {
        // We need to account for auto margins
        todo!()
        // vec2f(
        //     (self.style.margin.left + self.style.margin.right).to_pixels(rem_size),
        //     (self.style.margin.top + self.style.margin.bottom).to_pixels(rem_size),
        // )
    }

    fn padding_size(&self, rem_size: f32) -> Vector2F {
        // We need to account for auto padding
        todo!()
        // vec2f(
        //     (self.style.padding.left + self.style.padding.right).to_pixels(rem_size),
        //     (self.style.padding.top + self.style.padding.bottom).to_pixels(rem_size),
        // )
    }

    fn border_size(&self) -> Vector2F {
        let mut x = 0.0;
        if self.style.border.left {
            x += self.style.border.width;
        }
        if self.style.border.right {
            x += self.style.border.width;
        }

        let mut y = 0.0;
        if self.style.border.top {
            y += self.style.border.width;
        }
        if self.style.border.bottom {
            y += self.style.border.width;
        }

        vec2f(x, y)
    }
}

impl<V: View> Element<V> for Node<V> {
    type LayoutState = Vector2F; // Content size
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let mut size = Vector2F::zero();
        let rem_size = cx.pixels_per_rem();
        let margin_size = self.margin_size(rem_size);
        match self.style.width {
            Length::Hug => size.set_x(f32::INFINITY),
            Length::Fixed(width) => size.set_x(width.to_pixels(rem_size) + margin_size.x()),
            Length::Auto { min, max, .. } => size.set_x(constraint.max.x().max(min).min(max)),
        }
        match self.style.height {
            Length::Hug => size.set_y(f32::INFINITY),
            Length::Fixed(height) => size.set_y(height.to_pixels(rem_size) + margin_size.y()),
            Length::Auto { min, max, .. } => size.set_y(constraint.max.y().max(min).min(max)),
        }

        // Impose horizontal constraints
        if constraint.min.x().is_finite() {
            size.set_x(size.x().max(constraint.min.x()));
        }
        size.set_x(size.x().min(constraint.max.x()));

        // Impose vertical constraints
        if constraint.min.y().is_finite() {
            size.set_y(size.y().max(constraint.min.y()));
        }
        size.set_y(size.y().min(constraint.max.y()));

        let inset_size = self.inset_size(rem_size);
        let inner_size = size - inset_size;
        let size_of_children = match self.style.axis {
            Axis3d::X => self.layout_2d_children(Axis2d::X, inner_size, view, cx),
            Axis3d::Y => self.layout_2d_children(Axis2d::Y, inner_size, view, cx),
            Axis3d::Z => todo!(), // self.layout_stacked_children(inner_constraint, view, cx),
        };

        if matches!(self.style.width, Length::Hug) {
            size.set_x(size_of_children.x() + inset_size.x());
        }
        if matches!(self.style.height, Length::Hug) {
            size.set_y(size_of_children.y() + inset_size.y());
        }

        (size, size_of_children)
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        size_of_children: &mut Vector2F,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Self::PaintState {
        let rem_size = cx.pixels_per_rem();
        let margin: Edges<f32> = todo!(); // &self.style.margin.to_pixels(rem_size);

        // Account for margins
        let content_bounds = RectF::from_points(
            bounds.origin() + vec2f(margin.left, margin.top),
            bounds.lower_right() - vec2f(margin.right, margin.bottom),
        );

        // Paint drop shadow
        for shadow in &self.style.shadows {
            scene.push_shadow(scene::Shadow {
                bounds: content_bounds + shadow.offset,
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

        // Render the background and/or the border (if it not an overlay border).
        let Fill::Color(fill_color) = self.style.fill;
        let is_fill_visible = !fill_color.is_fully_transparent();
        if is_fill_visible || self.style.border.is_visible() {
            scene.push_quad(Quad {
                bounds: content_bounds,
                background: is_fill_visible.then_some(fill_color),
                border: scene::Border {
                    width: self.style.border.width,
                    color: self.style.border.color,
                    overlay: false,
                    top: self.style.border.top,
                    right: self.style.border.right,
                    bottom: self.style.border.bottom,
                    left: self.style.border.left,
                },
                corner_radius: self.style.corner_radius,
            });
        }

        if !self.content.is_empty() {
            // Account for padding first.
            let padding: Edges<f32> = todo!(); // &self.style.padding.to_pixels(rem_size);
            let padded_bounds = RectF::from_points(
                content_bounds.origin() + vec2f(padding.left, padding.top),
                content_bounds.lower_right() - vec2f(padding.right, padding.top),
            );

            match self.style.axis {
                Axis3d::X => self.paint_2d_children(
                    scene,
                    Axis2d::X,
                    padded_bounds,
                    visible_bounds,
                    size_of_children,
                    view,
                    cx,
                ),
                Axis3d::Y => self.paint_2d_children(
                    scene,
                    Axis2d::Y,
                    padded_bounds,
                    visible_bounds,
                    size_of_children,
                    view,
                    cx,
                ),
                Axis3d::Z => todo!(),
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
        self.content
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

    width: Length,
    height: Length,
    margin: Edges<Length>,
    padding: Edges<Length>,
    text: OptionalTextStyle,
    opacity: f32,
    fill: Fill,
    border: Border,
    corner_radius: f32, // corner radius matches swift!
    shadows: Vec<Shadow>,
}

#[optional_struct]
struct TextStyle {
    size: Rems,
    font_family: Arc<str>,
    weight: FontWeight,
    style: FontStyle,
}

#[derive(Add)]
struct Size<T> {
    width: T,
    height: T,
}

// Sides?
#[derive(Clone, Default)]
struct Edges<T> {
    top: T,
    bottom: T,
    left: T,
    right: T,
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
struct Border {
    color: Color,
    width: f32,
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

impl Border {
    fn is_visible(&self) -> bool {
        self.width > 0.
            && !self.color.is_fully_transparent()
            && (self.top || self.bottom || self.left || self.right)
    }
}

pub mod length {
    use derive_more::{Add, Into};

    #[derive(Add, Into, Clone, Copy, Default, Debug, PartialEq)]
    pub struct Rems(f32);

    pub fn rems(rems: f32) -> Rems {
        Rems(rems)
    }

    impl Rems {
        pub fn to_pixels(&self, root_font_size: f32) -> f32 {
            self.0 * root_font_size
        }
    }

    #[derive(Clone, Copy, Default, Debug)]
    pub enum Length {
        #[default]
        Hug,
        Fixed(Rems),
        Auto {
            flex: f32,
            min: f32,
            max: f32,
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
            min: 0.,
            max: f32::INFINITY,
        }
    }

    pub fn constrained(flex: f32, min: Option<f32>, max: Option<f32>) -> Length {
        Length::Auto {
            flex,
            min: min.unwrap_or(0.),
            max: max.unwrap_or(f32::INFINITY),
        }
    }

    impl Length {
        pub fn flex(&self) -> Option<f32> {
            match self {
                Length::Auto { flex, .. } => Some(*flex),
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

#[derive(Clone, Copy, Default)]
enum Axis2d {
    X,
    #[default]
    Y,
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
