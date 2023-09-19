use super::{
    AbsoluteLength, Bounds, DefiniteLength, Edges, Layout, Length, Pixels, Point, Result, Size,
    Style,
};
use std::fmt::Debug;
pub use taffy::tree::NodeId as LayoutId;
pub use taffy::*;
pub struct TaffyLayoutEngine(Taffy);

impl TaffyLayoutEngine {
    pub fn new() -> Self {
        TaffyLayoutEngine(Taffy::new())
    }

    pub fn request_layout(
        &mut self,
        style: Style,
        rem_size: Pixels,
        children: &[LayoutId],
    ) -> Result<LayoutId> {
        let style = style.to_taffy(rem_size);
        if children.is_empty() {
            Ok(self.0.new_leaf(style)?)
        } else {
            Ok(self.0.new_with_children(style, children)?)
        }
    }

    pub fn layout(&mut self, id: LayoutId) -> Result<Layout> {
        Ok(self.0.layout(id).map(Into::into)?)
    }
}

trait ToTaffy<Output> {
    fn to_taffy(&self, rem_size: Pixels) -> Output;
}

impl ToTaffy<taffy::style::Style> for Style {
    fn to_taffy(&self, rem_size: Pixels) -> taffy::style::Style {
        taffy::style::Style {
            display: self.display,
            overflow: self.overflow.clone().into(),
            scrollbar_width: self.scrollbar_width,
            position: self.position,
            inset: self.inset.to_taffy(rem_size),
            size: self.size.to_taffy(rem_size),
            min_size: self.min_size.to_taffy(rem_size),
            max_size: self.max_size.to_taffy(rem_size),
            aspect_ratio: self.aspect_ratio,
            margin: self.margin.to_taffy(rem_size),
            padding: self.padding.to_taffy(rem_size),
            border: self.border_widths.to_taffy(rem_size),
            align_items: self.align_items,
            align_self: self.align_self,
            align_content: self.align_content,
            justify_content: self.justify_content,
            gap: self.gap.to_taffy(rem_size),
            flex_direction: self.flex_direction,
            flex_wrap: self.flex_wrap,
            flex_basis: self.flex_basis.to_taffy(rem_size),
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            ..Default::default() // Ignore grid properties for now
        }
    }
}

// impl ToTaffy for Bounds<Length> {
//     type Output = taffy::prelude::Bounds<taffy::prelude::LengthPercentageAuto>;

//     fn to_taffy(
//         &self,
//         rem_size: Pixels,
//     ) -> taffy::prelude::Bounds<taffy::prelude::LengthPercentageAuto> {
//         taffy::prelude::Bounds {
//             origin: self.origin.to_taffy(rem_size),
//             size: self.size.to_taffy(rem_size),
//         }
//     }
// }

impl ToTaffy<taffy::style::LengthPercentageAuto> for Length {
    fn to_taffy(&self, rem_size: Pixels) -> taffy::prelude::LengthPercentageAuto {
        match self {
            Length::Definite(length) => length.to_taffy(rem_size),
            Length::Auto => taffy::prelude::LengthPercentageAuto::Auto,
        }
    }
}

impl ToTaffy<taffy::style::Dimension> for Length {
    fn to_taffy(&self, rem_size: Pixels) -> taffy::prelude::Dimension {
        match self {
            Length::Definite(length) => length.to_taffy(rem_size),
            Length::Auto => taffy::prelude::Dimension::Auto,
        }
    }
}

impl ToTaffy<taffy::style::LengthPercentage> for DefiniteLength {
    fn to_taffy(&self, rem_size: Pixels) -> taffy::style::LengthPercentage {
        match self {
            DefiniteLength::Absolute(length) => match length {
                AbsoluteLength::Pixels(pixels) => {
                    taffy::style::LengthPercentage::Length(pixels.into())
                }
                AbsoluteLength::Rems(rems) => {
                    taffy::style::LengthPercentage::Length((*rems * rem_size).into())
                }
            },
            DefiniteLength::Fraction(fraction) => {
                taffy::style::LengthPercentage::Percent(*fraction)
            }
        }
    }
}

