use core::fmt::Debug;
use derive_more::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use refineable::Refineable;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

#[derive(Refineable, Default, Add, AddAssign, Sub, SubAssign, Copy, Debug, PartialEq, Eq, Hash)]
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

impl Point<Pixels> {
    pub fn scale(&self, factor: f32) -> Point<ScaledPixels> {
        Point {
            x: self.x.scale(factor),
            y: self.y.scale(factor),
        }
    }
}

impl<T, Rhs> Mul<Rhs> for Point<T>
where
    T: Mul<Rhs, Output = T> + Clone + Debug,
    Rhs: Clone + Debug,
{
    type Output = Point<T>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        Point {
            x: self.x * rhs.clone(),
            y: self.y * rhs,
        }
    }
}

impl<T: Clone + Debug + Mul<S, Output = T>, S: Clone> MulAssign<S> for Point<T> {
    fn mul_assign(&mut self, rhs: S) {
        self.x = self.x.clone() * rhs.clone();
        self.y = self.y.clone() * rhs;
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

impl<T: Clone + Debug + Div<S, Output = T>, S: Clone> Div<S> for Point<T> {
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        Self {
            x: self.x / rhs.clone(),
            y: self.y / rhs,
        }
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

#[derive(Refineable, Default, Clone, Copy, Debug, PartialEq, Div, Hash)]
#[refineable(debug)]
#[repr(C)]
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

impl Size<Pixels> {
    pub fn scale(&self, factor: f32) -> Size<ScaledPixels> {
        Size {
            width: self.width.scale(factor),
            height: self.height.scale(factor),
        }
    }
}

impl<T: Clone + Debug + Ord> Size<T> {
    pub fn max(&self, other: &Self) -> Self {
        Size {
            width: if self.width >= other.width {
                self.width.clone()
            } else {
                other.width.clone()
            },
            height: if self.height >= other.height {
                self.height.clone()
            } else {
                other.height.clone()
            },
        }
    }
}

impl<T, Rhs> Mul<Rhs> for Size<T>
where
    T: Mul<Rhs, Output = Rhs> + Debug + Clone,
    Rhs: Debug + Clone,
{
    type Output = Size<Rhs>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        Size {
            width: self.width * rhs.clone(),
            height: self.height * rhs,
        }
    }
}

impl<T: Clone + Debug + Mul<S, Output = T>, S: Clone> MulAssign<S> for Size<T> {
    fn mul_assign(&mut self, rhs: S) {
        self.width = self.width.clone() * rhs.clone();
        self.height = self.height.clone() * rhs;
    }
}

impl<T: Eq + Debug + Clone> Eq for Size<T> {}

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

#[derive(Refineable, Clone, Default, Debug, Eq, PartialEq)]
#[refineable(debug)]
#[repr(C)]
pub struct Bounds<T: Clone + Debug> {
    pub origin: Point<T>,
    pub size: Size<T>,
}

impl<T, Rhs> Mul<Rhs> for Bounds<T>
where
    T: Mul<Rhs, Output = Rhs> + Clone + Debug,
    Point<T>: Mul<Rhs, Output = Point<Rhs>>,
    Rhs: Clone + Debug,
{
    type Output = Bounds<Rhs>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        Bounds {
            origin: self.origin * rhs.clone(),
            size: self.size * rhs,
        }
    }
}

impl<T: Clone + Debug + Mul<S, Output = T>, S: Clone> MulAssign<S> for Bounds<T> {
    fn mul_assign(&mut self, rhs: S) {
        self.origin *= rhs.clone();
        self.size *= rhs;
    }
}

impl<T: Clone + Debug + Div<S, Output = T>, S: Clone> Div<S> for Bounds<T>
where
    Size<T>: Div<S, Output = Size<T>>,
{
    type Output = Self;

    fn div(self, rhs: S) -> Self {
        Self {
            origin: self.origin / rhs.clone(),
            size: self.size / rhs,
        }
    }
}

impl<T: Clone + Debug + Add<T, Output = T>> Bounds<T> {
    pub fn upper_right(&self) -> Point<T> {
        Point {
            x: self.origin.x.clone() + self.size.width.clone(),
            y: self.origin.y.clone(),
        }
    }

    pub fn lower_right(&self) -> Point<T> {
        Point {
            x: self.origin.x.clone() + self.size.width.clone(),
            y: self.origin.y.clone() + self.size.height.clone(),
        }
    }
}

impl<T: Clone + Debug + PartialOrd + Add<T, Output = T>> Bounds<T> {
    pub fn contains_point(&self, point: Point<T>) -> bool {
        point.x >= self.origin.x
            && point.x <= self.origin.x.clone() + self.size.width.clone()
            && point.y >= self.origin.y
            && point.y <= self.origin.y.clone() + self.size.height.clone()
    }

