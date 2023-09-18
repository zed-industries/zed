use bytemuck::{Pod, Zeroable};
use core::fmt::Debug;
use derive_more::{Add, AddAssign, Div, Mul, Sub};
use refineable::Refineable;
use std::ops::Mul;

#[derive(Refineable, Default, Add, AddAssign, Sub, Mul, Div, Copy, Debug, PartialEq, Eq, Hash)]
#[refineable(debug)]
#[repr(C)]
pub struct Point<T: Clone + Debug> {
    pub x: T,
    pub y: T,
}

impl<T: Clone + Debug> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Clone + Debug> Clone for Point<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

unsafe impl<T: Clone + Debug + Zeroable + Pod> Zeroable for Point<T> {}

unsafe impl<T: Clone + Debug + Zeroable + Pod> Pod for Point<T> {}

#[derive(Refineable, Default, Clone, Copy, Debug)]
#[refineable(debug)]
pub struct Size<T: Clone + Debug> {
    pub width: T,
    pub height: T,
}

impl Size<Length> {
    pub fn full() -> Self {
        Self {
            width: relative(1.),
            height: relative(1.),
        }
    }
}

impl Size<DefiniteLength> {
    pub fn zero() -> Self {
        Self {
            width: px(0.),
            height: px(0.),
        }
    }
}

impl Size<Length> {
    pub fn auto() -> Self {
        Self {
            width: Length::Auto,
            height: Length::Auto,
        }
    }
}

#[derive(Refineable, Clone, Default, Debug)]
#[refineable(debug)]
pub struct Bounds<T: Clone + Debug> {
    pub origin: Point<T>,
    pub size: Size<T>,
}

impl<T: Clone + Debug + Copy> Copy for Bounds<T> {}

#[derive(Refineable, Clone, Default, Debug)]
#[refineable(debug)]
pub struct Edges<T: Clone + Debug> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl Edges<Length> {
    pub fn auto() -> Self {
        Self {
            top: Length::Auto,
            right: Length::Auto,
            bottom: Length::Auto,
            left: Length::Auto,
        }
    }

    pub fn zero() -> Self {
        Self {
            top: px(0.),
            right: px(0.),
            bottom: px(0.),
            left: px(0.),
        }
    }
}

impl Edges<DefiniteLength> {
    pub fn zero() -> Self {
        Self {
            top: px(0.),
            right: px(0.),
            bottom: px(0.),
            left: px(0.),
        }
    }
}

impl Edges<AbsoluteLength> {
    pub fn zero() -> Self {
        Self {
            top: px(0.),
            right: px(0.),
            bottom: px(0.),
            left: px(0.),
        }
    }

    pub fn to_pixels(&self, rem_size: Pixels) -> Edges<Pixels> {
        Edges {
            top: self.top.to_pixels(rem_size),
            right: self.right.to_pixels(rem_size),
            bottom: self.bottom.to_pixels(rem_size),
            left: self.left.to_pixels(rem_size),
        }
    }
}

impl Edges<Pixels> {
    pub fn is_empty(&self) -> bool {
        self.top == px(0.) && self.right == px(0.) && self.bottom == px(0.) && self.left == px(0.)
    }
}

#[derive(Clone, Copy, Default, Add, AddAssign, Sub, Mul, Div, PartialEq)]
#[repr(transparent)]
pub struct Pixels(pub(crate) f32);

unsafe impl bytemuck::Pod for Pixels {}
unsafe impl bytemuck::Zeroable for Pixels {}

impl Debug for Pixels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} px", self.0)
    }
}

impl From<Pixels> for f32 {
    fn from(pixels: Pixels) -> Self {
        pixels.0
    }
}

impl From<&Pixels> for f32 {
    fn from(pixels: &Pixels) -> Self {
        pixels.0
    }
}

#[derive(Clone, Copy, Default, Add, Sub, Mul, Div)]
pub struct Rems(f32);

impl Mul<Pixels> for Rems {
    type Output = Pixels;

    fn mul(self, other: Pixels) -> Pixels {
        Pixels(self.0 * other.0)
    }
}

impl Debug for Rems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} rem", self.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AbsoluteLength {
    Pixels(Pixels),
    Rems(Rems),
}

impl From<Pixels> for AbsoluteLength {
    fn from(pixels: Pixels) -> Self {
        AbsoluteLength::Pixels(pixels)
    }
}

impl From<Rems> for AbsoluteLength {
    fn from(rems: Rems) -> Self {
        AbsoluteLength::Rems(rems)
    }
}

impl AbsoluteLength {
    pub fn to_pixels(&self, rem_size: Pixels) -> Pixels {
        match self {
            AbsoluteLength::Pixels(pixels) => *pixels,
            AbsoluteLength::Rems(rems) => *rems * rem_size,
        }
    }
}

impl Default for AbsoluteLength {
    fn default() -> Self {
        px(0.)
    }
}

/// A non-auto length that can be defined in pixels, rems, or percent of parent.
#[derive(Clone, Copy)]
pub enum DefiniteLength {
    Absolute(AbsoluteLength),
    /// A fraction of the parent's size between 0 and 1.
    Fraction(f32),
}

impl Debug for DefiniteLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DefiniteLength::Absolute(length) => Debug::fmt(length, f),
            DefiniteLength::Fraction(fract) => write!(f, "{}%", (fract * 100.0) as i32),
        }
    }
}

impl From<Pixels> for DefiniteLength {
    fn from(pixels: Pixels) -> Self {
        Self::Absolute(pixels.into())
    }
}

impl From<Rems> for DefiniteLength {
    fn from(rems: Rems) -> Self {
        Self::Absolute(rems.into())
    }
}

impl From<AbsoluteLength> for DefiniteLength {
    fn from(length: AbsoluteLength) -> Self {
        Self::Absolute(length)
    }
}

impl Default for DefiniteLength {
    fn default() -> Self {
        Self::Absolute(AbsoluteLength::default())
    }
}

/// A length that can be defined in pixels, rems, percent of parent, or auto.
#[derive(Clone, Copy)]
pub enum Length {
    Definite(DefiniteLength),
    Auto,
}

impl Debug for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Length::Definite(definite_length) => write!(f, "{:?}", definite_length),
            Length::Auto => write!(f, "auto"),
        }
    }
}

pub fn relative<T: From<DefiniteLength>>(fraction: f32) -> T {
    DefiniteLength::Fraction(fraction).into()
}

pub fn rems<T: From<Rems>>(rems: f32) -> T {
    Rems(rems).into()
}

pub fn px<T: From<Pixels>>(pixels: f32) -> T {
    Pixels(pixels).into()
}

pub fn auto() -> Length {
    Length::Auto
}

impl From<Pixels> for Length {
    fn from(pixels: Pixels) -> Self {
        Self::Definite(pixels.into())
    }
}

impl From<Rems> for Length {
    fn from(rems: Rems) -> Self {
        Self::Definite(rems.into())
    }
}

impl From<DefiniteLength> for Length {
    fn from(length: DefiniteLength) -> Self {
        Self::Definite(length)
    }
}

impl From<AbsoluteLength> for Length {
    fn from(length: AbsoluteLength) -> Self {
        Self::Definite(length.into())
    }
}

impl Default for Length {
    fn default() -> Self {
        Self::Definite(DefiniteLength::default())
    }
}

impl From<()> for Length {
    fn from(_: ()) -> Self {
        Self::Definite(DefiniteLength::default())
    }
}
