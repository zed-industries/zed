use playground_macros::tailwind_lengths;
use taffy::style::{Position, *};

#[derive(Clone, Debug)]
struct Style {
    display: Display,
    position: Position,
    overflow: Point<Overflow>,
    inset: Edges<Length>,
}

#[derive(Clone, Copy, Debug)]
pub enum Length {
    Rems(f32),
    Pixels(f32),
    Percent(f32),
    Auto,
}

impl Default for Length {
    fn default() -> Self {
        Self::Rems(0.)
    }
}

#[derive(Clone, Default, Debug)]
pub struct Edges<T> {
    top: T,
    bottom: T,
    left: T,
    right: T,
}

#[derive(Clone, Copy, Debug)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl Style {
    // Display ////////////////////

    fn block(mut self) -> Self {
        self.display = Display::Block;
        self
    }

    fn flex(mut self) -> Self {
        self.display = Display::Flex;
        self
    }

    fn grid(mut self) -> Self {
        self.display = Display::Grid;
        self
    }

    // Overflow ///////////////////

    pub fn overflow_visible(mut self) -> Self {
        self.overflow.x = Overflow::Visible;
        self.overflow.y = Overflow::Visible;
        self
    }

    pub fn overflow_hidden(mut self) -> Self {
        self.overflow.x = Overflow::Hidden;
        self.overflow.y = Overflow::Hidden;
        self
    }

    pub fn overflow_scroll(mut self) -> Self {
        self.overflow.x = Overflow::Scroll;
        self.overflow.y = Overflow::Scroll;
        self
    }

    pub fn overflow_x_visible(mut self) -> Self {
        self.overflow.x = Overflow::Visible;
        self
    }

    pub fn overflow_x_hidden(mut self) -> Self {
        self.overflow.x = Overflow::Hidden;
        self
    }

    pub fn overflow_x_scroll(mut self) -> Self {
        self.overflow.x = Overflow::Scroll;
        self
    }

    pub fn overflow_y_visible(mut self) -> Self {
        self.overflow.y = Overflow::Visible;
        self
    }

    pub fn overflow_y_hidden(mut self) -> Self {
        self.overflow.y = Overflow::Hidden;
        self
    }

    pub fn overflow_y_scroll(mut self) -> Self {
        self.overflow.y = Overflow::Scroll;
        self
    }

    // Position ///////////////////

    pub fn relative(mut self) -> Self {
        self.position = Position::Relative;
        self
    }

    pub fn absolute(mut self) -> Self {
        self.position = Position::Absolute;

        self
    }

    #[tailwind_lengths]
    pub fn inset(mut self, length: Length) -> Self {
        self.inset.top = length;
        self.inset.right = length;
        self.inset.bottom = length;
        self.inset.left = length;
        self
    }
}

// mod traits {
//     pub trait Element<V> {
//         type Layout;

//         /// Add the element and its children to the layout tree.
//         fn layout(&mut self, &view)

//         // fn layout(&self) ->
//     }
// }

// #![allow(unused_variables, dead_code)]

// use self::length::rems;
// use crate::color::Hsla;
// use derive_more::{Add, Deref, DerefMut};
// use gpui::{
//     color::Color,
//     fonts::{HighlightStyle, Underline},
//     geometry::{
//         rect::RectF,
//         vector::{vec2f, Vector2F},
//     },
//     text_layout::{Line, ShapedBoundary},
//     AppContext, Entity, LayoutContext, SceneBuilder, View, ViewContext, WindowContext,
// };
// use length::{Length, Rems};
// use optional_struct::*;
// use std::{any::Any, borrow::Cow, f32, ops::Range, sync::Arc};

// mod traits {
//     use super::*;

//     pub trait Element<V> {
//         type Paint;

//         fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> (NodeId, Self::Paint);
//         fn paint(
//             &mut self,
//             view: &mut V,
//             layout: &Layout,
//             paint: &mut Self::Paint,
//             cx: &mut PaintContext<V>,
//         );

//         /// Convert this element into a dynamically-typed element.
//         fn into_any(self) -> super::AnyElement<V>
//         where
//             Self: Sized,
//         {
//             super::AnyElement {
//                 element: Box::new(self),
//                 paint: None,
//             }
//         }
//     }

