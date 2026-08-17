// Generated from vec_mask.rs.tera template. Edit the template, not the generated file.

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::ops::*;

/// A 4-dimensional `u32` vector mask.
#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct BVec4A {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

impl BVec4A {
    /// All false.
    pub const FALSE: Self = Self::splat(false);

    /// All true.
    pub const TRUE: Self = Self::splat(true);

    /// Creates a new vector mask.
    #[inline(always)]
    pub const fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        Self {
            x: MASK[x as usize],
            y: MASK[y as usize],
            z: MASK[z as usize],
            w: MASK[w as usize],
        }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    pub const fn splat(v: bool) -> Self {
        Self::new(v, v, v, v)
    }

    /// Returns a bitmask with the lowest 4 bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    pub fn bitmask(self) -> u32 {
        (self.x & 0x1) | (self.y & 0x1) << 1 | (self.z & 0x1) << 2 | (self.w & 0x1) << 3
    }

    /// Returns true if any of the elements are true, false otherwise.
    #[inline]
    pub fn any(self) -> bool {
        ((self.x | self.y | self.z | self.w) & 0x1) != 0
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    pub fn all(self) -> bool {
        ((self.x & self.y & self.z & self.w) & 0x1) != 0
    }

    #[inline]
    fn into_bool_array(self) -> [bool; 4] {
        [
            (self.x & 0x1) != 0,
            (self.y & 0x1) != 0,
            (self.z & 0x1) != 0,
            (self.w & 0x1) != 0,
        ]
    }

    #[inline]
    fn into_u32_array(self) -> [u32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl Default for BVec4A {
    #[inline]
    fn default() -> Self {
        Self::FALSE
    }
}

impl BitAnd for BVec4A {
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

impl BitAndAssign for BVec4A {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl BitOr for BVec4A {
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

impl BitOrAssign for BVec4A {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl BitXor for BVec4A {
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

impl BitXorAssign for BVec4A {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl Not for BVec4A {
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

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for BVec4A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_u32_array();
        write!(
            f,
            "{}({:#x}, {:#x}, {:#x}, {:#x})",
            stringify!(BVec4A),
            arr[0],
            arr[1],
            arr[2],
            arr[3]
        )
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for BVec4A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_bool_array();
        write!(f, "[{}, {}, {}, {}]", arr[0], arr[1], arr[2], arr[3])
    }
}

impl From<BVec4A> for [bool; 4] {
    #[inline]
    fn from(mask: BVec4A) -> Self {
        mask.into_bool_array()
    }
}

impl From<BVec4A> for [u32; 4] {
    #[inline]
    fn from(mask: BVec4A) -> Self {
        mask.into_u32_array()
    }
}
