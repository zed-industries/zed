use core::fmt::Debug;
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, Neg, Sub, SubAssign};
use refineable::Refineable;
use serde_derive::{Deserialize, Serialize};
use std::{
    cmp::{self, PartialOrd},
    fmt,
    ops::{Add, Div, Mul, MulAssign, Sub},
};

#[derive(Refineable, Default, Add, AddAssign, Sub, SubAssign, Copy, Debug, PartialEq, Eq, Hash)]
#[refineable(debug)]
#[repr(C)]
pub struct Point<T: Default + Clone + Debug> {
    pub x: T,
    pub y: T,
}

pub fn point<T: Clone + Debug + Default>(x: T, y: T) -> Point<T> {
    Point { x, y }
}

impl<T: Clone + Debug + Default> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(T::default(), T::default())
    }

    pub fn map<U: Clone + Default + Debug>(&self, f: impl Fn(T) -> U) -> Point<U> {
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

    pub fn magnitude(&self) -> f64 {
        ((self.x.0.powi(2) + self.y.0.powi(2)) as f64).sqrt()
    }
}

impl<T, Rhs> Mul<Rhs> for Point<T>
where
    T: Mul<Rhs, Output = T> + Clone + Default + Debug,
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

impl<T, S> MulAssign<S> for Point<T>
where
    T: Clone + Mul<S, Output = T> + Default + Debug,
    S: Clone,
{
    fn mul_assign(&mut self, rhs: S) {
        self.x = self.x.clone() * rhs.clone();
        self.y = self.y.clone() * rhs;
    }
}

impl<T, S> Div<S> for Point<T>
where
    T: Div<S, Output = T> + Clone + Default + Debug,
    S: Clone,
{
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        Self {
            x: self.x / rhs.clone(),
            y: self.y / rhs,
        }
    }
}

impl<T> Point<T>
where
    T: PartialOrd + Clone + Default + Debug,
{
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

    pub fn min(&self, other: &Self) -> Self {
        Point {
            x: if self.x <= other.x {
                self.x.clone()
            } else {
                other.x.clone()
            },
            y: if self.y <= other.y {
                self.y.clone()
            } else {
                other.y.clone()
            },
        }
    }
}

impl<T: Clone + Default + Debug> Clone for Point<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

#[derive(Refineable, Default, Clone, Copy, PartialEq, Div, Hash, Serialize, Deserialize)]
#[refineable(debug)]
#[repr(C)]
pub struct Size<T: Clone + Default + Debug> {
    pub width: T,
    pub height: T,
}

pub fn size<T>(width: T, height: T) -> Size<T>
where
    T: Clone + Default + Debug,
{
    Size { width, height }
}