//     pub trait AnyElement<V> {
//         fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> (NodeId, Box<dyn Any>);
//         fn paint(
//             &mut self,
//             view: &mut V,
//             layout: &Layout,
//             paint: &mut dyn Any,
//             cx: &mut PaintContext<V>,
//         );
//     }

//     impl<E, V> AnyElement<V> for E
//     where
//         E: Element<V>,
//     {
//         fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> (NodeId, Box<dyn Any>) {
//             let (node_id, paint_state) = self.layout(view, cx);
//             (node_id, Box::new(paint_state))
//         }

//         fn paint(
//             &mut self,
//             view: &mut V,
//             layout: &Layout,
//             paint: &mut dyn Any,
//             cx: &mut PaintContext<V>,
//         ) {
//             let paint = paint.downcast_mut().unwrap();
//             self.paint(view, layout, paint, cx);
//         }
//     }
// }

// /// A dynamically-typed element.
// struct AnyElement<V> {
//     // An element as a trait object.
//     element: Box<dyn traits::AnyElement<V>>,
//     // Data computed during layout that is used during paint.
//     paint: Option<Box<dyn Any>>,
// }

// impl<V> AnyElement<V> {
//     pub fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> NodeId {
//         let (id, paint_data) = self.element.layout(view, cx);
//         self.paint = Some(paint_data);
//         id
//     }

//     pub fn paint(&mut self, view: &mut V, cx: &mut PaintContext<V>) {
//         let layout = cx.layout_engine().layout(node)
//         let paint = self
//             .paint
//             .as_mut()
//             .expect("called paint before calling layout");
//         self.element.paint(view, paint, cx);
//     }
// }

// // impl<V> traits::AnyElement<V> for AnyElement<V> {
// //     fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> NodeId {
// //         self.element.layout(view, cx)
// //     }

// //     fn paint(&mut self, view: &mut V, cx: &mut PaintContext<V>) {
// //         self.element.paint(view, cx)
// //     }
// // }

// // impl<E, V> traits::AnyElement<V> for E
// // where
// //     E: Element<V>,
// // {
// //     fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> NodeId {
// //         todo!()
// //     }

// //     fn paint(&mut self, view: &mut V, cx: &mut PaintContext<V>) {
// //         todo!()
// //     }
// // }

// /// A basic stylable element, akin to a `div` in HTML.
// pub struct Frame<V> {
//     style: FrameStyle,
//     metadata: FrameMetadata,
//     children: Vec<AnyElement<V>>,
//     id: Option<Cow<'static, str>>,
//     before_paint: Option<Box<dyn FnMut(RectF, &mut (), &mut PaintContext<V>)>>,
// }

// /// The final result of the layout engine for a single node.
// struct Layout {
//     /// The relative ordering of the node
//     ///
//     /// Nodes with a higher order should be rendered on top of those with a lower order.
//     /// This is effectively a topological sort of each tree.
//     pub order: u32,
//     /// The position and size of the element in the layout.
//     pub bounds: RectF,
// }

// type NodeId = taffy::prelude::NodeId;

// #[derive(Deref, DerefMut)]
// pub struct PaintContext<'a, 'b, 'c, V>(gpui::PaintContext<'a, 'b, 'c, V>);

// impl<'a, 'b, 'c, V> PaintContext<'a, 'b, 'c, V> {
//     fn set_stroke_style(&mut self, stroke: Stroke) {}
//     fn stroke_rect(&mut self, rect: RectF, edges: Edges<bool>) {}

//     fn set_fill_style(&mut self, fill: Fill) {}
//     fn fill_rect(&mut self, rect: RectF) {}
// }

// impl<V: 'static> traits::Element<V> for Frame<V> {
//     type Paint = ();

//     fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> (NodeId, ()) {
//         let children_ids: Vec<_> = self.children.iter_mut().map(|child| todo!()).collect();
//         let node_id = cx
//             .layout_engine()
//             .new_with_children(self.style.layout.clone(), &children_ids)
//             .unwrap();
//         (node_id, ())
//     }

//     fn paint(&mut self, view: &mut V, layout: &Layout, paint: &mut (), cx: &mut PaintContext<V>) {
//         // Fill the rectangle with the background color
//         cx.set_fill_style(self.style.fill);
//         cx.fill_rect(layout.bounds);

