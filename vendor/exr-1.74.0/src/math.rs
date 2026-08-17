
// calculations inspired by
// https://github.com/AcademySoftwareFoundation/openexr/blob/master/OpenEXR/IlmImf/ImfTiledMisc.cpp

//! Simple math utilities.

use std::convert::TryFrom;
use crate::error::{i32_to_usize};
use crate::error::Result;
use std::ops::{Add, Sub, Div, Mul};
use std::fmt::Debug;

/// Simple two-dimensional vector of any numerical type.
/// Supports only few mathematical operations
/// as this is used mainly as data struct.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Vec2<T> (pub T, pub T);

impl<T> Vec2<T> {

    /// Returns the vector with the maximum of either coordinates.
    pub fn max(self, other: Self) -> Self where T: Ord {
        Vec2(self.0.max(other.0), self.1.max(other.1))
    }

    /// Returns the vector with the minimum of either coordinates.
    pub fn min(self, other: Self) -> Self where T: Ord {
        Vec2(self.0.min(other.0), self.1.min(other.1))
    }

    /// Try to convert all components of this vector to a new type,
    /// yielding either a vector of that new type, or an error.
    pub fn try_from<S>(value: Vec2<S>) -> std::result::Result<Self, T::Error> where T: TryFrom<S> {
        let x = T::try_from(value.0)?;
        let y = T::try_from(value.1)?;
        Ok(Vec2(x, y))
    }



    /// Seeing this vector as a dimension or size (width and height),
    /// this returns the area that this dimensions contains (`width * height`).
    #[inline] pub fn area(self) -> T where T: std::ops::Mul<T, Output = T> {
        self.0 * self.1
    }

    /// The first component of this 2D vector.
    #[inline] pub fn x(self) -> T { self.0 }

    /// The second component of this 2D vector.
    #[inline] pub fn y(self) -> T { self.1 }

    /// The first component of this 2D vector.
    #[inline] pub fn width(self) -> T { self.0 }

    /// The second component of this 2D vector.
    #[inline] pub fn height(self) -> T { self.1 }

    // TODO use this!
    /// Convert this two-dimensional coordinate to an index suited for one-dimensional flattened image arrays.
    /// Works for images that store the pixels row by row, one after another, in a single array.
    /// In debug mode, panics for an index out of bounds.
    #[inline] pub fn flat_index_for_size(self, resolution: Vec2<T>) -> T
        where T: Copy + Debug + Ord + Mul<Output=T> + Add<Output=T>
    {
        debug_assert!(
            self.x() < resolution.width() && self.y() < resolution.height(),
            "Vec2 index {:?} is invalid for resolution {:?}", self, resolution
        );

        let Vec2(x, y) = self;
        y * resolution.width() + x
    }
}



impl Vec2<i32> {

    /// Try to convert to [`Vec2<usize>`], returning an error on negative numbers.
    pub fn to_usize(self, error_message: &'static str) -> Result<Vec2<usize>> {
        let x = i32_to_usize(self.0, error_message)?;
        let y = i32_to_usize(self.1, error_message)?;
        Ok(Vec2(x, y))
    }

}

impl Vec2<usize> {

    /// Panics for too large values
    pub fn to_i32(self) -> Vec2<i32> {
        let x = i32::try_from(self.0).expect("vector x coordinate too large");
        let y = i32::try_from(self.1).expect("vector y coordinate too large");
        Vec2(x, y)
    }

}


impl<T: std::ops::Add<T>> std::ops::Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn add(self, other: Vec2<T>) -> Self::Output {
        Vec2(self.0 + other.0, self.1 + other.1)
    }
}

impl<T: std::ops::Sub<T>> std::ops::Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn sub(self, other: Vec2<T>) -> Self::Output {
        Vec2(self.0 - other.0, self.1 - other.1)
    }
}

impl<T: std::ops::Div<T>> std::ops::Div<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn div(self, other: Vec2<T>) -> Self::Output {
        Vec2(self.0 / other.0, self.1 / other.1)
    }
}

impl<T: std::ops::Mul<T>> std::ops::Mul<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T::Output>;
    fn mul(self, other: Vec2<T>) -> Self::Output {
        Vec2(self.0 * other.0, self.1 * other.1)
    }
}

impl<T> std::ops::Neg for Vec2<T> where T: std::ops::Neg<Output=T> {
    type Output = Vec2<T>;
    fn neg(self) -> Self::Output { Vec2(-self.0, -self.1) }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self { Vec2(x, y) }
}

impl<T> From<Vec2<T>> for (T, T) {
    fn from(vec2: Vec2<T>) -> Self { (vec2.0, vec2.1) }
}

/// Computes `floor(log(x)/log(2))`. Returns 0 where argument is 0.
// TODO does rust std not provide this?
pub(crate) fn floor_log_2(mut number: u32) -> u32 {
    let mut log = 0;

    // TODO check if this unrolls properly?
    while number > 1 {
        log += 1;
        number >>= 1;
    }

    log
}


/// Computes `ceil(log(x)/log(2))`. Returns 0 where argument is 0.
// taken from https://github.com/openexr/openexr/blob/master/OpenEXR/IlmImf/ImfTiledMisc.cpp
// TODO does rust std not provide this?
pub(crate) fn ceil_log_2(mut number: u32) -> u32 {
    let mut log = 0;
    let mut round_up = 0;

    // TODO check if this unrolls properly
    while number > 1 {
        if number & 1 != 0 {
            round_up = 1;
        }

        log +=  1;
        number >>= 1;
    }

    log + round_up
}


/// Round up or down in specific calculations.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum RoundingMode {

    /// Round down.
    Down,

    /// Round up.
    Up,
}

impl RoundingMode {
    pub(crate) fn log2(self, number: u32) -> u32 {
        match self {
            RoundingMode::Down => self::floor_log_2(number),
            RoundingMode::Up => self::ceil_log_2(number),
        }
    }

    /// Only works for positive numbers.
    pub(crate) fn divide<T>(self, dividend: T, divisor: T) -> T
        where T: Copy + Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<u8> + std::cmp::PartialOrd
    {
        assert!(
            dividend >= T::from(0) && divisor >= T::from(1),
            "division with rounding up only works for positive numbers"
        );

        match self {
            RoundingMode::Up => (dividend + divisor - T::from(1_u8)) / divisor, // only works for positive numbers
            RoundingMode::Down => dividend / divisor,
        }
    }
}

// TODO log2 tests
