// Generated from vec_mask.rs.tera template. Edit the template, not the generated file.

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::{hash, ops::*};

/// A 2-dimensional boolean vector.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BVec2 {
    pub x: bool,
    pub y: bool,
}

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

const FALSE: BVec2 = BVec2::new(false, false);

impl BVec2 {
    /// Creates a new vector mask.
    #[inline(always)]
    pub const fn new(x: bool, y: bool) -> Self {
        Self { x, y }
    }

    /// Returns a bitmask with the lowest two bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    pub fn bitmask(self) -> u32 {
        (self.x as u32) | (self.y as u32) << 1
    }

    /// Returns true if any of the elements are true, false otherwise.
    #[inline]
    pub fn any(self) -> bool {
        self.x || self.y
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    pub fn all(self) -> bool {
        self.x && self.y
    }

    #[inline]
    fn into_bool_array(self) -> [bool; 2] {
        [self.x, self.y]
    }

    #[inline]
    fn into_u32_array(self) -> [u32; 2] {
        [MASK[self.x as usize], MASK[self.y as usize]]
    }
}

impl Default for BVec2 {
    #[inline]
    fn default() -> Self {
        FALSE
    }
}

impl PartialEq for BVec2 {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.bitmask().eq(&rhs.bitmask())
    }
}

impl Eq for BVec2 {}

impl hash::Hash for BVec2 {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.bitmask().hash(state);
    }
}

impl BitAnd for BVec2 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
        }
    }
}

impl BitAndAssign for BVec2 {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl BitOr for BVec2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
        }
    }
}

impl BitOrAssign for BVec2 {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl BitXor for BVec2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
        }
    }
}

impl BitXorAssign for BVec2 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl Not for BVec2 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for BVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_u32_array();
        write!(f, "{}({:#x}, {:#x})", stringify!(BVec2), arr[0], arr[1])
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for BVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_bool_array();
        write!(f, "[{}, {}]", arr[0], arr[1])
    }
}

impl From<BVec2> for [bool; 2] {
    #[inline]
    fn from(mask: BVec2) -> Self {
        mask.into_bool_array()
    }
}

impl From<BVec2> for [u32; 2] {
    #[inline]
    fn from(mask: BVec2) -> Self {
        mask.into_u32_array()
    }
}
