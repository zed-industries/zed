#![allow(unused_variables, dead_code)]

use derive_more::{Add, Deref, DerefMut};
use gpui::{
    color::Color,
    elements::layout_highlighted_chunks,
    fonts::HighlightStyle,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{json, ToJson},
    scene,
    serde_json::Value,
    text_layout::{Line, ShapedBoundary},
    AnyElement, AppContext, Element, Entity, LayoutContext, PaintContext, Quad, SceneBuilder,
    SizeConstraint, View, ViewContext, WindowContext,
};
use length::{Length, Rems};
use log::warn;
use optional_struct::*;
use std::{any::Any, borrow::Cow, f32, ops::Range, sync::Arc};

use crate::color::{Hsla, Rgba};

use self::length::rems;

pub struct Frame<V> {
    style: FrameStyle,
    children: Vec<AnyElement<V>>,
    id: Option<Cow<'static, str>>,
    before_paint: Option<Box<dyn FnMut(RectF, &mut FrameLayout, &mut PaintContext<V>)>>,
}

pub fn column<V>() -> Frame<V> {
    Frame::default()
}

pub fn row<V>() -> Frame<V> {
    Frame {
        style: FrameStyle {
            axis: Axis3d::X,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn stack<V>() -> Frame<V> {
    Frame {
        style: FrameStyle {
            axis: Axis3d::Z,
            ..Default::default()
        },
        ..Default::default()
    }
}

impl<V> Default for Frame<V> {
    fn default() -> Self {
        Self {
            style: Default::default(),
            children: Default::default(),
            id: None,
            before_paint: None,
        }
    }
}

impl<V: 'static> Element<V> for Frame<V> {
    type LayoutState = FrameLayout;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        if self.style.text.is_some() {
            let mut style = TextStyle::from_legacy(&cx.text_style(), cx);
            self.style.text.clone().apply_to(&mut style);
            cx.push_text_style(style.to_legacy());
        }

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
        layout: &mut FrameLayout,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Self::PaintState {
        if let Some(before_paint) = &mut self.before_paint {
            before_paint(bounds, layout, cx);
        }

        let bounds_center = bounds.size() / 2.;
        let bounds_target = bounds_center + (bounds_center * self.style.align.0);
        let layout_center = layout.size / 2.;
        let layout_target = layout_center + layout_center * self.style.align.0;
        let delta = bounds_target - layout_target;

        let aligned_bounds = RectF::new(bounds.origin() + delta, layout.size);
        let margined_bounds = RectF::from_points(
            aligned_bounds.origin() + vec2f(layout.margins.left, layout.margins.top),
            aligned_bounds.lower_right() - vec2f(layout.margins.right, layout.margins.bottom),
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
        let is_fill_visible = fill_color.a > 0.;
        if is_fill_visible || self.style.borders.is_visible() {
            scene.push_quad(Quad {
                bounds: margined_bounds,
                background: is_fill_visible.then_some(fill_color.into()),
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

                for child in &mut self.children {
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
            "type": "Frame",
            "bounds": bounds.to_json(),
            // TODO!
            // "children": self.content.iter().map(|child| child.debug(view, cx)).collect::<Vec<Value>>()
        })
    }

    fn metadata(&self) -> Option<&dyn Any> {
        Some(&self.style)
    }
}

impl<V: 'static> Frame<V> {
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

    pub fn size(self, size: impl Into<Size<Length>>) -> Self {
        let size = size.into();
        self.width(size.width).height(size.height)
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

    pub fn text_color(mut self, color: Hsla) -> Self {
        self.style.text.color = Some(color);
        self
    }

    pub fn margins(mut self, margins: impl Into<Edges<Length>>) -> Self {
        self.style.margins = margins.into();
        self
    }

    pub fn margin_x(mut self, margin: impl Into<Length>) -> Self {
        self.style.margins.set_x(margin.into());
        self
    }

    pub fn margin_y(mut self, margin: impl Into<Length>) -> Self {
        self.style.margins.set_y(margin.into());
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

    pub fn align(mut self, alignment: f32) -> Self {
        let cross_axis = self
            .style
            .axis
            .to_2d()
            .map(Axis2d::rotate)
            .unwrap_or(Axis2d::Y);
        self.style.align.set(cross_axis, alignment);
        self
    }

    pub fn justify(mut self, alignment: f32) -> Self {
        let axis = self.style.axis.to_2d().unwrap_or(Axis2d::X);
        self.style.align.set(axis, alignment);
        self
    }

    fn id_string(&self) -> String {
        self.id.as_deref().unwrap_or("<anonymous>").to_string()
    }

    fn layout_xy(
        &mut self,
        primary_axis: Axis2d,
        constraint: SizeConstraint,
        rem_pixels: f32,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> FrameLayout {
        self.style.text.is_some();

        let cross_axis = primary_axis.rotate();
        let total_flex = self.style.flex();
        let mut layout = FrameLayout {
            size: Default::default(),
            padding: self.style.padding.fixed_pixels(rem_pixels),
            margins: self.style.margins.fixed_pixels(rem_pixels),
            borders: self.style.borders.edges(),
        };
        let fixed_padding_size = layout.padding.size();
        let fixed_margin_size = layout.margins.size();
        let borders_size = layout.borders.size();
        let fixed_constraint = constraint - fixed_margin_size - borders_size - fixed_padding_size;

        // Determine the child constraints in each dimension based on the styled size
        let mut child_constraint = SizeConstraint::default();
        for axis in [Axis2d::X, Axis2d::Y] {
            let length = self.style.size.get(axis);
            let content_length = match length {
                Length::Hug => {
                    // Tell the children not to expand
                    0.
                }
                Length::Fixed(fixed_length) => {
                    // Tell the children to expand up to the fixed length minus the padding.
                    fixed_length.to_pixels(rem_pixels) - fixed_padding_size.get(axis)
                }
                Length::Auto { .. } => {
                    // Tell the children to expand to fill their share of the flex space in this node.
                    length.flex_pixels(
                        rem_pixels,
                        &mut total_flex.get(axis),
                        &mut fixed_constraint.max.get(axis),
                    )
                }
            };
            child_constraint.max.set(axis, content_length);
            if axis == cross_axis {
                child_constraint.min.set(axis, content_length);
            }
        }

        // Lay out inflexible children. Total up flex of flexible children for
        // use in a second pass.
        let mut remaining_length = child_constraint.max.get(primary_axis);
        let mut remaining_flex = 0.;
        let mut total_length = 0.;
        let mut cross_axis_max: f32 = 0.;

        for child in &mut self.children {
            if let Some(child_flex) = child
                .metadata::<FrameStyle>()
                .map(|style| style.flex().get(primary_axis))
            {
                if child_flex > 0. {
                    remaining_flex += child_flex;
                    continue;
                }
            }

            let child_size = child.layout(child_constraint, view, cx);
            let child_length = child_size.get(primary_axis);
            remaining_length -= child_length;
            total_length += child_length;
            cross_axis_max = cross_axis_max.max(child_size.get(cross_axis));
        }

        // Distribute the remaining length among the flexible children.
        for child in &mut self.children {
            if let Some(child_flex) = child
                .metadata::<FrameStyle>()
                .map(|style| style.flex().get(primary_axis))
            {
                if child_flex > 0. {
                    let max_child_length = (child_flex / remaining_flex) * remaining_length;
                    child_constraint.max.set(primary_axis, max_child_length);

                    let child_size = child.layout(child_constraint, view, cx);
                    let child_length = child_size.get(primary_axis);
                    total_length += child_length;
                    remaining_length -= child_length;
                    remaining_flex -= child_flex;
                    cross_axis_max = cross_axis_max.max(child_size.get(cross_axis));
                }
            }
        }

        let content_size = match primary_axis {
            Axis2d::X => vec2f(total_length, cross_axis_max),
            Axis2d::Y => vec2f(cross_axis_max, total_length),
        };

        // Distribute remaining space to flexible padding and margins.
        for axis in [Axis2d::X, Axis2d::Y] {
            let length = self.style.size.get(axis);
            match length {
                Length::Hug => {
                    let mut remaining_flex = total_flex.get(axis);
                    let mut remaining_length =
                        fixed_constraint.min.get(axis) - content_size.get(axis);

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
                Length::Fixed(fixed_length) => {
                    let fixed_length = fixed_length.to_pixels(rem_pixels);

                    // With a fixed length, we can only distribute the space in the fixed-length container
                    // not consumed by the content.
                    let mut padding_flex = self.style.padding.flex().get(axis);
                    let mut max_padding_length = (fixed_length - content_size.get(axis)).max(0.);
                    layout.padding.compute_flex_edges(
                        &self.style.padding,
                        axis,
                        &mut padding_flex,
                        &mut max_padding_length,
                        rem_pixels,
                    );

                    // Similarly, distribute the available space for margins so we preserve the fixed length
                    // of the container.
                    let mut margin_flex = self.style.margins.flex().get(axis);
                    let mut max_margin_length = constraint.max.get(axis) - fixed_length;
                    layout.margins.compute_flex_edges(
                        &self.style.margins,
                        axis,
                        &mut margin_flex,
                        &mut max_margin_length,
                        rem_pixels,
                    );

                    layout
                        .size
                        .set(axis, fixed_length + layout.margins.size().get(axis))
                }
                Length::Auto { .. } => {
                    let mut remaining_flex = total_flex.get(axis);
                    let mut remaining_length = fixed_constraint.max.get(axis);
                    let flex_length =
                        length.flex_pixels(rem_pixels, &mut remaining_flex, &mut remaining_length);

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
                        flex_length
                            + layout.padding.size().get(axis)
                            + layout.borders.size().get(axis)
                            + layout.margins.size().get(axis),
                    )
                }
            }
        }

        layout
    }

    fn before_paint<H>(mut self, handler: H) -> Self
    where
        H: 'static + FnMut(RectF, &mut FrameLayout, &mut PaintContext<V>),
    {
        self.before_paint = Some(Box::new(handler));
        self
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

struct Interactive<Style> {
    default: Style,
    hovered: Style,
    active: Style,
    disabled: Style,
}

#[derive(Clone, Default)]
pub struct FrameStyle {
    axis: Axis3d,
    wrap: bool,
    align: Alignment,
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

impl FrameStyle {
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
    color: Hsla,
}

impl TextStyle {
    fn from_legacy(text_style: &gpui::fonts::TextStyle, _cx: &WindowContext) -> Self {
        Self {
            size: rems(text_style.font_size / 16.), // TODO: Get this from the context!
            font_family: text_style.font_family_name.clone(),
            weight: text_style.font_properties.weight.into(),
            style: text_style.font_properties.style.into(),
            color: text_style.color.into(),
        }
    }

    fn to_legacy(&self, cx: &WindowContext) -> Result<gpui::fonts::TextStyle> {
        let font_family_id = cx.font_cache().load_family(
            &[self.font_family.as_ref()],
            &gpui::fonts::Features::default(),
        )?;
        let font_properties = gpui::fonts::Properties {
            style: self.style.into(),
            weight: self.weight.into(),
            stretch: Default::default(),
        };
        let font_id = cx
            .font_cache()
            .select_font(font_family_id, &font_properties);

        Ok(gpui::fonts::TextStyle {
            color: self.color.into(),
            font_family_name: self.font_family.clone(),
            font_family_id,
            font_id,
            font_size: todo!(),
            font_properties,
            underline: todo!(),
            soft_wrap: true,
        })
    }
}

impl OptionalTextStyle {
    pub fn is_some(&self) -> bool {
        self.size.is_some()
            && self.font_family.is_some()
            && self.weight.is_some()
            && self.style.is_some()
            && self.color.is_some()
    }
}

// pub color: Color,
// pub font_family_name: Arc<str>,
// pub font_family_id: FamilyId,
// pub font_id: FontId,
// pub font_size: f32,
// #[schemars(with = "PropertiesDef")]
// pub font_properties: Properties,
// pub underline: Underline,
// pub soft_wrap: bool,

#[derive(Add, Default, Clone)]
pub struct Size<T> {
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

impl From<Length> for Size<Length> {
    fn from(value: Length) -> Self {
        Self {
            width: value,
            height: value,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Edges<T> {
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

impl<T: Clone> Edges<T> {
    pub fn set_x(&mut self, value: T) {
        self.left = value.clone();
        self.right = value
    }

    pub fn set_y(&mut self, value: T) {
        self.top = value.clone();
        self.bottom = value
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

impl<L> From<L> for Edges<Length>
where
    L: Into<Length>,
{
    fn from(uniform: L) -> Self {
        let uniform = uniform.into();
        Edges {
            top: uniform,
            bottom: uniform,
            left: uniform,
            right: uniform,
        }
    }
}

impl<Vertical, Horizontal> From<(Vertical, Horizontal)> for Edges<Length>
where
    Vertical: Into<Length>,
    Horizontal: Into<Length>,
{
    fn from((vertical, horizontal): (Vertical, Horizontal)) -> Self {
        let vertical = vertical.into();
        let horizontal = horizontal.into();
        Edges {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }
}

impl<Top, Bottom, Left, Right> From<(Top, Bottom, Left, Right)> for Edges<Length>
where
    Top: Into<Length>,
    Bottom: Into<Length>,
    Left: Into<Length>,
    Right: Into<Length>,
{
    fn from((top, bottom, left, right): (Top, Bottom, Left, Right)) -> Self {
        Edges {
            top: top.into(),
            bottom: bottom.into(),
            left: left.into(),
            right: right.into(),
        }
    }
}

struct CornerRadii {
    top_left: f32,
    top_right: f32,
    bottom_right: f32,
    bottom_left: f32,
}

#[derive(Clone)]
pub enum Fill {
    Color(Rgba),
}

impl<C: Into<Rgba>> From<C> for Fill {
    fn from(value: C) -> Self {
        Fill::Color(value.into())
    }
}

impl Default for Fill {
    fn default() -> Self {
        Fill::Color(Rgba::default())
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

#[derive(Clone, Deref, DerefMut)]
struct Alignment(Vector2F);

impl Default for Alignment {
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

impl From<gpui::fonts::Style> for FontStyle {
    fn from(value: gpui::fonts::Style) -> Self {
        use gpui::fonts::Style;

        match value {
            Style::Normal => FontStyle::Normal,
            Style::Italic => FontStyle::Italic,
            Style::Oblique => FontStyle::Oblique,
        }
    }
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

impl From<gpui::fonts::Weight> for FontWeight {
    fn from(value: gpui::fonts::Weight) -> Self {
        use gpui::fonts::Weight;

        match value {
            Weight::THIN => FontWeight::Thin,
            Weight::EXTRA_LIGHT => FontWeight::ExtraLight,
            Weight::LIGHT => FontWeight::Light,
            Weight::NORMAL => FontWeight::Normal,
            Weight::MEDIUM => FontWeight::Medium,
            Weight::SEMIBOLD => FontWeight::Semibold,
            Weight::BOLD => FontWeight::Bold,
            Weight::EXTRA_BOLD => FontWeight::ExtraBold,
            Weight::BLACK => FontWeight::Black,
        }
    }
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

pub fn text<V: 'static>(text: impl Into<Cow<'static, str>>) -> Frame<V> {
    row().child(Text {
        text: text.into(),
        ..Default::default()
    })
}

#[derive(Default, Debug)]
pub struct FrameLayout {
    size: Vector2F,
    padding: Edges<f32>,
    borders: Edges<f32>,
    margins: Edges<f32>,
}

impl<V: 'static> Element<V> for Text {
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

trait ElementExt<V: 'static> {
    fn margin_left(self, margin_left: impl Into<Length>) -> Frame<V>
    where
        Self: Element<V> + Sized,
    {
        column().child(self).margin_left(margin_left)
    }
}

impl<V, E> ElementExt<V> for E
where
    V: 'static,
    E: Element<V>,
{
    fn margin_left(self, margin_left: impl Into<Length>) -> Frame<V>
    where
        Self: Sized,
    {
        column().child(self).margin_left(margin_left)
    }
}

pub fn view<F, E>(mut render: F) -> ViewFn
where
    F: 'static + FnMut(&mut ViewContext<ViewFn>) -> E,
    E: Element<ViewFn>,
{
    ViewFn(Box::new(move |cx| (render)(cx).into_any()))
}

pub struct ViewFn(Box<dyn FnMut(&mut ViewContext<ViewFn>) -> AnyElement<ViewFn>>);

impl Entity for ViewFn {
    type Event = ();
}

impl View for ViewFn {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        (self.0)(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        length::{auto, rems},
        *,
    };
    use crate::themes::rose_pine;
    use gpui::TestAppContext;

    #[gpui::test]
    fn test_frame_layout(cx: &mut TestAppContext) {
        cx.add_window(|_| {
            view(|_| {
                let theme = rose_pine::dawn();
                column()
                    .width(auto())
                    .height(auto())
                    .justify(1.)
                    .child(
                        row()
                            .width(auto())
                            .height(rems(10.))
                            .justify(1.)
                            .child(
                                row()
                                    .width(rems(10.))
                                    .height(auto())
                                    .fill(theme.surface(1.)),
                            )
                            .before_paint(|bounds, layout, cx| {
                                assert_eq!(bounds.origin(), vec2f(0., 0.));
                                assert_eq!(layout.size.x(), cx.window_size().x());
                                assert_eq!(layout.size.y(), rems(10.).to_pixels(cx.rem_pixels()));
                            }),
                    )
                    .child(row())
                    .before_paint(|bounds, layout, cx| {
                        assert_eq!(layout.size, cx.window_size());
                    })
            })
        })
        .remove(cx);
    }
}
