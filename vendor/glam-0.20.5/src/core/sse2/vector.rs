#![allow(clippy::many_single_char_names)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use super::float::*;
use crate::core::{
    storage::{Align16, XY, XYZ, XYZW},
    traits::{scalar::*, vector::*},
};
use core::mem::MaybeUninit;

impl MaskVectorConst for __m128 {
    const FALSE: __m128 = const_f32x4!([0.0; 4]);
}

impl MaskVector for __m128 {
    #[inline(always)]
    fn bitand(self, other: Self) -> Self {
        unsafe { _mm_and_ps(self, other) }
    }

    #[inline(always)]
    fn bitor(self, other: Self) -> Self {
        unsafe { _mm_or_ps(self, other) }
    }

    #[inline]
    fn not(self) -> Self {
        unsafe { _mm_andnot_ps(self, _mm_set_ps1(f32::from_bits(0xff_ff_ff_ff))) }
    }
}

impl MaskVector3 for __m128 {
    #[inline(always)]
    fn new(x: bool, y: bool, z: bool) -> Self {
        // A SSE2 mask can be any bit pattern but for the `MaskVector3` implementation of select we
        // expect either 0 or 0xff_ff_ff_ff. This should be a safe assumption as this type can only
        // be created via this function or by `Vector3` methods.

        unsafe {
            _mm_set_ps(
                0.0,
                f32::from_bits(MaskConst::MASK[z as usize]),
                f32::from_bits(MaskConst::MASK[y as usize]),
                f32::from_bits(MaskConst::MASK[x as usize]),
            )
        }
    }

    #[inline(always)]
    fn bitmask(self) -> u32 {
        unsafe { (_mm_movemask_ps(self) as u32) & 0x7 }
    }

    #[inline(always)]
    fn any(self) -> bool {
        unsafe { (_mm_movemask_ps(self) & 0x7) != 0 }
    }

    #[inline(always)]
    fn all(self) -> bool {
        unsafe { (_mm_movemask_ps(self) & 0x7) == 0x7 }
    }

    #[inline]
    fn into_bool_array(self) -> [bool; 3] {
        let bitmask = MaskVector3::bitmask(self);
        [(bitmask & 1) != 0, (bitmask & 2) != 0, (bitmask & 4) != 0]
    }

    #[inline]
    fn into_u32_array(self) -> [u32; 3] {
        let bitmask = MaskVector3::bitmask(self);
        [
            MaskConst::MASK[(bitmask & 1) as usize],
            MaskConst::MASK[((bitmask >> 1) & 1) as usize],
            MaskConst::MASK[((bitmask >> 2) & 1) as usize],
        ]
    }
}

impl MaskVector4 for __m128 {
    #[inline(always)]
    fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        // A SSE2 mask can be any bit pattern but for the `Vec4Mask` implementation of select we
        // expect either 0 or 0xff_ff_ff_ff. This should be a safe assumption as this type can only
        // be created via this function or by `Vec4` methods.

        const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];
        unsafe {
            _mm_set_ps(
                f32::from_bits(MASK[w as usize]),
                f32::from_bits(MASK[z as usize]),
                f32::from_bits(MASK[y as usize]),
                f32::from_bits(MASK[x as usize]),
            )
        }
    }
    #[inline(always)]
    fn bitmask(self) -> u32 {
        unsafe { _mm_movemask_ps(self) as u32 }
    }

    #[inline(always)]
    fn any(self) -> bool {
        unsafe { _mm_movemask_ps(self) != 0 }
    }

    #[inline(always)]
    fn all(self) -> bool {
        unsafe { _mm_movemask_ps(self) == 0xf }
    }

    #[inline]
    fn into_bool_array(self) -> [bool; 4] {
        let bitmask = MaskVector4::bitmask(self);
        [
            (bitmask & 1) != 0,
            (bitmask & 2) != 0,
            (bitmask & 4) != 0,
            (bitmask & 8) != 0,
        ]
    }

    #[inline]
    fn into_u32_array(self) -> [u32; 4] {
        let bitmask = MaskVector4::bitmask(self);
        [
            MaskConst::MASK[(bitmask & 1) as usize],
            MaskConst::MASK[((bitmask >> 1) & 1) as usize],
            MaskConst::MASK[((bitmask >> 2) & 1) as usize],
            MaskConst::MASK[((bitmask >> 3) & 1) as usize],
        ]
    }
}

