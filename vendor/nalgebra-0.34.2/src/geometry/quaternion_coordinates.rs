use std::ops::{Deref, DerefMut};

use simba::simd::SimdValue;

use crate::Scalar;
use crate::base::coordinates::IJKW;

use crate::geometry::Quaternion;

impl<T: Scalar + SimdValue> Deref for Quaternion<T> {
    type Target = IJKW<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

impl<T: Scalar + SimdValue> DerefMut for Quaternion<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}
