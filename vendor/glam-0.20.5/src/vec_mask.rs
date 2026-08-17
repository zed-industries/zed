use crate::core::traits::vector::{
    MaskVector, MaskVector2, MaskVector3, MaskVector4, MaskVectorConst,
};
#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::{hash, ops::*};

#[cfg(all(
    target_arch = "x86",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use core::arch::x86::*;
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use core::arch::x86_64::*;

#[cfg(all(target_feature = "simd128", not(feature = "scalar-math")))]
use core::arch::wasm32::v128;

macro_rules! impl_vecnmask_methods {
    ($vecnmask:ident, $trait:ident) => {
        /// Returns a bitmask with the lowest two bits set from the elements of `self`.
        ///
        /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
        /// into the first lowest bit, element `y` into the second, etc.
        #[inline]
        pub fn bitmask(self) -> u32 {
            $trait::bitmask(self.0)
        }

        /// Returns true if any of the elements are true, false otherwise.
        #[inline]
        pub fn any(self) -> bool {
            $trait::any(self.0)
        }

        /// Returns true if all the elements are true, false otherwise.
        #[inline]
        pub fn all(self) -> bool {
            $trait::all(self.0)
        }
    };
}

macro_rules! impl_vecnmask_traits {
    ($vecnmask:ident, $inner:ident) => {
        impl Default for $vecnmask {
            #[inline]
            fn default() -> Self {
                Self($inner::FALSE)
            }
        }

        impl PartialEq for $vecnmask {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.bitmask().eq(&other.bitmask())
            }
        }

        impl Eq for $vecnmask {}

        impl hash::Hash for $vecnmask {
            #[inline]
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                self.bitmask().hash(state);
            }
        }

        impl BitAnd for $vecnmask {
            type Output = Self;
            #[inline]
            fn bitand(self, other: Self) -> Self {
                Self(MaskVector::bitand(self.0, other.0))
            }
        }

        impl BitAndAssign for $vecnmask {
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                self.0 = MaskVector::bitand(self.0, other.0);
            }
        }

        impl BitOr for $vecnmask {
            type Output = Self;
            #[inline]
            fn bitor(self, other: Self) -> Self {
                Self(MaskVector::bitor(self.0, other.0))
            }
        }

        impl BitOrAssign for $vecnmask {
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.0 = MaskVector::bitor(self.0, other.0);
            }
        }

        impl Not for $vecnmask {
            type Output = Self;
            #[inline]
            fn not(self) -> Self {
                Self(MaskVector::not(self.0))
            }
        }

        impl From<$vecnmask> for $inner {
            #[inline]
            fn from(t: $vecnmask) -> Self {
                t.0
            }
        }
    };
}