/// Calculates the vector 3 dot product and returns answer in x lane of __m128.
#[inline(always)]
unsafe fn dot3_in_x(lhs: __m128, rhs: __m128) -> __m128 {
    let x2_y2_z2_w2 = _mm_mul_ps(lhs, rhs);
    let y2_0_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_00_01);
    let z2_0_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_00_10);
    let x2y2_0_0_0 = _mm_add_ss(x2_y2_z2_w2, y2_0_0_0);
    _mm_add_ss(x2y2_0_0_0, z2_0_0_0)
}

/// Calculates the vector 4 dot product and returns answer in x lane of __m128.
#[inline(always)]
unsafe fn dot4_in_x(lhs: __m128, rhs: __m128) -> __m128 {
    let x2_y2_z2_w2 = _mm_mul_ps(lhs, rhs);
    let z2_w2_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_11_10);
    let x2z2_y2w2_0_0 = _mm_add_ps(x2_y2_z2_w2, z2_w2_0_0);
    let y2w2_0_0_0 = _mm_shuffle_ps(x2z2_y2w2_0_0, x2z2_y2w2_0_0, 0b00_00_00_01);
    _mm_add_ps(x2z2_y2w2_0_0, y2w2_0_0_0)
}

impl VectorConst for __m128 {
    const ZERO: __m128 = const_f32x4!([0.0; 4]);
    const ONE: __m128 = const_f32x4!([1.0; 4]);
}

impl Vector3Const for __m128 {
    const X: __m128 = const_f32x4!([1.0, 0.0, 0.0, 0.0]);
    const Y: __m128 = const_f32x4!([0.0, 1.0, 0.0, 0.0]);
    const Z: __m128 = const_f32x4!([0.0, 0.0, 1.0, 0.0]);
}

impl Vector4Const for __m128 {
    const X: __m128 = const_f32x4!([1.0, 0.0, 0.0, 0.0]);
    const Y: __m128 = const_f32x4!([0.0, 1.0, 0.0, 0.0]);
    const Z: __m128 = const_f32x4!([0.0, 0.0, 1.0, 0.0]);
    const W: __m128 = const_f32x4!([0.0, 0.0, 0.0, 1.0]);
}

impl NanConstEx for __m128 {
    const NAN: __m128 = const_f32x4!([f32::NAN; 4]);
}

impl Vector<f32> for __m128 {
    type Mask = __m128;

    #[inline(always)]
    fn splat(s: f32) -> Self {
        unsafe { _mm_set_ps1(s) }
    }

    #[inline(always)]
    fn select(mask: Self::Mask, if_true: Self, if_false: Self) -> Self {
        unsafe { _mm_or_ps(_mm_andnot_ps(mask, if_false), _mm_and_ps(if_true, mask)) }
    }

    #[inline(always)]
    fn cmpeq(self, other: Self) -> Self::Mask {
        unsafe { _mm_cmpeq_ps(self, other) }
    }

    #[inline(always)]
    fn cmpne(self, other: Self) -> Self::Mask {
        unsafe { _mm_cmpneq_ps(self, other) }
    }

    #[inline(always)]
    fn cmpge(self, other: Self) -> Self::Mask {
        unsafe { _mm_cmpge_ps(self, other) }
    }

    #[inline(always)]
    fn cmpgt(self, other: Self) -> Self::Mask {
        unsafe { _mm_cmpgt_ps(self, other) }
    }