    pub fn map<U: Clone + Debug, F: Fn(T) -> U>(&self, f: F) -> Bounds<U> {
        Bounds {
            origin: self.origin.map(&f),
            size: self.size.map(f),
        }
    }
}

impl Bounds<Pixels> {
    pub fn scale(&self, factor: f32) -> Bounds<ScaledPixels> {
        Bounds {
            origin: self.origin.scale(factor),
            size: self.size.scale(factor),
        }
    }
}

impl<T: Clone + Debug + Copy> Copy for Bounds<T> {}

#[derive(Refineable, Clone, Default, Debug, Eq, PartialEq)]
#[refineable(debug)]
#[repr(C)]
pub struct Edges<T: Clone + Debug> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl<T: Clone + Debug + Mul<Output = T>> Mul for Edges<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            top: self.top.clone() * rhs.top,
            right: self.right.clone() * rhs.right,
            bottom: self.bottom.clone() * rhs.bottom,
            left: self.left.clone() * rhs.left,
        }
    }
}

impl<T: Clone + Debug + Mul<S, Output = T>, S: Clone> MulAssign<S> for Edges<T> {
    fn mul_assign(&mut self, rhs: S) {
        self.top = self.top.clone() * rhs.clone();
        self.right = self.right.clone() * rhs.clone();
        self.bottom = self.bottom.clone() * rhs.clone();
        self.left = self.left.clone() * rhs.clone();
    }
}

impl<T: Clone + Debug + Copy> Copy for Edges<T> {}

impl<T: Clone + Debug> Edges<T> {
    pub fn map<U: Clone + Debug, F: Fn(&T) -> U>(&self, f: F) -> Edges<U> {
        Edges {
            top: f(&self.top),
            right: f(&self.right),
            bottom: f(&self.bottom),
            left: f(&self.left),
        }
    }

    pub fn any<F: Fn(&T) -> bool>(&self, predicate: F) -> bool {
        predicate(&self.top)
            || predicate(&self.right)
            || predicate(&self.bottom)
            || predicate(&self.left)
    }
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

#[derive(Refineable, Clone, Default, Debug, Eq, PartialEq)]
#[refineable(debug)]
#[repr(C)]
pub struct Corners<T: Clone + Debug> {
    pub top_left: T,
    pub top_right: T,
    pub bottom_right: T,
    pub bottom_left: T,
}

impl<T: Clone + Debug> Corners<T> {
    pub fn map<U: Clone + Debug, F: Fn(&T) -> U>(&self, f: F) -> Corners<U> {
        Corners {
            top_left: f(&self.top_left),
            top_right: f(&self.top_right),
            bottom_right: f(&self.bottom_right),
            bottom_left: f(&self.bottom_left),
        }
    }
}

impl<T: Clone + Debug + Mul<Output = T>> Mul for Corners<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            top_left: self.top_left.clone() * rhs.top_left,
            top_right: self.top_right.clone() * rhs.top_right,
            bottom_right: self.bottom_right.clone() * rhs.bottom_right,
            bottom_left: self.bottom_left.clone() * rhs.bottom_left,
        }
    }
}

impl<T: Clone + Debug + Mul<S, Output = T>, S: Clone> MulAssign<S> for Corners<T> {
    fn mul_assign(&mut self, rhs: S) {
        self.top_left = self.top_left.clone() * rhs.clone();
        self.top_right = self.top_right.clone() * rhs.clone();
        self.bottom_right = self.bottom_right.clone() * rhs.clone();
        self.bottom_left = self.bottom_left.clone() * rhs;
    }
}

impl<T: Clone + Debug + Copy> Copy for Corners<T> {}

#[derive(Clone, Copy, Default, Add, AddAssign, Sub, SubAssign, Div, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Pixels(pub(crate) f32);

impl Mul<f32> for Pixels {
    type Output = Pixels;

    fn mul(self, other: f32) -> Pixels {
        Pixels(self.0 * other)
    }
}

impl Mul<Pixels> for f32 {
    type Output = Pixels;

    fn mul(self, rhs: Pixels) -> Self::Output {
        Pixels(self * rhs.0)
    }
}

impl MulAssign<f32> for Pixels {
    fn mul_assign(&mut self, other: f32) {
        self.0 *= other;
    }
}