//         // Paint child elements
//         for child in &mut self.children {
//             child.paint(view, cx);
//         }

//         // Applying border style if present
//         if self.style.borders.is_visible() {
//             cx.set_stroke_style(self.style.borders.stroke);
//             cx.stroke_rect(layout.bounds, self.style.borders.edges()); // Draw the border
//         }
//     }
// }

// impl<V: 'static> Frame<V> {
//     pub fn id(mut self, id: impl Into<Cow<'static, str>>) -> Self {
//         self.id = Some(id.into());
//         self
//     }

//     pub fn child(mut self, child: impl traits::Element<V>) -> Self {
//         self.children.push(child.into_any());
//         self
//     }

//     pub fn children<I, E>(mut self, children: I) -> Self
//     where
//         I: IntoIterator<Item = E>,
//         E: traits::Element<V>,
//     {
//         self.children
//             .extend(children.into_iter().map(|child| child.into_any()));
//         self
//     }

//     pub fn fill(mut self, fill: impl Into<Fill>) -> Self {
//         self.style.fill = fill.into();
//         self
//     }

//     pub fn text_size(mut self, text_size: Rems) -> Self {
//         self.style.text.size = Some(text_size);
//         self
//     }

//     pub fn text_color(mut self, color: Hsla) -> Self {
//         self.style.text.color = Some(color);
//         self
//     }

//     fn before_paint<H>(mut self, handler: H) -> Self
//     where
//         H: 'static + FnMut(RectF, &mut (), &mut PaintContext<V>),
//     {
//         self.before_paint = Some(Box::new(handler));
//         self
//     }
// }

// pub struct TopBottom {
//     top: Length,
//     bottom: Length,
// }

// impl<T: Into<Length>> From<(T, T)> for TopBottom {
//     fn from((top, bottom): (T, T)) -> Self {
//         Self {
//             top: top.into(),
//             bottom: bottom.into(),
//         }
//     }
// }

// impl<T: Copy + Into<Length>> From<T> for TopBottom {
//     fn from(both: T) -> Self {
//         Self {
//             top: both.into(),
//             bottom: both.into(),
//         }
//     }
// }

// pub struct LeftRight {
//     left: Length,
//     right: Length,
// }

// impl From<(Length, Length)> for LeftRight {
//     fn from((left, right): (Length, Length)) -> Self {
//         Self { left, right }
//     }
// }

// impl From<Length> for LeftRight {
//     fn from(both: Length) -> Self {
//         Self {
//             left: both,
//             right: both,
//         }
//     }
// }

// struct Interactive<Style> {
//     default: Style,
//     hovered: Style,
//     active: Style,
//     disabled: Style,
// }

// #[derive(Default)]
// pub struct FrameMetadata {
//     layout_node_id: Option<taffy::tree::NodeId>,
// }

// #[derive(Clone, Default)]
// pub struct FrameStyle {
//     text: OptionalTextStyle,
//     opacity: f32,
//     fill: Fill,
//     borders: Borders,
//     corner_radius: f32,
//     shadows: Vec<Shadow>,
//     layout: taffy::style::Style,
// }

// #[optional_struct]
// struct TextStyle {
//     size: Rems,
//     font_family: Arc<str>,
//     weight: FontWeight,
//     style: FontStyle,
//     color: Hsla,
//     soft_wrap: bool,
//     underline: Underline,
// }

// impl TextStyle {
//     fn from_legacy(text_style: &gpui::fonts::TextStyle, _cx: &WindowContext) -> Self {
//         Self {
//             size: rems(text_style.font_size / 16.), // TODO: Get this from the context!
//             font_family: text_style.font_family_name.clone(),
//             weight: text_style.font_properties.weight.into(),
//             style: text_style.font_properties.style.into(),
//             color: text_style.color.into(),
//             soft_wrap: text_style.soft_wrap,
//             underline: text_style.underline,
//         }
//     }

//     fn to_legacy(&self, cx: &WindowContext) -> anyhow::Result<gpui::fonts::TextStyle> {
//         let font_family_id = cx.font_cache().load_family(
//             &[self.font_family.as_ref()],
//             &gpui::fonts::Features::default(),
//         )?;
//         let font_properties = gpui::fonts::Properties {
//             style: self.style.into(),
//             weight: self.weight.into(),
//             stretch: Default::default(),
//         };
//         let font_id = cx
//             .font_cache()
//             .select_font(font_family_id, &font_properties)?;