macro_rules! impl_vec2mask {
    ($vec2mask:ident, $t:ty, $inner:ident) => {
        impl $vec2mask {
            /// Creates a new vector mask.
            #[inline]
            pub fn new(x: bool, y: bool) -> Self {
                Self(MaskVector2::new(x, y))
            }

            impl_vecnmask_methods!($vec2mask, MaskVector2);
        }

        impl_vecnmask_traits!($vec2mask, $inner);

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $vec2mask {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let arr = self.0.into_u32_array();
                write!(f, "{}({:#x}, {:#x})", stringify!($vec2mask), arr[0], arr[1])
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $vec2mask {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let arr = self.0.into_bool_array();
                write!(f, "[{}, {}]", arr[0], arr[1])
            }
        }

        impl From<$vec2mask> for [bool; 2] {
            #[inline]
            fn from(mask: $vec2mask) -> Self {
                mask.0.into_bool_array()
            }
        }

        impl From<$vec2mask> for [u32; 2] {
            #[inline]
            fn from(mask: $vec2mask) -> Self {
                mask.0.into_u32_array()
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl AsRef<[$t; 2]> for $vec2mask {
            #[inline]
            fn as_ref(&self) -> &[$t; 2] {
                unsafe { &*(self as *const Self as *const [$t; 2]) }
            }
        }
    };
}

macro_rules! impl_vec3mask {
    ($vec3mask:ident, $t:ty, $inner:ident) => {
        impl $vec3mask {
            /// Creates a new vector mask.
            #[inline]
            pub fn new(x: bool, y: bool, z: bool) -> Self {
                Self(MaskVector3::new(x, y, z))
            }

            impl_vecnmask_methods!($vec3mask, MaskVector3);
        }

        impl_vecnmask_traits!($vec3mask, $inner);

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $vec3mask {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let arr = MaskVector3::into_u32_array(self.0);
                write!(
                    f,
                    "{}({:#x}, {:#x}, {:#x})",
                    stringify!($vec3mask),
                    arr[0],
                    arr[1],
                    arr[2]
                )
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $vec3mask {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let arr = MaskVector3::into_bool_array(self.0);
                write!(f, "[{}, {}, {}]", arr[0], arr[1], arr[2])
            }
        }

        impl From<$vec3mask> for [bool; 3] {
            #[inline]
            fn from(mask: $vec3mask) -> Self {
                MaskVector3::into_bool_array(mask.0)
            }
        }

        impl From<$vec3mask> for [u32; 3] {
            #[inline]
            fn from(mask: $vec3mask) -> Self {
                MaskVector3::into_u32_array(mask.0)
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl AsRef<[$t; 3]> for $vec3mask {
            #[inline]
            fn as_ref(&self) -> &[$t; 3] {
                unsafe { &*(self as *const Self as *const [$t; 3]) }
            }
        }
    };
}

macro_rules! impl_vec4mask {
    ($vec4mask:ident, $t:ty, $inner:ident) => {
        impl $vec4mask {
            /// Creates a new vector mask.
            #[inline]
            pub fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
                Self(MaskVector4::new(x, y, z, w))
            }

            impl_vecnmask_methods!($vec4mask, MaskVector4);
        }

        impl_vecnmask_traits!($vec4mask, $inner);

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $vec4mask {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let arr = MaskVector4::into_u32_array(self.0);
                write!(
                    f,
                    "{}({:#x}, {:#x}, {:#x}, {:#x})",
                    stringify!($vec4mask),
                    arr[0],
                    arr[1],
                    arr[2],
                    arr[3]
                )
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $vec4mask {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let arr = MaskVector4::into_bool_array(self.0);
                write!(f, "[{}, {}, {}, {}]", arr[0], arr[1], arr[2], arr[3])
            }
        }

        impl From<$vec4mask> for [bool; 4] {
            #[inline]
            fn from(mask: $vec4mask) -> Self {
                MaskVector4::into_bool_array(mask.0)
            }
        }

        impl From<$vec4mask> for [u32; 4] {
            #[inline]
            fn from(mask: $vec4mask) -> Self {
                MaskVector4::into_u32_array(mask.0)
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl AsRef<[$t; 4]> for $vec4mask {
            #[inline]
            fn as_ref(&self) -> &[$t; 4] {
                unsafe { &*(self as *const Self as *const [$t; 4]) }
            }
        }
    };
}

// BVec3A /////////////////////////////////////////////////////////////////////////////////////////

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
type Mask128 = __m128;
#[cfg(all(target_feature = "simd128", not(feature = "scalar-math")))]
type Mask128 = v128;

/// A 3-dimensional SIMD vector mask.
///
/// This type is 16 byte aligned and is backed by a SIMD vector. If SIMD is not available `BVec3A`
/// will be a type alias for `BVec3`.
#[cfg(all(
    any(target_feature = "sse2", target_feature = "simd128"),
    not(feature = "scalar-math")
))]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BVec3A(pub(crate) Mask128);

#[cfg(all(
    any(target_feature = "sse2", target_feature = "simd128"),
    not(feature = "scalar-math")
))]
impl_vec3mask!(BVec3A, u32, Mask128);

#[cfg(any(
    not(any(target_feature = "sse2", target_feature = "simd128")),
    feature = "scalar-math"
))]
pub type BVec3A = BVec3;

