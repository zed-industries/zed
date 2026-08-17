// Generated from vec_mask.rs.tera template. Edit the template, not the generated file.

use core::fmt;
use core::ops::*;

/// Creates a 4-dimensional `bool` vector mask.
#[inline(always)]
#[must_use]
pub const fn bvec4(x: bool, y: bool, z: bool, w: bool) -> BVec4 {
    BVec4::new(x, y, z, w)
}

/// A 4-dimensional `bool` vector mask.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C, align(1))]
pub struct BVec4 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub w: bool,
}

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

impl BVec4 {
    /// All false.
    pub const FALSE: Self = Self::splat(false);

    /// All true.
    pub const TRUE: Self = Self::splat(true);

    /// Creates a new vector mask.
    #[inline(always)]
    #[must_use]
    pub const fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a vector mask with all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: bool) -> Self {
        Self::new(v, v, v, v)
    }

    /// Creates a new vector mask from a bool array.
    #[inline]
    #[must_use]
    pub const fn from_array(a: [bool; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }

    /// Returns a bitmask with the lowest 4 bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    #[must_use]
    pub fn bitmask(self) -> u32 {
        (self.x as u32) | ((self.y as u32) << 1) | ((self.z as u32) << 2) | ((self.w as u32) << 3)
    }

    /// Returns true if any of the elements are true, false otherwise.
    #[inline]
    #[must_use]
    pub fn any(self) -> bool {
        self.x || self.y || self.z || self.w
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    #[must_use]
    pub fn all(self) -> bool {
        self.x && self.y && self.z && self.w
    }

    /// Tests the value at `index`.
    ///
    /// Panics if `index` is greater than 3.
    #[inline]
    #[must_use]
    pub fn test(&self, index: usize) -> bool {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => panic!("index out of bounds"),
        }
    }

    /// Sets the element at `index`.
    ///
    /// Panics if `index` is greater than 3.
    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            3 => self.w = value,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    #[must_use]
    fn into_bool_array(self) -> [bool; 4] {
        [self.x, self.y, self.z, self.w]
    }

    #[inline]
    #[must_use]
    fn into_u32_array(self) -> [u32; 4] {
        [
            MASK[self.x as usize],
            MASK[self.y as usize],
            MASK[self.z as usize],
            MASK[self.w as usize],
        ]
    }
}

impl Default for BVec4 {
    #[inline]
    fn default() -> Self {
        Self::FALSE
    }
}

impl BitAnd for BVec4 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
            z: self.z & rhs.z,
            w: self.w & rhs.w,
        }
    }
}

impl BitAndAssign for BVec4 {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl BitOr for BVec4 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
            z: self.z | rhs.z,
            w: self.w | rhs.w,
        }
    }
}

impl BitOrAssign for BVec4 {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl BitXor for BVec4 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
            z: self.z ^ rhs.z,
            w: self.w ^ rhs.w,
        }
    }
}

impl BitXorAssign for BVec4 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl Not for BVec4 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
            w: !self.w,
        }
    }
}

impl fmt::Debug for BVec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_u32_array();
        write!(
            f,
            "{}({:#x}, {:#x}, {:#x}, {:#x})",
            stringify!(BVec4),
            arr[0],
            arr[1],
            arr[2],
            arr[3]
        )
    }
}

impl fmt::Display for BVec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_bool_array();
        write!(f, "[{}, {}, {}, {}]", arr[0], arr[1], arr[2], arr[3])
    }
}

impl From<[bool; 4]> for BVec4 {
    #[inline]
    fn from(a: [bool; 4]) -> Self {
        Self::from_array(a)
    }
}

impl From<BVec4> for [bool; 4] {
    #[inline]
    fn from(mask: BVec4) -> Self {
        mask.into_bool_array()
    }
}

impl From<BVec4> for [u32; 4] {
    #[inline]
    fn from(mask: BVec4) -> Self {
        mask.into_u32_array()
    }
}