//         Ok(gpui::fonts::TextStyle {
//             color: self.color.into(),
//             font_family_name: self.font_family.clone(),
//             font_family_id,
//             font_id,
//             font_size: self.size.to_pixels(16.), // TODO: Get this from the context!
//             font_properties,
//             underline: self.underline,
//             soft_wrap: self.soft_wrap,
//         })
//     }
// }

// impl OptionalTextStyle {
//     pub fn is_some(&self) -> bool {
//         self.size.is_some()
//             || self.font_family.is_some()
//             || self.weight.is_some()
//             || self.style.is_some()
//             || self.color.is_some()
//     }
// }

// // pub color: Color,
// // pub font_family_name: Arc<str>,
// // pub font_family_id: FamilyId,
// // pub font_id: FontId,
// // pub font_size: f32,
// // #[schemars(with = "PropertiesDef")]
// // pub font_properties: Properties,
// // pub underline: Underline,
// // pub soft_wrap: bool,

// #[derive(Add, Default, Clone)]
// pub struct Size<T> {
//     width: T,
//     height: T,
// }

// impl<T: Copy> Size<T> {
//     fn get(&self, axis: Axis2d) -> T {
//         match axis {
//             Axis2d::X => self.width,
//             Axis2d::Y => self.height,
//         }
//     }
// }

// impl Size<Length> {
//     fn fixed_pixels(&self, rem_pixels: f32) -> Size<f32> {
//         Size {
//             width: self.width.fixed_pixels(rem_pixels),
//             height: self.height.fixed_pixels(rem_pixels),
//         }
//     }

//     pub fn fixed(&self) -> Size<Rems> {
//         Size {
//             width: self.width.fixed().unwrap_or_default(),
//             height: self.height.fixed().unwrap_or_default(),
//         }
//     }

//     pub fn flex(&self) -> Vector2F {
//         vec2f(
//             self.width.flex().unwrap_or(0.),
//             self.height.flex().unwrap_or(0.),
//         )
//     }
// }

// impl Size<Rems> {
//     pub fn to_pixels(&self, rem_size: f32) -> Vector2F {
//         vec2f(
//             self.width.to_pixels(rem_size),
//             self.height.to_pixels(rem_size),
//         )
//     }
// }

// impl From<Length> for Size<Length> {
//     fn from(value: Length) -> Self {
//         Self {
//             width: value,
//             height: value,
//         }
//     }
// }

// impl<T> Edges<T> {
//     fn start(&self, axis: Axis2d) -> &T {
//         match axis {
//             Axis2d::X => &self.left,
//             Axis2d::Y => &self.top,
//         }
//     }

//     fn start_mut(&mut self, axis: Axis2d) -> &mut T {
//         match axis {
//             Axis2d::X => &mut self.left,
//             Axis2d::Y => &mut self.top,
//         }
//     }

//     fn end(&self, axis: Axis2d) -> &T {
//         match axis {
//             Axis2d::X => &self.right,
//             Axis2d::Y => &self.bottom,
//         }
//     }

//     fn end_mut(&mut self, axis: Axis2d) -> &mut T {
//         match axis {
//             Axis2d::X => &mut self.right,
//             Axis2d::Y => &mut self.bottom,
//         }
//     }
// }

// impl<T: Clone> Edges<T> {
//     pub fn set_x(&mut self, value: T) {
//         self.left = value.clone();
//         self.right = value
//     }

//     pub fn set_y(&mut self, value: T) {
//         self.top = value.clone();
//         self.bottom = value
//     }
// }

// impl Edges<f32> {
//     fn size(&self) -> Vector2F {
//         vec2f(self.left + self.right, self.top + self.bottom)
//     }

//     fn compute_flex_edges(
//         &mut self,
//         style_edges: &Edges<Length>,
//         axis: Axis2d,
//         remaining_flex: &mut f32,
//         remaining_length: &mut f32,
//         rem_pixels: f32,
//     ) {
//         *self.start_mut(axis) +=
//             style_edges
//                 .start(axis)
//                 .flex_pixels(rem_pixels, remaining_flex, remaining_length);
//         *self.end_mut(axis) +=
//             style_edges
//                 .end(axis)
//                 .flex_pixels(rem_pixels, remaining_flex, remaining_length);
//     }
// }