impl ToTaffy<taffy::style::LengthPercentageAuto> for DefiniteLength {
    fn to_taffy(&self, rem_size: Pixels) -> taffy::style::LengthPercentageAuto {
        match self {
            DefiniteLength::Absolute(length) => match length {
                AbsoluteLength::Pixels(pixels) => {
                    taffy::style::LengthPercentageAuto::Length(pixels.into())
                }
                AbsoluteLength::Rems(rems) => {
                    taffy::style::LengthPercentageAuto::Length((*rems * rem_size).into())
                }
            },
            DefiniteLength::Fraction(fraction) => {
                taffy::style::LengthPercentageAuto::Percent(*fraction)
            }
        }
    }
}

impl ToTaffy<taffy::style::Dimension> for DefiniteLength {
    fn to_taffy(&self, rem_size: Pixels) -> taffy::style::Dimension {
        match self {
            DefiniteLength::Absolute(length) => match length {
                AbsoluteLength::Pixels(pixels) => taffy::style::Dimension::Length(pixels.into()),
                AbsoluteLength::Rems(rems) => {
                    taffy::style::Dimension::Length((*rems * rem_size).into())
                }
            },
            DefiniteLength::Fraction(fraction) => taffy::style::Dimension::Percent(*fraction),
        }
    }
}

impl ToTaffy<taffy::style::LengthPercentage> for AbsoluteLength {
    fn to_taffy(&self, rem_size: Pixels) -> taffy::style::LengthPercentage {
        match self {
            AbsoluteLength::Pixels(pixels) => taffy::style::LengthPercentage::Length(pixels.into()),
            AbsoluteLength::Rems(rems) => {
                taffy::style::LengthPercentage::Length((*rems * rem_size).into())
            }
        }
    }
}

impl<T, T2: Clone + Debug> From<taffy::geometry::Point<T>> for Point<T2>
where
    T: Into<T2>,
{
    fn from(point: taffy::geometry::Point<T>) -> Point<T2> {
        Point {
            x: point.x.into(),
            y: point.y.into(),
        }
    }
}

impl<T: Clone + Debug, T2> Into<taffy::geometry::Point<T2>> for Point<T>
where
    T: Into<T2>,
{
    fn into(self) -> taffy::geometry::Point<T2> {
        taffy::geometry::Point {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
}

impl<T: ToTaffy<U> + Clone + Debug, U> ToTaffy<taffy::geometry::Size<U>> for Size<T> {
    fn to_taffy(&self, rem_size: Pixels) -> taffy::geometry::Size<U> {
        taffy::geometry::Size {
            width: self.width.to_taffy(rem_size).into(),
            height: self.height.to_taffy(rem_size).into(),
        }
    }
}

impl<T, U> ToTaffy<taffy::geometry::Rect<U>> for Edges<T>
where
    T: ToTaffy<U> + Clone + Debug,
{
    fn to_taffy(&self, rem_size: Pixels) -> taffy::geometry::Rect<U> {
        taffy::geometry::Rect {
            top: self.top.to_taffy(rem_size).into(),
            right: self.right.to_taffy(rem_size).into(),
            bottom: self.bottom.to_taffy(rem_size).into(),
            left: self.left.to_taffy(rem_size).into(),
        }
    }
}

impl<S, T: Clone + Default + Debug> From<taffy::geometry::Size<S>> for Size<T>
where
    S: Into<T>,
{
    fn from(value: taffy::geometry::Size<S>) -> Self {
        Self {
            width: value.width.into(),
            height: value.height.into(),
        }
    }
}

impl From<&taffy::tree::Layout> for Layout {
    fn from(layout: &taffy::tree::Layout) -> Self {
        Layout {
            order: layout.order,
            bounds: Bounds {
                origin: layout.location.into(),
                size: layout.size.into(),
            },
        }
    }
}

impl From<f32> for Pixels {
    fn from(pixels: f32) -> Self {
        Pixels(pixels)
    }
}
