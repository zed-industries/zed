use bytemuck::{Pod, Zeroable};
use core::fmt::Debug;
use derive_more::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use refineable::Refineable;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(
    Refineable, Default, Add, AddAssign, Sub, SubAssign, Mul, Div, Copy, Debug, PartialEq, Eq, Hash,
)]
#[refineable(debug)]
#[repr(C)]
pub struct Point<T: Clone + Debug> {
    pub x: T,
    pub y: T,
}

pub fn point<T: Clone + Debug>(x: T, y: T) -> Point<T> {
    Point { x, y }
}

impl<T: Clone + Debug> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn map<U: Clone + Debug, F: Fn(T) -> U>(&self, f: F) -> Point<U> {
        Point {
            x: f(self.x.clone()),
            y: f(self.y.clone()),
        }
    }
}

impl<T: Clone + Debug + Sub<Output = T>> SubAssign<Size<T>> for Point<T> {
    fn sub_assign(&mut self, rhs: Size<T>) {
        self.x = self.x.clone() - rhs.width;
        self.y = self.y.clone() - rhs.height;
    }
}

impl<T: Clone + Debug + Add<Output = T> + Copy> AddAssign<T> for Point<T> {
    fn add_assign(&mut self, rhs: T) {
        self.x = self.x.clone() + rhs;
        self.y = self.y.clone() + rhs;
    }
}

impl<T: Clone + Debug + std::cmp::PartialOrd> Point<T> {
    pub fn max(&self, other: &Self) -> Self {
        Point {
            x: if self.x >= other.x {
                self.x.clone()
            } else {
                other.x.clone()
            },
            y: if self.y >= other.y {
                self.y.clone()
            } else {
                other.y.clone()
            },
        }
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

#[derive(Refineable, Default, Clone, Copy, Debug, PartialEq)]
#[refineable(debug)]
pub struct Size<T: Clone + Debug> {
    pub width: T,
    pub height: T,
}

pub fn size<T: Clone + Debug>(width: T, height: T) -> Size<T> {
    Size { width, height }
}

impl<T: Clone + Debug> Size<T> {
    pub fn map<U: Clone + Debug, F: Fn(T) -> U>(&self, f: F) -> Size<U> {
        Size {
            width: f(self.width.clone()),
            height: f(self.height.clone()),
        }
    }
}

impl From<Size<Option<Pixels>>> for Size<Option<f32>> {
    fn from(val: Size<Option<Pixels>>) -> Self {
        Size {
            width: val.width.map(|p| p.0 as f32),
            height: val.height.map(|p| p.0 as f32),
        }
    }
}

impl Size<Length> {
    pub fn full() -> Self {
        Self {
            width: relative(1.).into(),
            height: relative(1.).into(),
        }
    }
}

impl Size<DefiniteLength> {
    pub fn zero() -> Self {
        Self {
            width: px(0.).into(),
            height: px(0.).into(),
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

#[derive(Refineable, Clone, Default, Debug, PartialEq)]
#[refineable(debug)]
pub struct Bounds<T: Clone + Debug> {
    pub origin: Point<T>,
    pub size: Size<T>,
}

impl<T: Clone + Debug + Copy + Add<T, Output = T>> Bounds<T> {
    pub fn upper_right(&self) -> Point<T> {
        Point {
            x: self.origin.x + self.size.width,
            y: self.origin.y,
        }
    }

    pub fn lower_right(&self) -> Point<T> {
        Point {
            x: self.origin.x + self.size.width,
            y: self.origin.y + self.size.height,
        }
    }
}

impl<T: Clone + Debug + Copy + PartialOrd + Add<T, Output = T>> Bounds<T> {
    pub fn contains_point(&self, point: Point<T>) -> bool {
        point.x >= self.origin.x
            && point.x <= self.origin.x + self.size.width
            && point.y >= self.origin.y
            && point.y <= self.origin.y + self.size.height
    }
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
            top: px(0.).into(),
            right: px(0.).into(),
            bottom: px(0.).into(),
            left: px(0.).into(),
        }
    }
}

impl Edges<DefiniteLength> {
    pub fn zero() -> Self {
        Self {
            top: px(0.).into(),
            right: px(0.).into(),
            bottom: px(0.).into(),
            left: px(0.).into(),
        }
    }
}

impl Edges<AbsoluteLength> {
    pub fn zero() -> Self {
        Self {
            top: px(0.).into(),
            right: px(0.).into(),
            bottom: px(0.).into(),
            left: px(0.).into(),
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

#[derive(Clone, Copy, Default, Add, AddAssign, Sub, SubAssign, Div, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Pixels(pub(crate) f32);

impl Pixels {
    pub fn round(&self) -> Self {
        Self(self.0.round())
    }
}

impl Mul<f32> for Pixels {
    type Output = Pixels;

    fn mul(self, other: f32) -> Pixels {
        Pixels(self.0 * other)
    }
}

impl Mul<Pixels> for Pixels {
    type Output = Pixels;

    fn mul(self, rhs: Pixels) -> Self::Output {
        Pixels(self.0 * rhs.0)
    }
}

impl Eq for Pixels {}

impl Ord for Pixels {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl std::hash::Hash for Pixels {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl From<f64> for Pixels {
    fn from(val: f64) -> Self {
        Pixels(val as f32)
    }
}

impl From<f32> for Pixels {
    fn from(val: f32) -> Self {
        Pixels(val)
    }
}

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

impl From<Pixels> for f64 {
    fn from(pixels: Pixels) -> Self {
        pixels.0 as f64
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
        px(0.).into()
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

pub fn relative(fraction: f32) -> DefiniteLength {
    DefiniteLength::Fraction(fraction).into()
}

pub fn rems(rems: f32) -> Rems {
    Rems(rems)
}

pub fn px(pixels: f32) -> Pixels {
    Pixels(pixels)
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