// impl Edges<Length> {
//     fn fixed_pixels(&self, rem_pixels: f32) -> Edges<f32> {
//         Edges {
//             top: self.top.fixed_pixels(rem_pixels),
//             bottom: self.bottom.fixed_pixels(rem_pixels),
//             left: self.left.fixed_pixels(rem_pixels),
//             right: self.right.fixed_pixels(rem_pixels),
//         }
//     }

//     fn flex_pixels(
//         &self,
//         rem_pixels: f32,
//         remaining_flex: &mut f32,
//         remaining_length: &mut f32,
//     ) -> Edges<f32> {
//         Edges {
//             top: self
//                 .top
//                 .flex_pixels(rem_pixels, remaining_flex, remaining_length),
//             bottom: self
//                 .bottom
//                 .flex_pixels(rem_pixels, remaining_flex, remaining_length),
//             left: self
//                 .left
//                 .flex_pixels(rem_pixels, remaining_flex, remaining_length),
//             right: self
//                 .right
//                 .flex_pixels(rem_pixels, remaining_flex, remaining_length),
//         }
//     }

//     // pub fn fixed(&self) -> Size<Rems> {
//     //     let mut size = Size::default();
//     //     size.width += self.left.fixed().unwrap_or_default();
//     //     size.width += self.right.fixed().unwrap_or_default();
//     //     size
//     // }

//     pub fn flex(&self) -> Vector2F {
//         vec2f(
//             self.left.flex().unwrap_or(0.) + self.right.flex().unwrap_or(0.),
//             self.top.flex().unwrap_or(0.) + self.bottom.flex().unwrap_or(0.),
//         )
//     }
// }

// impl Edges<Rems> {
//     pub fn to_pixels(&self, rem_size: f32) -> Edges<f32> {
//         Edges {
//             top: self.top.to_pixels(rem_size),
//             bottom: self.bottom.to_pixels(rem_size),
//             left: self.left.to_pixels(rem_size),
//             right: self.right.to_pixels(rem_size),
//         }
//     }
// }

// impl<L> From<L> for Edges<Length>
// where
//     L: Into<Length>,
// {
//     fn from(uniform: L) -> Self {
//         let uniform = uniform.into();
//         Edges {
//             top: uniform,
//             bottom: uniform,
//             left: uniform,
//             right: uniform,
//         }
//     }
// }

// impl<Vertical, Horizontal> From<(Vertical, Horizontal)> for Edges<Length>
// where
//     Vertical: Into<Length>,
//     Horizontal: Into<Length>,
// {
//     fn from((vertical, horizontal): (Vertical, Horizontal)) -> Self {
//         let vertical = vertical.into();
//         let horizontal = horizontal.into();
//         Edges {
//             top: vertical,
//             bottom: vertical,
//             left: horizontal,
//             right: horizontal,
//         }
//     }
// }

// impl<Top, Bottom, Left, Right> From<(Top, Bottom, Left, Right)> for Edges<Length>
// where
//     Top: Into<Length>,
//     Bottom: Into<Length>,
//     Left: Into<Length>,
//     Right: Into<Length>,
// {
//     fn from((top, bottom, left, right): (Top, Bottom, Left, Right)) -> Self {
//         Edges {
//             top: top.into(),
//             bottom: bottom.into(),
//             left: left.into(),
//             right: right.into(),
//         }
//     }
// }

// struct CornerRadii {
//     top_left: f32,
//     top_right: f32,
//     bottom_right: f32,
//     bottom_left: f32,
// }

// #[derive(Clone)]
// pub enum Fill {
//     Color(Hsla),
// }

// #[derive(Copy, Clone, Default)]
// pub struct Stroke {
//     pub width: f32,
//     pub color: Hsla,
// }

// impl Stroke {
//     fn is_visible(&self) -> bool {
//         self.width > 0. && self.color.a > 0.
//     }
// }