impl<T> Size<T>
where
    T: Clone + Default + Debug,
{
    pub fn map<U>(&self, f: impl Fn(T) -> U) -> Size<U>
    where
        U: Clone + Default + Debug,
    {
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

impl<T> Size<T>
where
    T: PartialOrd + Clone + Default + Debug,
{
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

impl<T> Sub for Size<T>
where
    T: Sub<Output = T> + Clone + Default + Debug,
{
    type Output = Size<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Size {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl<T, Rhs> Mul<Rhs> for Size<T>
where
    T: Mul<Rhs, Output = Rhs> + Clone + Default + Debug,
    Rhs: Clone + Default + Debug,
{
    type Output = Size<Rhs>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        Size {
            width: self.width * rhs.clone(),
            height: self.height * rhs,
        }
    }
}

impl<T, S> MulAssign<S> for Size<T>
where
    T: Mul<S, Output = T> + Clone + Default + Debug,
    S: Clone,
{
    fn mul_assign(&mut self, rhs: S) {
        self.width = self.width.clone() * rhs.clone();
        self.height = self.height.clone() * rhs;
    }
}

impl<T> Eq for Size<T> where T: Eq + Default + Debug + Clone {}

impl<T> Debug for Size<T>
where
    T: Clone + Default + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Size {{ {:?} × {:?} }}", self.width, self.height)
    }
}

impl<T: Clone + Default + Debug> From<Point<T>> for Size<T> {
    fn from(point: Point<T>) -> Self {
        Self {
            width: point.x,
            height: point.y,
        }
    }
}

impl From<Size<Pixels>> for Size<GlobalPixels> {
    fn from(size: Size<Pixels>) -> Self {
        Size {
            width: GlobalPixels(size.width.0),
            height: GlobalPixels(size.height.0),
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
pub struct Bounds<T: Clone + Default + Debug> {
    pub origin: Point<T>,
    pub size: Size<T>,
}

impl<T> Bounds<T>
where
    T: Clone + Debug + Sub<Output = T> + Default,
{
    pub fn from_corners(upper_left: Point<T>, lower_right: Point<T>) -> Self {
        let origin = Point {
            x: upper_left.x.clone(),
            y: upper_left.y.clone(),
        };
        let size = Size {
            width: lower_right.x - upper_left.x,
            height: lower_right.y - upper_left.y,
        };
        Bounds { origin, size }
    }
}

impl<T> Bounds<T>
where
    T: Clone + Debug + PartialOrd + Add<T, Output = T> + Sub<Output = T> + Default,
{
    pub fn intersects(&self, other: &Bounds<T>) -> bool {
        let my_lower_right = self.lower_right();
        let their_lower_right = other.lower_right();

        self.origin.x < their_lower_right.x
            && my_lower_right.x > other.origin.x
            && self.origin.y < their_lower_right.y
            && my_lower_right.y > other.origin.y
    }

    pub fn dilate(&mut self, amount: T) {
        self.origin.x = self.origin.x.clone() - amount.clone();
        self.origin.y = self.origin.y.clone() - amount.clone();
        let double_amount = amount.clone() + amount;
        self.size.width = self.size.width.clone() + double_amount.clone();
        self.size.height = self.size.height.clone() + double_amount;
    }
}

impl<T: Clone + Default + Debug + PartialOrd + Add<T, Output = T> + Sub<Output = T>> Bounds<T> {
    pub fn intersect(&self, other: &Self) -> Self {
        let upper_left = self.origin.max(&other.origin);
        let lower_right = self.lower_right().min(&other.lower_right());
        Self::from_corners(upper_left, lower_right)
    }

    pub fn union(&self, other: &Self) -> Self {
        let top_left = self.origin.min(&other.origin);
        let bottom_right = self.lower_right().max(&other.lower_right());
        Bounds::from_corners(top_left, bottom_right)
    }
}

impl<T, Rhs> Mul<Rhs> for Bounds<T>
where
    T: Mul<Rhs, Output = Rhs> + Clone + Default + Debug,
    Point<T>: Mul<Rhs, Output = Point<Rhs>>,
    Rhs: Clone + Default + Debug,
{
    type Output = Bounds<Rhs>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        Bounds {
            origin: self.origin * rhs.clone(),
            size: self.size * rhs,
        }
    }
}

impl<T, S> MulAssign<S> for Bounds<T>
where
    T: Mul<S, Output = T> + Clone + Default + Debug,
    S: Clone,
{
    fn mul_assign(&mut self, rhs: S) {
        self.origin *= rhs.clone();
        self.size *= rhs;
    }
}

impl<T, S> Div<S> for Bounds<T>
where
    Size<T>: Div<S, Output = Size<T>>,
    T: Div<S, Output = T> + Default + Clone + Debug,
    S: Clone,
{
    type Output = Self;

    fn div(self, rhs: S) -> Self {
        Self {
            origin: self.origin / rhs.clone(),
            size: self.size / rhs,
        }
    }
}

impl<T> Bounds<T>
where
    T: Add<T, Output = T> + Clone + Default + Debug,
{
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

    pub fn lower_left(&self) -> Point<T> {
        Point {
            x: self.origin.x.clone(),
            y: self.origin.y.clone() + self.size.height.clone(),
        }
    }
}

impl<T> Bounds<T>
where
    T: Add<T, Output = T> + PartialOrd + Clone + Default + Debug,
{
    pub fn contains_point(&self, point: &Point<T>) -> bool {
        point.x >= self.origin.x
            && point.x <= self.origin.x.clone() + self.size.width.clone()
            && point.y >= self.origin.y
            && point.y <= self.origin.y.clone() + self.size.height.clone()
    }

    pub fn map<U>(&self, f: impl Fn(T) -> U) -> Bounds<U>
    where
        U: Clone + Default + Debug,
    {
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

impl<T: Clone + Debug + Copy + Default> Copy for Bounds<T> {}

#[derive(Refineable, Clone, Default, Debug, Eq, PartialEq)]
#[refineable(debug)]
#[repr(C)]
pub struct Edges<T: Clone + Default + Debug> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl<T> Mul for Edges<T>
where
    T: Mul<Output = T> + Clone + Default + Debug,
{
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

impl<T, S> MulAssign<S> for Edges<T>
where
    T: Mul<S, Output = T> + Clone + Default + Debug,
    S: Clone,
{
    fn mul_assign(&mut self, rhs: S) {
        self.top = self.top.clone() * rhs.clone();
        self.right = self.right.clone() * rhs.clone();
        self.bottom = self.bottom.clone() * rhs.clone();
        self.left = self.left.clone() * rhs;
    }
}

impl<T: Clone + Default + Debug + Copy> Copy for Edges<T> {}

impl<T: Clone + Default + Debug> Edges<T> {
    pub fn map<U>(&self, f: impl Fn(&T) -> U) -> Edges<U>
    where
        U: Clone + Default + Debug,
    {
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

impl Edges<Pixels> {
    pub fn scale(&self, factor: f32) -> Edges<ScaledPixels> {
        Edges {
            top: self.top.scale(factor),
            right: self.right.scale(factor),
            bottom: self.bottom.scale(factor),
            left: self.left.scale(factor),
        }
    }
}

#[derive(Refineable, Clone, Default, Debug, Eq, PartialEq)]
#[refineable(debug)]
#[repr(C)]
pub struct Corners<T: Clone + Default + Debug> {
    pub top_left: T,
    pub top_right: T,
    pub bottom_right: T,
    pub bottom_left: T,
}

impl Corners<AbsoluteLength> {
    pub fn to_pixels(&self, size: Size<Pixels>, rem_size: Pixels) -> Corners<Pixels> {
        let max = size.width.max(size.height) / 2.;
        Corners {
            top_left: self.top_left.to_pixels(rem_size).min(max),
            top_right: self.top_right.to_pixels(rem_size).min(max),
            bottom_right: self.bottom_right.to_pixels(rem_size).min(max),
            bottom_left: self.bottom_left.to_pixels(rem_size).min(max),
        }
    }
}

impl Corners<Pixels> {
    pub fn scale(&self, factor: f32) -> Corners<ScaledPixels> {
        Corners {
            top_left: self.top_left.scale(factor),
            top_right: self.top_right.scale(factor),
            bottom_right: self.bottom_right.scale(factor),
            bottom_left: self.bottom_left.scale(factor),
        }
    }
}

impl<T: Clone + Default + Debug> Corners<T> {
    pub fn map<U>(&self, f: impl Fn(&T) -> U) -> Corners<U>
    where
        U: Clone + Default + Debug,
    {
        Corners {
            top_left: f(&self.top_left),
            top_right: f(&self.top_right),
            bottom_right: f(&self.bottom_right),
            bottom_left: f(&self.bottom_left),
        }
    }
}

impl<T> Mul for Corners<T>
where
    T: Mul<Output = T> + Clone + Default + Debug,
{
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

impl<T, S> MulAssign<S> for Corners<T>
where
    T: Mul<S, Output = T> + Clone + Default + Debug,
    S: Clone,
{
    fn mul_assign(&mut self, rhs: S) {
        self.top_left = self.top_left.clone() * rhs.clone();
        self.top_right = self.top_right.clone() * rhs.clone();
        self.bottom_right = self.bottom_right.clone() * rhs.clone();
        self.bottom_left = self.bottom_left.clone() * rhs;
    }
}

impl<T> Copy for Corners<T> where T: Copy + Clone + Default + Debug {}

#[derive(
    Clone,
    Copy,
    Default,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Neg,
    Div,
    DivAssign,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[repr(transparent)]
pub struct Pixels(pub(crate) f32);

impl std::ops::Div for Pixels {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl std::ops::DivAssign for Pixels {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl std::ops::RemAssign for Pixels {
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}

impl std::ops::Rem for Pixels {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Self(self.0 % rhs.0)
    }
}

impl Mul<f32> for Pixels {
    type Output = Pixels;

    fn mul(self, other: f32) -> Pixels {
        Pixels(self.0 * other)
    }
}

impl Mul<usize> for Pixels {
    type Output = Pixels;

    fn mul(self, other: usize) -> Pixels {
        Pixels(self.0 * other as f32)
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
    pub const MAX: Pixels = Pixels(f32::MAX);

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    pub fn as_isize(&self) -> isize {
        self.0 as isize
    }

    pub fn floor(&self) -> Self {
        Self(self.0.floor())
    }

    pub fn round(&self) -> Self {
        Self(self.0.round())
    }

    pub fn scale(&self, factor: f32) -> ScaledPixels {
        ScaledPixels(self.0 * factor)
    }

    pub fn pow(&self, exponent: f32) -> Self {
        Self(self.0.powf(exponent))
    }

    pub fn abs(&self) -> Self {
        Self(self.0.abs())
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
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl std::hash::Hash for Pixels {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl From<f64> for Pixels {
    fn from(pixels: f64) -> Self {
        Pixels(pixels as f32)
    }
}

impl From<f32> for Pixels {
    fn from(pixels: f32) -> Self {
        Pixels(pixels)
    }
}

impl Debug for Pixels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl From<Pixels> for u32 {
    fn from(pixels: Pixels) -> Self {
        pixels.0 as u32
    }
}

impl From<u32> for Pixels {
    fn from(pixels: u32) -> Self {
        Pixels(pixels as f32)
    }
}

impl From<Pixels> for usize {
    fn from(pixels: Pixels) -> Self {
        pixels.0 as usize
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

impl fmt::Debug for DevicePixels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} px (device)", self.0)
    }
}

impl From<DevicePixels> for i32 {
    fn from(device_pixels: DevicePixels) -> Self {
        device_pixels.0
    }
}

impl From<i32> for DevicePixels {
    fn from(device_pixels: i32) -> Self {
        DevicePixels(device_pixels)
    }
}

impl From<u32> for DevicePixels {
    fn from(device_pixels: u32) -> Self {
        DevicePixels(device_pixels as i32)
    }
}

impl From<DevicePixels> for u32 {
    fn from(device_pixels: DevicePixels) -> Self {
        device_pixels.0 as u32
    }
}

impl From<DevicePixels> for u64 {
    fn from(device_pixels: DevicePixels) -> Self {
        device_pixels.0 as u64
    }
}

impl From<u64> for DevicePixels {
    fn from(device_pixels: u64) -> Self {
        DevicePixels(device_pixels as i32)
    }
}

#[derive(Clone, Copy, Default, Add, AddAssign, Sub, SubAssign, Div, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ScaledPixels(pub(crate) f32);

impl ScaledPixels {
    pub fn floor(&self) -> Self {
        Self(self.0.floor())
    }

    pub fn ceil(&self) -> Self {
        Self(self.0.ceil())
    }
}

impl Eq for ScaledPixels {}

impl Debug for ScaledPixels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl From<ScaledPixels> for f64 {
    fn from(scaled_pixels: ScaledPixels) -> Self {
        scaled_pixels.0 as f64
    }
}

#[derive(Clone, Copy, Default, Add, AddAssign, Sub, SubAssign, Div, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GlobalPixels(pub(crate) f32);

impl Debug for GlobalPixels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} px (global coordinate space)", self.0)
    }
}

impl From<GlobalPixels> for f64 {
    fn from(global_pixels: GlobalPixels) -> Self {
        global_pixels.0 as f64
    }
}

impl From<f64> for GlobalPixels {
    fn from(global_pixels: f64) -> Self {
        GlobalPixels(global_pixels as f32)
    }
}

impl sqlez::bindable::StaticColumnCount for GlobalPixels {}

impl sqlez::bindable::Bind for GlobalPixels {
    fn bind(
        &self,
        statement: &sqlez::statement::Statement,
        start_index: i32,
    ) -> anyhow::Result<i32> {
        self.0.bind(statement, start_index)
    }
}

#[derive(Clone, Copy, Default, Add, Sub, Mul, Div, Neg)]
pub struct Rems(f32);

impl Mul<Pixels> for Rems {
    type Output = Pixels;

    fn mul(self, other: Pixels) -> Pixels {
        Pixels(self.0 * other.0)
    }
}

impl Debug for Rems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} rem", self.0)
    }
}

#[derive(Clone, Copy, Debug, Neg)]
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
#[derive(Clone, Copy, Neg)]
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

pub const fn px(pixels: f32) -> Pixels {
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

impl<T: IsZero + Debug + Clone + Default> IsZero for Point<T> {
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T> IsZero for Size<T>
where
    T: IsZero + Default + Debug + Clone,
{
    fn is_zero(&self) -> bool {
        self.width.is_zero() || self.height.is_zero()
    }
}

impl<T: IsZero + Debug + Clone + Default> IsZero for Bounds<T> {
    fn is_zero(&self) -> bool {
        self.size.is_zero()
    }
}

impl<T> IsZero for Corners<T>
where
    T: IsZero + Clone + Default + Debug,
{
    fn is_zero(&self) -> bool {
        self.top_left.is_zero()
            && self.top_right.is_zero()
            && self.bottom_right.is_zero()
            && self.bottom_left.is_zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds_intersects() {
        let bounds1 = Bounds {
            origin: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 5.0,
                height: 5.0,
            },
        };
        let bounds2 = Bounds {
            origin: Point { x: 4.0, y: 4.0 },
            size: Size {
                width: 5.0,
                height: 5.0,
            },
        };
        let bounds3 = Bounds {
            origin: Point { x: 10.0, y: 10.0 },
            size: Size {
                width: 5.0,
                height: 5.0,
            },
        };

        // Test Case 1: Intersecting bounds
        assert_eq!(bounds1.intersects(&bounds2), true);

        // Test Case 2: Non-Intersecting bounds
        assert_eq!(bounds1.intersects(&bounds3), false);

        // Test Case 3: Bounds intersecting with themselves
        assert_eq!(bounds1.intersects(&bounds1), true);
    }
}
