use super::{
    AbsoluteLength, Bounds, DefiniteLength, Edges, Layout, Length, Pixels, Point, Result, Size,
    Style,
};
use gpui2::taffy::{self, Taffy};
use std::fmt::Debug;

pub use gpui2::taffy::tree::NodeId as LayoutId;
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
        todo!()
    }
}

trait ToTaffy {
    type Output;

    fn to_taffy(&self, rem_size: Pixels) -> Self::Output;
}

impl ToTaffy for Style {
    type Output = taffy::style::Style;

    fn to_taffy(&self, rem_size: Pixels) -> Self::Output {
        todo!()
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

impl ToTaffy for Length {
    type Output = taffy::style::LengthPercentageAuto;

    fn to_taffy(&self, rem_size: Pixels) -> taffy::prelude::LengthPercentageAuto {
        match self {
            Length::Definite(length) => length.to_taffy(rem_size).into(),
            Length::Auto => taffy::prelude::LengthPercentageAuto::Auto,
        }
    }
}

impl ToTaffy for DefiniteLength {
    type Output = taffy::style::LengthPercentage;

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

impl ToTaffy for AbsoluteLength {
    type Output = taffy::style::LengthPercentage;

    fn to_taffy(&self, rem_size: Pixels) -> Self::Output {
        match self {
            AbsoluteLength::Pixels(pixels) => taffy::style::LengthPercentage::Length(pixels.into()),
            AbsoluteLength::Rems(rems) => {
                taffy::style::LengthPercentage::Length((*rems * rem_size).into())
            }
        }
    }
}

impl<T, T2> From<taffy::geometry::Point<T>> for Point<T2>
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

impl<T, T2> Into<taffy::geometry::Point<T2>> for Point<T>
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

impl<T: ToTaffy + Clone> ToTaffy for Size<T> {
    type Output = taffy::geometry::Size<T::Output>;

    fn to_taffy(&self, rem_size: Pixels) -> Self::Output {
        taffy::geometry::Size {
            width: self.width.to_taffy(rem_size).into(),
            height: self.height.to_taffy(rem_size).into(),
        }
    }
}

impl<T: ToTaffy + Clone> ToTaffy for Edges<T> {
    type Output = taffy::geometry::Rect<T::Output>;

    fn to_taffy(&self, rem_size: Pixels) -> Self::Output {
        taffy::geometry::Rect {
            top: self.top.to_taffy(rem_size),
            right: self.right.to_taffy(rem_size),
            bottom: self.bottom.to_taffy(rem_size),
            left: self.left.to_taffy(rem_size),
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
        Self(pixels)
    }
}