    #[inline(always)]
    fn cmple(self, other: Self) -> Self::Mask {
        unsafe { _mm_cmple_ps(self, other) }
    }

    #[inline(always)]
    fn cmplt(self, other: Self) -> Self::Mask {
        unsafe { _mm_cmplt_ps(self, other) }
    }

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        unsafe { _mm_add_ps(self, other) }
    }

    #[inline(always)]
    fn div(self, other: Self) -> Self {
        unsafe { _mm_div_ps(self, other) }
    }

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        unsafe { _mm_mul_ps(self, other) }
    }

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        unsafe { _mm_sub_ps(self, other) }
    }

    #[inline(always)]
    fn add_scalar(self, other: f32) -> Self {
        unsafe { _mm_add_ps(self, _mm_set_ps1(other)) }
    }

    #[inline(always)]
    fn sub_scalar(self, other: f32) -> Self {
        unsafe { _mm_sub_ps(self, _mm_set_ps1(other)) }
    }

    #[inline(always)]
    fn mul_scalar(self, other: f32) -> Self {
        unsafe { _mm_mul_ps(self, _mm_set_ps1(other)) }
    }

    #[inline(always)]
    fn div_scalar(self, other: f32) -> Self {
        unsafe { _mm_div_ps(self, _mm_set_ps1(other)) }
    }

    #[inline(always)]
    fn rem(self, other: Self) -> Self {
        unsafe {
            let n = m128_floor(_mm_div_ps(self, other));
            _mm_sub_ps(self, _mm_mul_ps(n, other))
        }
    }

    #[inline(always)]
    fn rem_scalar(self, other: f32) -> Self {
        unsafe { self.rem(_mm_set1_ps(other)) }
    }

    #[inline(always)]
    fn min(self, other: Self) -> Self {
        unsafe { _mm_min_ps(self, other) }
    }

    #[inline(always)]
    fn max(self, other: Self) -> Self {
        unsafe { _mm_max_ps(self, other) }
    }
}

impl Vector3<f32> for __m128 {
    #[inline(always)]
    fn new(x: f32, y: f32, z: f32) -> Self {
        unsafe { _mm_set_ps(z, z, y, x) }
    }

    #[inline(always)]
    fn x(self) -> f32 {
        unsafe { _mm_cvtss_f32(self) }
    }

