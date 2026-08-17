use core::arch::wasm32::*;

pub const fn v128_from_f32x4(a: [f32; 4]) -> v128 {
    f32x4(a[0], a[1], a[2], a[3])
}

/// Calculates the vector 3 dot product and returns answer in x lane of v128.
#[inline(always)]
pub(crate) fn dot3_in_x(lhs: v128, rhs: v128) -> v128 {
    let x2_y2_z2_w2 = f32x4_mul(lhs, rhs);
    let y2_0_0_0 = i32x4_shuffle::<1, 0, 0, 0>(x2_y2_z2_w2, x2_y2_z2_w2);
    let z2_0_0_0 = i32x4_shuffle::<2, 0, 0, 0>(x2_y2_z2_w2, x2_y2_z2_w2);
    let x2y2_0_0_0 = f32x4_add(x2_y2_z2_w2, y2_0_0_0);
    f32x4_add(x2y2_0_0_0, z2_0_0_0)
}

/// Calculates the vector 4 dot product and returns answer in x lane of v128.
#[inline(always)]
pub(crate) fn dot4_in_x(lhs: v128, rhs: v128) -> v128 {
    let x2_y2_z2_w2 = f32x4_mul(lhs, rhs);
    let z2_w2_0_0 = i32x4_shuffle::<2, 3, 0, 0>(x2_y2_z2_w2, x2_y2_z2_w2);
    let x2z2_y2w2_0_0 = f32x4_add(x2_y2_z2_w2, z2_w2_0_0);
    let y2w2_0_0_0 = i32x4_shuffle::<1, 0, 0, 0>(x2z2_y2w2_0_0, x2z2_y2w2_0_0);
    f32x4_add(x2z2_y2w2_0_0, y2w2_0_0_0)
}

#[inline]
pub(crate) fn dot3(lhs: v128, rhs: v128) -> f32 {
    f32x4_extract_lane::<0>(dot3_in_x(lhs, rhs))
}

#[inline]
pub(crate) fn dot3_into_v128(lhs: v128, rhs: v128) -> v128 {
    let dot_in_x = dot3_in_x(lhs, rhs);
    i32x4_shuffle::<0, 0, 0, 0>(dot_in_x, dot_in_x)
}

#[inline]
pub(crate) fn dot4(lhs: v128, rhs: v128) -> f32 {
    f32x4_extract_lane::<0>(dot4_in_x(lhs, rhs))
}

#[inline]
pub(crate) fn dot4_into_v128(lhs: v128, rhs: v128) -> v128 {
    let dot_in_x = dot4_in_x(lhs, rhs);
    i32x4_shuffle::<0, 0, 0, 0>(dot_in_x, dot_in_x)
}