impl Pixels {
    pub fn round(&self) -> Self {
        Self(self.0.round())
    }

    pub fn scale(&self, factor: f32) -> ScaledPixels {
        ScaledPixels(self.0 * factor)
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

#[derive(
    Add, AddAssign, Clone, Copy, Default, Div, Eq, Hash, Ord, PartialEq, PartialOrd, Sub, SubAssign,
)]
#[repr(transparent)]
pub struct DevicePixels(pub(crate) i32);

impl DevicePixels {
    pub fn to_bytes(&self, bytes_per_pixel: u8) -> u32 {
        self.0 as u32 * bytes_per_pixel as u32
    }
}

impl std::fmt::Debug for DevicePixels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} px (device)", self.0)
    }
}

impl From<DevicePixels> for i32 {
    fn from(device_pixels: DevicePixels) -> Self {
        device_pixels.0
    }
}

impl From<i32> for DevicePixels {
    fn from(val: i32) -> Self {
        DevicePixels(val)
    }
}

impl From<DevicePixels> for u64 {
    fn from(device_pixels: DevicePixels) -> Self {
        device_pixels.0 as u64
    }
}

impl From<u64> for DevicePixels {
    fn from(val: u64) -> Self {
        DevicePixels(val as i32)
    }
}

#[derive(Clone, Copy, Default, Add, AddAssign, Sub, SubAssign, Div, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ScaledPixels(pub(crate) f32);

impl ScaledPixels {
    pub fn floor(&self) -> Self {
        Self(self.0.floor())
    }
}

impl Eq for ScaledPixels {}

impl Debug for ScaledPixels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} px (scaled)", self.0)
    }
}

impl From<ScaledPixels> for DevicePixels {
    fn from(scaled: ScaledPixels) -> Self {
        DevicePixels(scaled.0.ceil() as i32)
    }
}

impl From<DevicePixels> for ScaledPixels {
    fn from(device: DevicePixels) -> Self {
        ScaledPixels(device.0 as f32)
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

impl AbsoluteLength {
    pub fn is_zero(&self) -> bool {
        match self {
            AbsoluteLength::Pixels(px) => px.0 == 0.,
            AbsoluteLength::Rems(rems) => rems.0 == 0.,
        }
    }
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

impl DefiniteLength {
    pub fn to_pixels(&self, base_size: AbsoluteLength, rem_size: Pixels) -> Pixels {
        match self {
            DefiniteLength::Absolute(size) => size.to_pixels(rem_size),
            DefiniteLength::Fraction(fraction) => match base_size {
                AbsoluteLength::Pixels(px) => px * *fraction,
                AbsoluteLength::Rems(rems) => rems * rem_size * *fraction,
            },
        }
    }
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

/// Returns the Golden Ratio, i.e. `~(1.0 + sqrt(5.0)) / 2.0`.
pub fn phi() -> DefiniteLength {
    relative(1.61803398875)
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

pub trait IsZero {
    fn is_zero(&self) -> bool;
}

impl IsZero for DevicePixels {
    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl IsZero for ScaledPixels {
    fn is_zero(&self) -> bool {
        self.0 == 0.
    }
}

impl IsZero for Pixels {
    fn is_zero(&self) -> bool {
        self.0 == 0.
    }
}

impl IsZero for Rems {
    fn is_zero(&self) -> bool {
        self.0 == 0.
    }
}

impl IsZero for AbsoluteLength {
    fn is_zero(&self) -> bool {
        match self {
            AbsoluteLength::Pixels(pixels) => pixels.is_zero(),
            AbsoluteLength::Rems(rems) => rems.is_zero(),
        }
    }
}

impl IsZero for DefiniteLength {
    fn is_zero(&self) -> bool {
        match self {
            DefiniteLength::Absolute(length) => length.is_zero(),
            DefiniteLength::Fraction(fraction) => *fraction == 0.,
        }
    }
}

impl IsZero for Length {
    fn is_zero(&self) -> bool {
        match self {
            Length::Definite(length) => length.is_zero(),
            Length::Auto => false,
        }
    }
}

impl<T: IsZero + Debug + Clone> IsZero for Point<T> {
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T: IsZero + Debug + Clone> IsZero for Size<T> {
    fn is_zero(&self) -> bool {
        self.width.is_zero() && self.height.is_zero()
    }
}

impl<T: IsZero + Debug + Clone> IsZero for Bounds<T> {
    fn is_zero(&self) -> bool {
        self.origin.is_zero() && self.size.is_zero()
    }
}