    #[inline(always)]
    fn y(self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self, self, 0b01_01_01_01)) }
    }

    #[inline(always)]
    fn z(self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self, self, 0b10_10_10_10)) }
    }

    #[inline(always)]
    fn splat_x(self) -> Self {
        unsafe { _mm_shuffle_ps(self, self, 0b00_00_00_00) }
    }

    #[inline(always)]
    fn splat_y(self) -> Self {
        unsafe { _mm_shuffle_ps(self, self, 0b01_01_01_01) }
    }

    #[inline(always)]
    fn splat_z(self) -> Self {
        unsafe { _mm_shuffle_ps(self, self, 0b10_10_10_10) }
    }

    #[inline(always)]
    fn from_slice_unaligned(slice: &[f32]) -> Self {
        Vector3::new(slice[0], slice[1], slice[2])
    }

    #[inline(always)]
    fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        let xyz = self.as_ref_xyz();
        slice[0] = xyz.x;
        slice[1] = xyz.y;
        slice[2] = xyz.z;
    }

    #[inline(always)]
    fn as_ref_xyz(&self) -> &XYZ<f32> {
        unsafe { &*(self as *const Self).cast() }
    }

    #[inline(always)]
    fn as_mut_xyz(&mut self) -> &mut XYZ<f32> {
        unsafe { &mut *(self as *mut Self).cast() }
    }

    #[inline(always)]
    fn into_xy(self) -> XY<f32> {
        let mut out: MaybeUninit<Align16<XY<f32>>> = MaybeUninit::uninit();
        unsafe {
            _mm_store_ps(out.as_mut_ptr().cast(), self);
            out.assume_init().0
        }
    }

    #[inline]
    fn into_xyzw(self, w: f32) -> XYZW<f32> {
        unsafe {
            let mut t = _mm_move_ss(self, _mm_set_ss(w));
            t = _mm_shuffle_ps(t, t, 0b00_10_01_00);
            // TODO: need a SIMD path
            *_mm_move_ss(t, self).as_ref_xyzw()
        }
    }

    #[inline(always)]
    fn from_array(a: [f32; 3]) -> Self {
        unsafe { _mm_set_ps(a[2], a[2], a[1], a[0]) }
    }

    #[inline(always)]
    fn into_array(self) -> [f32; 3] {
        let mut out: MaybeUninit<Align16<[f32; 3]>> = MaybeUninit::uninit();
        unsafe {
            _mm_store_ps(out.as_mut_ptr().cast(), self);
            out.assume_init().0
        }
    }

    #[inline(always)]
    fn from_tuple(t: (f32, f32, f32)) -> Self {
        unsafe { _mm_set_ps(t.2, t.2, t.1, t.0) }
    }

    #[inline(always)]
    fn into_tuple(self) -> (f32, f32, f32) {
        let mut out: MaybeUninit<Align16<(f32, f32, f32)>> = MaybeUninit::uninit();
        unsafe {
            _mm_store_ps(out.as_mut_ptr().cast(), self);
            out.assume_init().0
        }
    }

    #[inline]
    fn min_element(self) -> f32 {
        unsafe {
            let v = self;
            let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b01_01_10_10));
            let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
            _mm_cvtss_f32(v)
        }
    }

    #[inline]
    fn max_element(self) -> f32 {
        unsafe {
            let v = self;
            let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_10_10));
            let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
            _mm_cvtss_f32(v)
        }
    }

    #[inline]
    fn dot(self, other: Self) -> f32 {
        unsafe { _mm_cvtss_f32(dot3_in_x(self, other)) }
    }

    #[inline]
    fn dot_into_vec(self, other: Self) -> Self {
        unsafe {
            let dot_in_x = dot3_in_x(self, other);
            _mm_shuffle_ps(dot_in_x, dot_in_x, 0b00_00_00_00)
        }
    }

    #[inline]
    fn cross(self, other: Self) -> Self {
        unsafe {
            // x  <-  a.y*b.z - a.z*b.y
            // y  <-  a.z*b.x - a.x*b.z
            // z  <-  a.x*b.y - a.y*b.x
            // We can save a shuffle by grouping it in this wacky order:
            // (self.zxy() * other - self * other.zxy()).zxy()
            let lhszxy = _mm_shuffle_ps(self, self, 0b01_01_00_10);
            let rhszxy = _mm_shuffle_ps(other, other, 0b01_01_00_10);
            let lhszxy_rhs = _mm_mul_ps(lhszxy, other);
            let rhszxy_lhs = _mm_mul_ps(rhszxy, self);
            let sub = _mm_sub_ps(lhszxy_rhs, rhszxy_lhs);
            _mm_shuffle_ps(sub, sub, 0b01_01_00_10)
        }
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        glam_assert!(
            MaskVector3::all(min.cmple(max)),
            "clamp: expected min <= max"
        );
        self.max(min).min(max)
    }
}