// impl<C: Into<Hsla>> From<C> for Fill {
//     fn from(value: C) -> Self {
//         Fill::Color(value.into())
//     }
// }

// impl Default for Fill {
//     fn default() -> Self {
//         Fill::Color(Hsla::default())
//     }
// }

// #[derive(Clone, Default)]
// pub struct Borders {
//     stroke: Stroke,
//     top: bool,
//     bottom: bool,
//     left: bool,
//     right: bool,
// }

// impl Borders {
//     pub fn is_visible(&self) -> bool {
//         self.stroke.is_visible() && (self.top || self.bottom || self.left || self.right)
//     }

//     pub fn edges(&self) -> Edges<bool> {
//         Edges {
//             top: self.top,
//             bottom: self.bottom,
//             left: self.left,
//             right: self.right,
//         }
//     }

//     pub fn top_width(&self) -> f32 {
//         if self.top {
//             self.stroke.width
//         } else {
//             0.
//         }
//     }

//     pub fn bottom_width(&self) -> f32 {
//         if self.bottom {
//             self.stroke.width
//         } else {
//             0.
//         }
//     }

//     pub fn left_width(&self) -> f32 {
//         if self.left {
//             self.stroke.width
//         } else {
//             0.
//         }
//     }

//     pub fn right_width(&self) -> f32 {
//         if self.right {
//             self.stroke.width
//         } else {
//             0.
//         }
//     }

//     pub fn edge_widths(&self) -> Edges<f32> {
//         let mut edges = Edges::default();
//         if self.stroke.width > 0. {
//             if self.top {
//                 edges.top = self.stroke.width;
//             }
//             if self.bottom {
//                 edges.bottom = self.stroke.width;
//             }
//             if self.left {
//                 edges.left = self.stroke.width;
//             }
//             if self.right {
//                 edges.right = self.stroke.width;
//             }
//         }
//         edges
//     }

//     pub fn size(&self) -> Vector2F {
//         let width = if self.left { self.stroke.width } else { 0. }
//             + if self.right { self.stroke.width } else { 0. };
//         let height = if self.top { self.stroke.width } else { 0. }
//             + if self.bottom { self.stroke.width } else { 0. };

//         vec2f(width, height)
//     }
// }

// pub mod length {
//     use derive_more::{Add, AddAssign, Into};

//     #[derive(Add, AddAssign, Into, Clone, Copy, Default, Debug, PartialEq)]
//     pub struct Rems(f32);

//     pub fn rems(rems: f32) -> Rems {
//         Rems(rems)
//     }

//     impl Rems {
//         pub fn to_pixels(&self, rem_pixels: f32) -> f32 {
//             self.0 * rem_pixels
//         }
//     }

//     #[derive(Clone, Copy, Default, Debug)]
//     pub enum Length {
//         #[default]
//         Hug,
//         Fixed(Rems),
//         Auto {
//             flex: f32,
//             min: Rems,
//             max: Rems,
//         },
//     }

//     impl From<Rems> for Length {
//         fn from(value: Rems) -> Self {
//             Length::Fixed(value)
//         }
//     }

//     pub fn auto() -> Length {
//         flex(1.)
//     }

//     pub fn flex(flex: f32) -> Length {
//         Length::Auto {
//             flex,
//             min: Default::default(),
//             max: rems(f32::INFINITY),
//         }
//     }

//     pub fn constrained(flex: f32, min: Option<Rems>, max: Option<Rems>) -> Length {
//         Length::Auto {
//             flex,
//             min: min.unwrap_or(Default::default()),
//             max: max.unwrap_or(rems(f32::INFINITY)),
//         }
//     }

//     impl Length {
//         pub fn flex_pixels(
//             &self,
//             rem_pixels: f32,
//             remaining_flex: &mut f32,
//             remaining_length: &mut f32,
//         ) -> f32 {
//             match self {
//                 Length::Auto { flex, min, max } => {
//                     let flex_length = *remaining_length / *remaining_flex;
//                     let length = (flex * flex_length)
//                         .clamp(min.to_pixels(rem_pixels), max.to_pixels(rem_pixels));
//                     *remaining_flex -= flex;
//                     *remaining_length -= length;
//                     length
//                 }
//                 _ => 0.,
//             }
//         }

