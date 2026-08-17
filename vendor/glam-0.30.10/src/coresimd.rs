use core::simd::{num::SimdFloat, *};

/// Calculates the vector 3 dot product and returns answer in x lane of f32x4.
#[inline(always)]
pub(crate) fn dot3_in_x(lhs: f32x4, rhs: f32x4) -> f32x4 {
    let x2_y2_z2_w2 = lhs * rhs;
    let y2_0_0_0 = simd_swizzle!(x2_y2_z2_w2, [1, 0, 0, 0]);
    let z2_0_0_0 = simd_swizzle!(x2_y2_z2_w2, [2, 0, 0, 0]);
    let x2y2_0_0_0 = x2_y2_z2_w2 + y2_0_0_0;
    x2y2_0_0_0 + z2_0_0_0
}

/// Calculates the vector 4 dot product and returns answer in x lane of f32x4.
#[inline(always)]
pub(crate) fn dot4_in_x(lhs: f32x4, rhs: f32x4) -> f32x4 {
    let x2_y2_z2_w2 = lhs * rhs;
    let z2_w2_0_0 = simd_swizzle!(x2_y2_z2_w2, [2, 3, 0, 0]);
    let x2z2_y2w2_0_0 = x2_y2_z2_w2 + z2_w2_0_0;
    let y2w2_0_0_0 = simd_swizzle!(x2z2_y2w2_0_0, [1, 0, 0, 0]);
    x2z2_y2w2_0_0 + y2w2_0_0_0
}

#[inline]
pub(crate) fn dot3(lhs: f32x4, rhs: f32x4) -> f32 {
    dot3_in_x(lhs, rhs)[0]
}

#[inline]
pub(crate) fn dot3_into_f32x4(lhs: f32x4, rhs: f32x4) -> f32x4 {
    let dot_in_x = dot3_in_x(lhs, rhs);
    simd_swizzle!(dot_in_x, [0, 0, 0, 0])
}

#[inline]
pub(crate) fn dot4(lhs: f32x4, rhs: f32x4) -> f32 {
    dot4_in_x(lhs, rhs)[0]
}

#[inline]
pub(crate) fn dot4_into_f32x4(lhs: f32x4, rhs: f32x4) -> f32x4 {
    let dot_in_x = dot4_in_x(lhs, rhs);
    simd_swizzle!(dot_in_x, [0, 0, 0, 0])
}

#[inline(always)]
pub(crate) fn f32x4_bitand(a: f32x4, b: f32x4) -> f32x4 {
    let a = a.to_bits();
    let b = b.to_bits();
    f32x4::from_bits(a & b)
}

#[inline(always)]
pub(crate) fn f32x4_bitxor(a: f32x4, b: f32x4) -> f32x4 {
    let a = a.to_bits();
    let b = b.to_bits();
    f32x4::from_bits(a ^ b)
}