impl Vector4<f32> for __m128 {
    #[inline(always)]
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        unsafe { _mm_set_ps(w, z, y, x) }
    }

    #[inline(always)]
    fn x(self) -> f32 {
        unsafe { _mm_cvtss_f32(self) }
    }

    #[inline(always)]
    fn y(self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self, self, 0b01_01_01_01)) }
    }

    #[inline(always)]
    fn z(self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self, self, 0b10_10_10_10)) }
    }

    #[inline(always)]
    fn w(self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self, self, 0b11_11_11_11)) }
    }

    #[inline(always)]
    fn splat_x(self) -> Self {
        unsafe { _mm_shuffle_ps(self, self, 0b00_00_00_00) }
    }

    #[inline(always)]
    fn splat_y(self) -> Self {
        unsafe { _mm_shuffle_ps(self, self, 0b01_01_01_01) }
    }

    #[inline(always)]
    fn splat_z(self) -> Self {
        unsafe { _mm_shuffle_ps(self, self, 0b10_10_10_10) }
    }

    #[inline(always)]
    fn splat_w(self) -> Self {
        unsafe { _mm_shuffle_ps(self, self, 0b11_11_11_11) }
    }

    #[inline(always)]
    fn from_slice_unaligned(slice: &[f32]) -> Self {
        assert!(slice.len() >= 4);
        unsafe { _mm_loadu_ps(slice.as_ptr()) }
    }

    #[inline(always)]
    fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        unsafe {
            assert!(slice.len() >= 4);
            _mm_storeu_ps(slice.as_mut_ptr(), self);
        }
    }

    #[inline(always)]
    fn as_ref_xyzw(&self) -> &XYZW<f32> {
        unsafe { &*(self as *const Self).cast() }
    }

    #[inline(always)]
    fn as_mut_xyzw(&mut self) -> &mut XYZW<f32> {
        unsafe { &mut *(self as *mut Self).cast() }
    }

    #[inline(always)]
    fn into_xy(self) -> XY<f32> {
        let mut out: MaybeUninit<Align16<XY<f32>>> = MaybeUninit::uninit();
        unsafe {
            _mm_store_ps(out.as_mut_ptr().cast(), self);
            out.assume_init().0
        }
    }

    #[inline(always)]
    fn into_xyz(self) -> XYZ<f32> {
        let mut out: MaybeUninit<Align16<XYZ<f32>>> = MaybeUninit::uninit();
        unsafe {
            _mm_store_ps(out.as_mut_ptr().cast(), self);
            out.assume_init().0
        }
    }

    #[inline(always)]
    fn from_array(a: [f32; 4]) -> Self {
        unsafe { _mm_loadu_ps(a.as_ptr()) }
    }

    #[inline(always)]
    fn into_array(self) -> [f32; 4] {
        let mut out: MaybeUninit<Align16<[f32; 4]>> = MaybeUninit::uninit();
        unsafe {
            _mm_store_ps(out.as_mut_ptr().cast(), self);
            out.assume_init().0
        }
    }

    #[inline(always)]
    fn from_tuple(t: (f32, f32, f32, f32)) -> Self {
        unsafe { _mm_set_ps(t.3, t.2, t.1, t.0) }
    }

    #[inline(always)]
    fn into_tuple(self) -> (f32, f32, f32, f32) {
        let mut out: MaybeUninit<Align16<(f32, f32, f32, f32)>> = MaybeUninit::uninit();
        unsafe {
            _mm_store_ps(out.as_mut_ptr().cast(), self);
            out.assume_init().0
        }
    }

    #[inline]
    fn min_element(self) -> f32 {
        unsafe {
            let v = self;
            let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_11_10));
            let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
            _mm_cvtss_f32(v)
        }
    }

    #[inline]
    fn max_element(self) -> f32 {
        unsafe {
            let v = self;
            let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_11_10));
            let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
            _mm_cvtss_f32(v)
        }
    }

    #[inline]
    fn dot(self, other: Self) -> f32 {
        unsafe { _mm_cvtss_f32(dot4_in_x(self, other)) }
    }

    #[inline]
    fn dot_into_vec(self, other: Self) -> Self {
        unsafe {
            let dot_in_x = dot4_in_x(self, other);
            _mm_shuffle_ps(dot_in_x, dot_in_x, 0b00_00_00_00)
        }
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        glam_assert!(
            MaskVector4::all(min.cmple(max)),
            "clamp: expected min <= max"
        );
        self.max(min).min(max)
    }
}

impl SignedVector<f32> for __m128 {
    #[inline(always)]
    fn neg(self) -> Self {
        unsafe { _mm_sub_ps(Self::ZERO, self) }
    }
}