//         pub fn fixed_pixels(&self, rem: f32) -> f32 {
//             match self {
//                 Length::Fixed(rems) => rems.to_pixels(rem),
//                 _ => 0.,
//             }
//         }

//         pub fn flex(&self) -> Option<f32> {
//             match self {
//                 Length::Auto { flex, .. } => Some(*flex),
//                 _ => None,
//             }
//         }

//         pub fn fixed(&self) -> Option<Rems> {
//             match self {
//                 Length::Fixed(rems) => Some(*rems),
//                 _ => None,
//             }
//         }
//     }
// }

// #[derive(Clone, Deref, DerefMut)]
// struct Alignment(Vector2F);

// impl Default for Alignment {
//     fn default() -> Self {
//         Self(vec2f(-1., -1.))
//     }
// }

// #[derive(Clone, Copy, Default)]
// enum Axis3d {
//     X,
//     #[default]
//     Y,
//     Z,
// }

// impl Axis3d {
//     fn to_2d(self) -> Option<Axis2d> {
//         match self {
//             Axis3d::X => Some(Axis2d::X),
//             Axis3d::Y => Some(Axis2d::Y),
//             Axis3d::Z => None,
//         }
//     }
// }

// #[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
// pub enum Axis2d {
//     X,
//     #[default]
//     Y,
// }

// impl Axis2d {
//     fn rotate(self) -> Self {
//         match self {
//             Axis2d::X => Axis2d::Y,
//             Axis2d::Y => Axis2d::X,
//         }
//     }
// }

// #[derive(Clone, Copy, Default)]
// enum Overflow {
//     #[default]
//     Visible,
//     Hidden,
//     Auto,
// }

// #[derive(Clone, Copy)]
// enum Gap {
//     Fixed(f32),
//     Around,
//     Between,
//     Even,
// }

// impl Default for Gap {
//     fn default() -> Self {
//         Gap::Fixed(0.)
//     }
// }

// #[derive(Clone, Copy, Default)]
// struct Shadow {
//     offset: Vector2F,
//     blur: f32,
//     color: Color,
// }

// #[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
// enum FontStyle {
//     #[default]
//     Normal,
//     Italic,
//     Oblique,
// }

// impl From<gpui::fonts::Style> for FontStyle {
//     fn from(value: gpui::fonts::Style) -> Self {
//         use gpui::fonts::Style;

//         match value {
//             Style::Normal => FontStyle::Normal,
//             Style::Italic => FontStyle::Italic,
//             Style::Oblique => FontStyle::Oblique,
//         }
//     }
// }

// impl Into<gpui::fonts::Style> for FontStyle {
//     fn into(self) -> gpui::fonts::Style {
//         use gpui::fonts::Style;

//         match self {
//             FontStyle::Normal => Style::Normal,
//             FontStyle::Italic => Style::Italic,
//             FontStyle::Oblique => Style::Oblique,
//         }
//     }
// }

// #[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
// enum FontWeight {
//     Thin,
//     ExtraLight,
//     Light,
//     #[default]
//     Normal,
//     Medium,
//     Semibold,
//     Bold,
//     ExtraBold,
//     Black,
// }

// impl From<gpui::fonts::Weight> for FontWeight {
//     fn from(value: gpui::fonts::Weight) -> Self {
//         use gpui::fonts::Weight;

//         if value == Weight::THIN {
//             FontWeight::Thin
//         } else if value == Weight::EXTRA_LIGHT {
//             FontWeight::ExtraLight
//         } else if value == Weight::LIGHT {
//             FontWeight::Light
//         } else if value == Weight::NORMAL {
//             FontWeight::Normal
//         } else if value == Weight::MEDIUM {
//             FontWeight::Medium
//         } else if value == Weight::SEMIBOLD {
//             FontWeight::Semibold
//         } else if value == Weight::BOLD {
//             FontWeight::Bold
//         } else if value == Weight::EXTRA_BOLD {
//             FontWeight::ExtraBold
//         } else if value == Weight::BLACK {
//             FontWeight::Black
//         } else {
//             panic!("unknown font weight: {:?}", value);
//         }
//     }
// }

// impl Into<gpui::fonts::Weight> for FontWeight {
//     fn into(self) -> gpui::fonts::Weight {
//         use gpui::fonts::Weight;