#[cfg(all(
    any(target_feature = "sse2", target_feature = "simd128"),
    not(feature = "scalar-math")
))]
impl From<BVec3> for BVec3A {
    #[inline]
    fn from(b: BVec3) -> Self {
        Self::new(b.0.x, b.0.y, b.0.z)
    }
}

#[cfg(all(
    any(target_feature = "sse2", target_feature = "simd128"),
    not(feature = "scalar-math")
))]
impl From<BVec3A> for BVec3 {
    #[inline]
    fn from(b: BVec3A) -> Self {
        let b: [bool; 3] = b.into();
        Self::new(b[0], b[1], b[2])
    }
}

// BVec4A  ////////////////////////////////////////////////////////////////////////////////////////

/// A 4-dimensional SIMD vector mask.
///
/// This type is 16 byte aligned and is backed by a SIMD vector. If SIMD is not available `BVec4A`
/// will be a type alias for `BVec4`.
#[cfg(all(
    any(target_feature = "sse2", target_feature = "simd128"),
    not(feature = "scalar-math")
))]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BVec4A(pub(crate) Mask128);

#[cfg(all(
    any(target_feature = "sse2", target_feature = "simd128"),
    not(feature = "scalar-math")
))]
impl_vec4mask!(BVec4A, u32, Mask128);

#[cfg(any(
    not(any(target_feature = "sse2", target_feature = "simd128")),
    feature = "scalar-math"
))]
pub type BVec4A = BVec4;

#[cfg(all(
    any(target_feature = "sse2", target_feature = "simd128"),
    not(feature = "scalar-math")
))]
impl From<BVec4> for BVec4A {
    #[inline]
    fn from(b: BVec4) -> Self {
        Self::new(b.0.x, b.0.y, b.0.z, b.0.w)
    }
}

#[cfg(all(
    any(target_feature = "sse2", target_feature = "simd128"),
    not(feature = "scalar-math")
))]
impl From<BVec4A> for BVec4 {
    #[inline]
    fn from(b: BVec4A) -> Self {
        let b: [bool; 4] = b.into();
        Self::new(b[0], b[1], b[2], b[3])
    }
}

// boolean vectors ////////////////////////////////////////////////////////////////////////////////
type XYBool = crate::XY<bool>;

/// A 2-dimensional boolean vector.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BVec2(pub(crate) XYBool);
impl_vec2mask!(BVec2, bool, XYBool);

type XYZBool = crate::XYZ<bool>;

/// A 3-dimensional boolean vector.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BVec3(pub(crate) XYZBool);
impl_vec3mask!(BVec3, bool, XYZBool);

type XYZWBool = crate::XYZW<bool>;

/// A 4-dimensional boolean vector.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BVec4(pub(crate) XYZWBool);
impl_vec4mask!(BVec4, bool, XYZWBool);

mod const_test_bvec2 {
    const_assert_eq!(
        core::mem::align_of::<bool>(),
        core::mem::align_of::<super::BVec2>()
    );
    const_assert_eq!(2, core::mem::size_of::<super::BVec2>());
}

mod const_test_bvec3 {
    const_assert_eq!(
        core::mem::align_of::<bool>(),
        core::mem::align_of::<super::BVec3>()
    );
    const_assert_eq!(3, core::mem::size_of::<super::BVec3>());
}

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
mod const_test_bvec3a {
    const_assert_eq!(16, core::mem::align_of::<super::BVec3A>());
    const_assert_eq!(16, core::mem::size_of::<super::BVec3A>());
}

mod const_test_bvec4 {
    const_assert_eq!(
        core::mem::align_of::<bool>(),
        core::mem::align_of::<super::BVec4>()
    );
    const_assert_eq!(4, core::mem::size_of::<super::BVec4>());
}

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
mod const_test_bvec4a {
    const_assert_eq!(16, core::mem::align_of::<super::BVec4A>());
    const_assert_eq!(16, core::mem::size_of::<super::BVec4A>());
}