impl SignedVector3<f32> for __m128 {
    #[inline]
    fn abs(self) -> Self {
        unsafe { m128_abs(self) }
    }

    #[inline]
    fn signum(self) -> Self {
        const NEG_ONE: __m128 = const_f32x4!([-1.0; 4]);
        let mask = self.cmpge(Self::ZERO);
        let result = Self::select(mask, Self::ONE, NEG_ONE);
        let mask = unsafe { _mm_cmpunord_ps(self, self) };
        Self::select(mask, self, result)
    }
}

impl FloatVector3<f32> for __m128 {
    #[inline]
    fn is_finite(self) -> bool {
        let (x, y, z) = Vector3::into_tuple(self);
        x.is_finite() && y.is_finite() && z.is_finite()
    }

    #[inline]
    fn is_nan(self) -> bool {
        MaskVector3::any(FloatVector3::is_nan_mask(self))
    }

    #[inline(always)]
    fn is_nan_mask(self) -> Self::Mask {
        unsafe { _mm_cmpunord_ps(self, self) }
    }

    #[inline(always)]
    #[cfg(target_feature = "fma")]
    fn mul_add(self, b: Self, c: Self) -> Self {
        unsafe { _mm_fmadd_ps(self, b, c) }
    }

    #[inline]
    fn floor(self) -> Self {
        unsafe { m128_floor(self) }
    }

    #[inline]
    fn ceil(self) -> Self {
        unsafe { m128_ceil(self) }
    }

    #[inline]
    fn round(self) -> Self {
        unsafe { m128_round(self) }
    }

    #[inline(always)]
    fn recip(self) -> Self {
        unsafe { _mm_div_ps(Self::ONE, self) }
    }

    #[inline]
    fn exp(self) -> Self {
        let (x, y, z) = Vector3::into_tuple(self);
        unsafe { _mm_set_ps(0.0, z.exp(), y.exp(), x.exp()) }
    }

    #[inline]
    fn powf(self, n: f32) -> Self {
        let (x, y, z) = Vector3::into_tuple(self);
        unsafe { _mm_set_ps(0.0, z.powf(n), y.powf(n), x.powf(n)) }
    }

    #[inline]
    fn length(self) -> f32 {
        unsafe {
            let dot = dot3_in_x(self, self);
            _mm_cvtss_f32(_mm_sqrt_ps(dot))
        }
    }

    #[inline]
    fn length_recip(self) -> f32 {
        unsafe {
            let dot = dot3_in_x(self, self);
            _mm_cvtss_f32(_mm_div_ps(Self::ONE, _mm_sqrt_ps(dot)))
        }
    }

    #[inline]
    fn normalize(self) -> Self {
        unsafe {
            let length = _mm_sqrt_ps(Vector3::dot_into_vec(self, self));
            #[allow(clippy::let_and_return)]
            let normalized = _mm_div_ps(self, length);
            glam_assert!(FloatVector3::is_finite(normalized));
            normalized
        }
    }
}

impl SignedVector4<f32> for __m128 {
    #[inline]
    fn abs(self) -> Self {
        unsafe { m128_abs(self) }
    }

    #[inline]
    fn signum(self) -> Self {
        const NEG_ONE: __m128 = const_f32x4!([-1.0; 4]);
        let mask = self.cmpge(Self::ZERO);
        let result = Self::select(mask, Self::ONE, NEG_ONE);
        let mask = unsafe { _mm_cmpunord_ps(self, self) };
        Self::select(mask, self, result)
    }
}

impl FloatVector4<f32> for __m128 {
    #[inline]
    fn is_finite(self) -> bool {
        let (x, y, z, w) = Vector4::into_tuple(self);
        x.is_finite() && y.is_finite() && z.is_finite() && w.is_finite()
    }

    #[inline]
    fn is_nan(self) -> bool {
        MaskVector4::any(FloatVector4::is_nan_mask(self))
    }