//         match self {
//             FontWeight::Thin => Weight::THIN,
//             FontWeight::ExtraLight => Weight::EXTRA_LIGHT,
//             FontWeight::Light => Weight::LIGHT,
//             FontWeight::Normal => Weight::NORMAL,
//             FontWeight::Medium => Weight::MEDIUM,
//             FontWeight::Semibold => Weight::SEMIBOLD,
//             FontWeight::Bold => Weight::BOLD,
//             FontWeight::ExtraBold => Weight::EXTRA_BOLD,
//             FontWeight::Black => Weight::BLACK,
//         }
//     }
// }

// #[derive(Default)]
// pub struct Text {
//     text: Cow<'static, str>,
//     highlights: Option<Box<[(Range<usize>, HighlightStyle)]>>,
//     custom_runs: Option<(
//         Box<[Range<usize>]>,
//         Box<dyn FnMut(usize, RectF, &mut SceneBuilder, &mut AppContext)>,
//     )>,
// }

// pub struct TextLayout {
//     shaped_lines: Vec<Line>,
//     wrap_boundaries: Vec<Vec<ShapedBoundary>>,
//     line_height: f32,
// }

// trait Vector2FExt {
//     fn infinity() -> Self;
//     fn get(self, axis: Axis2d) -> f32;
//     fn set(&mut self, axis: Axis2d, value: f32);
//     fn increment_x(&mut self, delta: f32) -> f32;
//     fn increment_y(&mut self, delta: f32) -> f32;
// }

// impl Vector2FExt for Vector2F {
//     fn infinity() -> Self {
//         vec2f(f32::INFINITY, f32::INFINITY)
//     }

//     fn get(self, axis: Axis2d) -> f32 {
//         match axis {
//             Axis2d::X => self.x(),
//             Axis2d::Y => self.y(),
//         }
//     }

//     fn set(&mut self, axis: Axis2d, value: f32) {
//         match axis {
//             Axis2d::X => self.set_x(value),
//             Axis2d::Y => self.set_y(value),
//         }
//     }

//     fn increment_x(&mut self, delta: f32) -> f32 {
//         self.set_x(self.x() + delta);
//         self.x()
//     }

//     fn increment_y(&mut self, delta: f32) -> f32 {
//         self.set_y(self.y() + delta);
//         self.y()
//     }
// }

// // pub fn view<F, E>(mut render: F) -> ViewFn
// // where
// //     F: 'static + FnMut(&mut ViewContext<ViewFn>) -> E,
// //     E: traits::Element<ViewFn>,
// // {
// //     ViewFn(Box::new(move |cx| (render)(cx).into_any()))
// // }

// // pub struct ViewFn(Box<dyn FnMut(&mut ViewContext<ViewFn>) -> AnyElement<ViewFn>>);

// // impl Entity for ViewFn {
// //     type Event = ();
// // }

// // impl View for ViewFn {
// //     fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
// //         (self.0)(cx)
// //     }
// // }

// #[cfg(test)]
// mod tests {
//     use gpui::TestAppContext;

//     #[gpui::test]
//     fn test_frame_layout(cx: &mut TestAppContext) {
//         // cx.add_window(|_| {
//         // view(|_| {
//         //     let theme = rose_pine::dawn();
//         //     column()
//         //         .width(auto())
//         //         .height(auto())
//         //         .justify(1.)
//         //         .child(
//         //             row()
//         //                 .width(auto())
//         //                 .height(rems(10.))
//         //                 .justify(1.)
//         //                 .child(
//         //                     row()
//         //                         .width(rems(10.))
//         //                         .height(auto())
//         //                         .fill(theme.surface(1.)),
//         //                 )
//         //                 .before_paint(|bounds, layout, cx| {
//         //                     assert_eq!(bounds.origin(), vec2f(0., 0.));
//         //                     assert_eq!(layout.size.x(), cx.window_size().x());
//         //                     assert_eq!(layout.size.y(), rems(10.).to_pixels(cx.rem_pixels()));
//         //                 }),
//         //         )
//         //         .child(row())
//         //         .before_paint(|bounds, layout, cx| {
//         //             assert_eq!(layout.size, cx.window_size());
//         //         })
//         // })
//         // })
//         // .remove(cx);
//     }
// }
