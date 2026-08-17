use core::arch::aarch64::*;

union UnionCast {
    // u32x4: [u32; 4],
    f32x4: [f32; 4],
    v: float32x4_t,
}

#[inline]
pub const fn f32x4_from_array(f32x4: [f32; 4]) -> float32x4_t {
    unsafe { UnionCast { f32x4 }.v }
}

// #[inline]
// pub(crate) unsafe fn dot3_in_x(lhs: float32x4_t, rhs: float32x4_t) -> float32x4_t {
//     let x2_y2_z2_w2 = vmulq_f32(lhs, rhs);
//     let y2 = vdupq_laneq_f32(x2_y2_z2_w2, 1);
//     let z2 = vdupq_laneq_f32(x2_y2_z2_w2, 2);
//     let x2y2 = vaddq_f32(x2_y2_z2_w2, y2);
//     vaddq_f32(x2y2, z2)
// }

#[inline]
pub(crate) unsafe fn dot3(lhs: float32x4_t, rhs: float32x4_t) -> f32 {
    let x2_y2_z2_w2 = vmulq_f32(lhs, rhs);
    let x2_y2_z2 = vsetq_lane_f32(0.0, x2_y2_z2_w2, 3);
    vaddvq_f32(x2_y2_z2)
    // let dot = dot3_in_x(lhs, rhs);
    // vdups_laneq_f32(dot, 0)
}

#[inline]
pub(crate) unsafe fn dot3_into_f32x4(lhs: float32x4_t, rhs: float32x4_t) -> float32x4_t {
    let dot = dot3(lhs, rhs);
    vld1q_dup_f32(&dot as *const f32)
    // let dot = dot3_in_x(lhs, rhs);
    // vdupq_laneq_f32(dot, 0)
}

#[inline]
pub(crate) unsafe fn dot4(lhs: float32x4_t, rhs: float32x4_t) -> f32 {
    let x2_y2_z2_w2 = vmulq_f32(lhs, rhs);
    // TODO: horizontal add - might perform bad?
    vaddvq_f32(x2_y2_z2_w2)
}

#[inline]
pub(crate) unsafe fn dot4_into_f32x4(lhs: float32x4_t, rhs: float32x4_t) -> float32x4_t {
    let dot = dot4(lhs, rhs);
    vld1q_dup_f32(&dot as *const f32)
}