    #[inline(always)]
    fn is_nan_mask(self) -> Self::Mask {
        unsafe { _mm_cmpunord_ps(self, self) }
    }

    #[inline(always)]
    #[cfg(target_feature = "fma")]
    fn mul_add(self, b: Self, c: Self) -> Self {
        unsafe { _mm_fmadd_ps(self, b, c) }
    }

    #[inline]
    fn floor(self) -> Self {
        unsafe { m128_floor(self) }
    }

    #[inline]
    fn ceil(self) -> Self {
        unsafe { m128_ceil(self) }
    }

    #[inline]
    fn round(self) -> Self {
        unsafe { m128_round(self) }
    }

    #[inline(always)]
    fn recip(self) -> Self {
        unsafe { _mm_div_ps(Self::ONE, self) }
    }

    #[inline]
    fn exp(self) -> Self {
        let (x, y, z, w) = Vector4::into_tuple(self);
        unsafe { _mm_set_ps(w.exp(), z.exp(), y.exp(), x.exp()) }
    }

    #[inline]
    fn powf(self, n: f32) -> Self {
        let (x, y, z, w) = Vector4::into_tuple(self);
        unsafe { _mm_set_ps(w.powf(n), z.powf(n), y.powf(n), x.powf(n)) }
    }

    #[inline]
    fn length(self) -> f32 {
        unsafe {
            let dot = dot4_in_x(self, self);
            _mm_cvtss_f32(_mm_sqrt_ps(dot))
        }
    }

    #[inline]
    fn length_recip(self) -> f32 {
        unsafe {
            let dot = dot4_in_x(self, self);
            _mm_cvtss_f32(_mm_div_ps(Self::ONE, _mm_sqrt_ps(dot)))
        }
    }

    #[inline]
    fn normalize(self) -> Self {
        unsafe {
            let dot = Vector4::dot_into_vec(self, self);
            #[allow(clippy::let_and_return)]
            let normalized = _mm_div_ps(self, _mm_sqrt_ps(dot));
            glam_assert!(FloatVector4::is_finite(normalized));
            normalized
        }
    }
}

impl From<XYZW<f32>> for __m128 {
    #[inline(always)]
    fn from(v: XYZW<f32>) -> __m128 {
        unsafe { _mm_set_ps(v.w, v.z, v.y, v.x) }
    }
}

impl From<XYZ<f32>> for __m128 {
    #[inline(always)]
    fn from(v: XYZ<f32>) -> __m128 {
        unsafe { _mm_set_ps(v.z, v.z, v.y, v.x) }
    }
}

impl From<XY<f32>> for __m128 {
    #[inline(always)]
    fn from(v: XY<f32>) -> __m128 {
        unsafe { _mm_set_ps(v.y, v.y, v.y, v.x) }
    }
}

impl From<__m128> for XYZW<f32> {
    #[inline(always)]
    fn from(v: __m128) -> XYZW<f32> {
        let mut out: MaybeUninit<Align16<XYZW<f32>>> = MaybeUninit::uninit();
        unsafe {
            _mm_store_ps(out.as_mut_ptr().cast(), v);
            out.assume_init().0
        }
    }
}

impl From<__m128> for XYZ<f32> {
    #[inline(always)]
    fn from(v: __m128) -> XYZ<f32> {
        let mut out: MaybeUninit<Align16<XYZ<f32>>> = MaybeUninit::uninit();
        unsafe {
            _mm_store_ps(out.as_mut_ptr().cast(), v);
            out.assume_init().0
        }
    }
}

impl From<__m128> for XY<f32> {
    #[inline(always)]
    fn from(v: __m128) -> XY<f32> {
        let mut out: MaybeUninit<Align16<XY<f32>>> = MaybeUninit::uninit();
        unsafe {
            _mm_store_ps(out.as_mut_ptr().cast(), v);
            out.assume_init().0
        }
    }
}
