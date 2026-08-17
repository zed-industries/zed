use super::*;

pick! {
  if #[cfg(target_feature="sse")] {
    #[derive(Default, Clone, Copy, PartialEq)]
    #[repr(C, align(16))]
    pub struct f32x4 { pub(crate) sse: m128 }
  } else if #[cfg(target_feature="simd128")] {
    use core::arch::wasm32::*;

    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct f32x4 { pub(crate) simd: v128 }

    impl Default for f32x4 {
      fn default() -> Self {
        Self::splat(0.0)
      }
    }

    impl PartialEq for f32x4 {
      fn eq(&self, other: &Self) -> bool {
        u32x4_all_true(f32x4_eq(self.simd, other.simd))
      }
    }
  } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
    use core::arch::aarch64::*;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct f32x4 { pub(crate) neon : float32x4_t }

    impl Default for f32x4 {
      #[inline]
      #[must_use]
      fn default() -> Self {
        unsafe { Self { neon: vdupq_n_f32(0.0)} }
      }
    }

    impl PartialEq for f32x4 {
      #[inline]
      #[must_use]
      fn eq(&self, other: &Self) -> bool {
        unsafe { vminvq_u32(vceqq_f32(self.neon, other.neon))==u32::MAX }
      }

    }
    } else {
    #[derive(Default, Clone, Copy, PartialEq)]
    #[repr(C, align(16))]
    pub struct f32x4 { pub(crate) arr: [f32;4] }
  }
}

macro_rules! const_f32_as_f32x4 {
  ($i:ident, $f:expr) => {
    #[allow(non_upper_case_globals)]
    pub const $i: f32x4 = f32x4::new([$f; 4]);
  };
}

impl f32x4 {
  const_f32_as_f32x4!(ONE, 1.0);
  const_f32_as_f32x4!(ZERO, 0.0);
  const_f32_as_f32x4!(HALF, 0.5);
  const_f32_as_f32x4!(E, core::f32::consts::E);
  const_f32_as_f32x4!(FRAC_1_PI, core::f32::consts::FRAC_1_PI);
  const_f32_as_f32x4!(FRAC_2_PI, core::f32::consts::FRAC_2_PI);
  const_f32_as_f32x4!(FRAC_2_SQRT_PI, core::f32::consts::FRAC_2_SQRT_PI);
  const_f32_as_f32x4!(FRAC_1_SQRT_2, core::f32::consts::FRAC_1_SQRT_2);
  const_f32_as_f32x4!(FRAC_PI_2, core::f32::consts::FRAC_PI_2);
  const_f32_as_f32x4!(FRAC_PI_3, core::f32::consts::FRAC_PI_3);
  const_f32_as_f32x4!(FRAC_PI_4, core::f32::consts::FRAC_PI_4);
  const_f32_as_f32x4!(FRAC_PI_6, core::f32::consts::FRAC_PI_6);
  const_f32_as_f32x4!(FRAC_PI_8, core::f32::consts::FRAC_PI_8);
  const_f32_as_f32x4!(LN_2, core::f32::consts::LN_2);
  const_f32_as_f32x4!(LN_10, core::f32::consts::LN_10);
  const_f32_as_f32x4!(LOG2_E, core::f32::consts::LOG2_E);
  const_f32_as_f32x4!(LOG10_E, core::f32::consts::LOG10_E);
  const_f32_as_f32x4!(LOG10_2, core::f32::consts::LOG10_2);
  const_f32_as_f32x4!(LOG2_10, core::f32::consts::LOG2_10);
  const_f32_as_f32x4!(PI, core::f32::consts::PI);
  const_f32_as_f32x4!(SQRT_2, core::f32::consts::SQRT_2);
  const_f32_as_f32x4!(TAU, core::f32::consts::TAU);
}

unsafe impl Zeroable for f32x4 {}
unsafe impl Pod for f32x4 {}

impl Add for f32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: add_m128(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_add(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vaddq_f32(self.neon, rhs.neon) } }
      } else {
        Self { arr: [
          self.arr[0] + rhs.arr[0],
          self.arr[1] + rhs.arr[1],
          self.arr[2] + rhs.arr[2],
          self.arr[3] + rhs.arr[3],
        ]}
      }
    }
  }
}

impl Sub for f32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: sub_m128(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_sub(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vsubq_f32(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0] - rhs.arr[0],
          self.arr[1] - rhs.arr[1],
          self.arr[2] - rhs.arr[2],
          self.arr[3] - rhs.arr[3],
        ]}
      }
    }
  }
}

impl Mul for f32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: mul_m128(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_mul(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vmulq_f32(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0] * rhs.arr[0],
          self.arr[1] * rhs.arr[1],
          self.arr[2] * rhs.arr[2],
          self.arr[3] * rhs.arr[3],
        ]}
      }
    }
  }
}

impl Div for f32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn div(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: div_m128(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_div(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vdivq_f32(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0] / rhs.arr[0],
          self.arr[1] / rhs.arr[1],
          self.arr[2] / rhs.arr[2],
          self.arr[3] / rhs.arr[3],
        ]}
      }
    }
  }
}

impl Add<f32> for f32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: f32) -> Self::Output {
    self.add(Self::splat(rhs))
  }
}

impl Sub<f32> for f32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: f32) -> Self::Output {
    self.sub(Self::splat(rhs))
  }
}

impl Mul<f32> for f32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: f32) -> Self::Output {
    self.mul(Self::splat(rhs))
  }
}

impl Div<f32> for f32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn div(self, rhs: f32) -> Self::Output {
    self.div(Self::splat(rhs))
  }
}

impl Add<f32x4> for f32 {
  type Output = f32x4;
  #[inline]
  #[must_use]
  fn add(self, rhs: f32x4) -> Self::Output {
    f32x4::splat(self).add(rhs)
  }
}

impl Sub<f32x4> for f32 {
  type Output = f32x4;
  #[inline]
  #[must_use]
  fn sub(self, rhs: f32x4) -> Self::Output {
    f32x4::splat(self).sub(rhs)
  }
}

impl Mul<f32x4> for f32 {
  type Output = f32x4;
  #[inline]
  #[must_use]
  fn mul(self, rhs: f32x4) -> Self::Output {
    f32x4::splat(self).mul(rhs)
  }
}

impl Div<f32x4> for f32 {
  type Output = f32x4;
  #[inline]
  #[must_use]
  fn div(self, rhs: f32x4) -> Self::Output {
    f32x4::splat(self).div(rhs)
  }
}

impl BitAnd for f32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitand(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: bitand_m128(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: v128_and(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f32_u32(vandq_u32(vreinterpretq_u32_f32(self.neon), vreinterpretq_u32_f32(rhs.neon))) }}
      } else {
        Self { arr: [
          f32::from_bits(self.arr[0].to_bits() & rhs.arr[0].to_bits()),
          f32::from_bits(self.arr[1].to_bits() & rhs.arr[1].to_bits()),
          f32::from_bits(self.arr[2].to_bits() & rhs.arr[2].to_bits()),
          f32::from_bits(self.arr[3].to_bits() & rhs.arr[3].to_bits()),
        ]}
      }
    }
  }
}

impl BitOr for f32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitor(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: bitor_m128(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: v128_or(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f32_u32(vorrq_u32(vreinterpretq_u32_f32(self.neon), vreinterpretq_u32_f32(rhs.neon))) }}
      } else {
        Self { arr: [
          f32::from_bits(self.arr[0].to_bits() | rhs.arr[0].to_bits()),
          f32::from_bits(self.arr[1].to_bits() | rhs.arr[1].to_bits()),
          f32::from_bits(self.arr[2].to_bits() | rhs.arr[2].to_bits()),
          f32::from_bits(self.arr[3].to_bits() | rhs.arr[3].to_bits()),
        ]}
      }
    }
  }
}

impl BitXor for f32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitxor(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: bitxor_m128(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: v128_xor(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f32_u32(veorq_u32(vreinterpretq_u32_f32(self.neon), vreinterpretq_u32_f32(rhs.neon))) }}
      } else {
        Self { arr: [
          f32::from_bits(self.arr[0].to_bits() ^ rhs.arr[0].to_bits()),
          f32::from_bits(self.arr[1].to_bits() ^ rhs.arr[1].to_bits()),
          f32::from_bits(self.arr[2].to_bits() ^ rhs.arr[2].to_bits()),
          f32::from_bits(self.arr[3].to_bits() ^ rhs.arr[3].to_bits()),
        ]}
      }
    }
  }
}

impl CmpEq for f32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_eq(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: cmp_eq_mask_m128(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_eq(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f32_u32(vceqq_f32(self.neon, rhs.neon)) }}
      } else {
        Self { arr: [
          if self.arr[0] == rhs.arr[0] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[1] == rhs.arr[1] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[2] == rhs.arr[2] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[3] == rhs.arr[3] { f32::from_bits(u32::MAX) } else { 0.0 },
        ]}
      }
    }
  }
}

impl CmpGe for f32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_ge(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: cmp_ge_mask_m128(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_ge(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f32_u32(vcgeq_f32(self.neon, rhs.neon)) }}
      } else {
        Self { arr: [
          if self.arr[0] >= rhs.arr[0] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[1] >= rhs.arr[1] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[2] >= rhs.arr[2] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[3] >= rhs.arr[3] { f32::from_bits(u32::MAX) } else { 0.0 },
        ]}
      }
    }
  }
}

impl CmpGt for f32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_gt(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: cmp_gt_mask_m128(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_gt(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f32_u32(vcgtq_f32(self.neon, rhs.neon)) }}
      } else {
        Self { arr: [
          if self.arr[0] > rhs.arr[0] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[1] > rhs.arr[1] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[2] > rhs.arr[2] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[3] > rhs.arr[3] { f32::from_bits(u32::MAX) } else { 0.0 },
        ]}
      }
    }
  }
}

impl CmpNe for f32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_ne(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: cmp_neq_mask_m128(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_ne(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f32_u32(vmvnq_u32(vceqq_f32(self.neon, rhs.neon))) }}
      } else {
        Self { arr: [
          if self.arr[0] != rhs.arr[0] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[1] != rhs.arr[1] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[2] != rhs.arr[2] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[3] != rhs.arr[3] { f32::from_bits(u32::MAX) } else { 0.0 },
        ]}
      }
    }
  }
}

impl CmpLe for f32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_le(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: cmp_le_mask_m128(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_le(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f32_u32(vcleq_f32(self.neon, rhs.neon)) }}
      } else {
        Self { arr: [
          if self.arr[0] <= rhs.arr[0] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[1] <= rhs.arr[1] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[2] <= rhs.arr[2] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[3] <= rhs.arr[3] { f32::from_bits(u32::MAX) } else { 0.0 },
        ]}
      }
    }
  }
}

impl CmpLt for f32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_lt(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: cmp_lt_mask_m128(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_lt(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f32_u32(vcltq_f32(self.neon, rhs.neon)) }}
      } else {
        Self { arr: [
          if self.arr[0] < rhs.arr[0] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[1] < rhs.arr[1] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[2] < rhs.arr[2] { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[3] < rhs.arr[3] { f32::from_bits(u32::MAX) } else { 0.0 },
        ]}
      }
    }
  }
}

impl f32x4 {
  #[inline]
  #[must_use]
  pub const fn new(array: [f32; 4]) -> Self {
    #[allow(non_upper_case_globals)]
    unsafe {
      core::mem::transmute(array)
    }
  }

  #[inline]
  #[must_use]
  pub fn blend(self, t: Self, f: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse4.1")] {
        Self { sse: blend_varying_m128(f.sse, t.sse, self.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: v128_bitselect(t.simd, f.simd, self.simd) }
      } else {
        generic_bit_blend(self, t, f)
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn abs(self) -> Self {
    pick! {
      if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_abs(self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vabsq_f32(self.neon) }}
      } else {
        let non_sign_bits = f32x4::from(f32::from_bits(i32::MAX as u32));
        self & non_sign_bits
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn floor(self) -> Self {
    pick! {
      if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_floor(self.simd) }
      } else if #[cfg(target_feature="sse4.1")] {
        Self { sse: floor_m128(self.sse) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vrndmq_f32(self.neon) }}
      } else if #[cfg(feature="std")] {
        let base: [f32; 4] = cast(self);
        cast(base.map(|val| val.floor()))
      } else {
        let base: [f32; 4] = cast(self);
        let rounded: [f32; 4] = cast(self.round());
        cast([
          if base[0] < rounded[0] { rounded[0] - 1.0 } else { rounded[0] },
          if base[1] < rounded[1] { rounded[1] - 1.0 } else { rounded[1] },
          if base[2] < rounded[2] { rounded[2] - 1.0 } else { rounded[2] },
          if base[3] < rounded[3] { rounded[3] - 1.0 } else { rounded[3] },
        ])
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn ceil(self) -> Self {
    pick! {
      if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_ceil(self.simd) }
      } else if #[cfg(target_feature="sse4.1")] {
        Self { sse: ceil_m128(self.sse) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vrndpq_f32(self.neon) }}
      } else if #[cfg(feature="std")] {
        let base: [f32; 4] = cast(self);
        cast(base.map(|val| val.ceil()))
      } else {
        let base: [f32; 4] = cast(self);
        let rounded: [f32; 4] = cast(self.round());
        cast([
          if base[0] > rounded[0] { rounded[0] + 1.0 } else { rounded[0] },
          if base[1] > rounded[1] { rounded[1] + 1.0 } else { rounded[1] },
          if base[2] > rounded[2] { rounded[2] + 1.0 } else { rounded[2] },
          if base[3] > rounded[3] { rounded[3] + 1.0 } else { rounded[3] },
        ])
      }
    }
  }

  /// Calculates the lanewise maximum of both vectors. This is a faster
  /// implementation than `max`, but it doesn't specify any behavior if NaNs are
  /// involved.
  #[inline]
  #[must_use]
  pub fn fast_max(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: max_m128(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self {
          simd: f32x4_pmax(self.simd, rhs.simd),
        }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vmaxq_f32(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          if self.arr[0] < rhs.arr[0] { rhs.arr[0] } else { self.arr[0] },
          if self.arr[1] < rhs.arr[1] { rhs.arr[1] } else { self.arr[1] },
          if self.arr[2] < rhs.arr[2] { rhs.arr[2] } else { self.arr[2] },
          if self.arr[3] < rhs.arr[3] { rhs.arr[3] } else { self.arr[3] },
        ]}
      }
    }
  }

  /// Calculates the lanewise maximum of both vectors. If either lane is NaN,
  /// the other lane gets chosen. Use `fast_max` for a faster implementation
  /// that doesn't handle NaNs.
  #[inline]
  #[must_use]
  pub fn max(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse")] {
        // max_m128 seems to do rhs < self ? self : rhs. So if there's any NaN
        // involved, it chooses rhs, so we need to specifically check rhs for
        // NaN.
        rhs.is_nan().blend(self, Self { sse: max_m128(self.sse, rhs.sse) })
      } else if #[cfg(target_feature="simd128")] {
        // WASM has two max intrinsics:
        // - max: This propagates NaN, that's the opposite of what we need.
        // - pmax: This is defined as self < rhs ? rhs : self, which basically
        //   chooses self if either is NaN.
        //
        // pmax is what we want, but we need to specifically check self for NaN.
        Self {
          simd: v128_bitselect(
            rhs.simd,
            f32x4_pmax(self.simd, rhs.simd),
            f32x4_ne(self.simd, self.simd), // NaN check
          )
        }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vmaxnmq_f32(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].max(rhs.arr[0]),
          self.arr[1].max(rhs.arr[1]),
          self.arr[2].max(rhs.arr[2]),
          self.arr[3].max(rhs.arr[3]),
        ]}
      }
    }
  }

  /// Calculates the lanewise minimum of both vectors. This is a faster
  /// implementation than `min`, but it doesn't specify any behavior if NaNs are
  /// involved.
  #[inline]
  #[must_use]
  pub fn fast_min(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: min_m128(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self {
          simd: f32x4_pmin(self.simd, rhs.simd),
        }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vminq_f32(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          if self.arr[0] < rhs.arr[0] { self.arr[0] } else { rhs.arr[0] },
          if self.arr[1] < rhs.arr[1] { self.arr[1] } else { rhs.arr[1] },
          if self.arr[2] < rhs.arr[2] { self.arr[2] } else { rhs.arr[2] },
          if self.arr[3] < rhs.arr[3] { self.arr[3] } else { rhs.arr[3] },
        ]}
      }
    }
  }

  /// Calculates the lanewise minimum of both vectors. If either lane is NaN,
  /// the other lane gets chosen. Use `fast_min` for a faster implementation
  /// that doesn't handle NaNs.
  #[inline]
  #[must_use]
  pub fn min(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse")] {
        // min_m128 seems to do self < rhs ? self : rhs. So if there's any NaN
        // involved, it chooses rhs, so we need to specifically check rhs for
        // NaN.
        rhs.is_nan().blend(self, Self { sse: min_m128(self.sse, rhs.sse) })
      } else if #[cfg(target_feature="simd128")] {
        // WASM has two min intrinsics:
        // - min: This propagates NaN, that's the opposite of what we need.
        // - pmin: This is defined as rhs < self ? rhs : self, which basically
        //   chooses self if either is NaN.
        //
        // pmin is what we want, but we need to specifically check self for NaN.
        Self {
          simd: v128_bitselect(
            rhs.simd,
            f32x4_pmin(self.simd, rhs.simd),
            f32x4_ne(self.simd, self.simd), // NaN check
          )
        }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vminnmq_f32(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].min(rhs.arr[0]),
          self.arr[1].min(rhs.arr[1]),
          self.arr[2].min(rhs.arr[2]),
          self.arr[3].min(rhs.arr[3]),
        ]}
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn is_nan(self) -> Self {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: cmp_unord_mask_m128(self.sse, self.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_ne(self.simd, self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f32_u32(vmvnq_u32(vceqq_f32(self.neon, self.neon))) }}
      } else {
        Self { arr: [
          if self.arr[0].is_nan() { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[1].is_nan() { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[2].is_nan() { f32::from_bits(u32::MAX) } else { 0.0 },
          if self.arr[3].is_nan() { f32::from_bits(u32::MAX) } else { 0.0 },
        ]}
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn is_finite(self) -> Self {
    let shifted_exp_mask = u32x4::from(0xFF000000);
    let u: u32x4 = cast(self);
    let shift_u = u << 1_u64;
    let out = !(shift_u & shifted_exp_mask).cmp_eq(shifted_exp_mask);
    cast(out)
  }
  #[inline]
  #[must_use]
  pub fn is_inf(self) -> Self {
    let shifted_inf = u32x4::from(0xFF000000);
    let u: u32x4 = cast(self);
    let shift_u = u << 1_u64;
    let out = (shift_u).cmp_eq(shifted_inf);
    cast(out)
  }

  #[inline]
  #[must_use]
  pub fn round(self) -> Self {
    pick! {
      if #[cfg(target_feature="sse4.1")] {
        Self { sse: round_m128::<{round_op!(Nearest)}>(self.sse) }
      } else if #[cfg(target_feature="sse2")] {
        let mi: m128i = convert_to_i32_m128i_from_m128(self.sse);
        let f: f32x4 = f32x4 { sse: convert_to_m128_from_i32_m128i(mi) };
        let i: i32x4 = cast(mi);
        let mask: f32x4 = cast(i.cmp_eq(i32x4::from(0x80000000_u32 as i32)));
        mask.blend(self, f)
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_nearest(self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vrndnq_f32(self.neon) }}
      } else {
        // Note(Lokathor): This software fallback is probably very slow compared
        // to having a hardware option available, even just the sse2 version is
        // better than this. Oh well.
        let to_int = f32x4::from(1.0 / f32::EPSILON);
        let u: u32x4 = cast(self);
        let e: i32x4 = cast((u >> 23) & u32x4::from(0xff));
        let mut y: f32x4;

        let no_op_magic = i32x4::from(0x7f + 23);
        let no_op_mask: f32x4 = cast(e.cmp_gt(no_op_magic) | e.cmp_eq(no_op_magic));
        let no_op_val: f32x4 = self;

        let zero_magic = i32x4::from(0x7f - 1);
        let zero_mask: f32x4 = cast(e.cmp_lt(zero_magic));
        let zero_val: f32x4 = self * f32x4::from(0.0);

        let neg_bit: f32x4 = cast(cast::<u32x4, i32x4>(u).cmp_lt(i32x4::default()));
        let x: f32x4 = neg_bit.blend(-self, self);
        y = x + to_int - to_int - x;
        y = y.cmp_gt(f32x4::from(0.5)).blend(
          y + x - f32x4::from(-1.0),
          y.cmp_lt(f32x4::from(-0.5)).blend(y + x + f32x4::from(1.0), y + x),
        );
        y = neg_bit.blend(-y, y);

        no_op_mask.blend(no_op_val, zero_mask.blend(zero_val, y))
      }
    }
  }

  /// Rounds each lane into an integer. This is a faster implementation than
  /// `round_int`, but it doesn't handle out of range values or NaNs. For those
  /// values you get implementation defined behavior.
  #[inline]
  #[must_use]
  pub fn fast_round_int(self) -> i32x4 {
    pick! {
      if #[cfg(target_feature="sse2")] {
        cast(convert_to_i32_m128i_from_m128(self.sse))
      } else {
        self.round_int()
      }
    }
  }

  /// Rounds each lane into an integer. This saturates out of range values and
  /// turns NaNs into 0. Use `fast_round_int` for a faster implementation that
  /// doesn't handle out of range values or NaNs.
  #[inline]
  #[must_use]
  pub fn round_int(self) -> i32x4 {
    pick! {
      if #[cfg(target_feature="sse2")] {
        // Based on: https://github.com/v8/v8/blob/210987a552a2bf2a854b0baa9588a5959ff3979d/src/codegen/shared-ia32-x64/macro-assembler-shared-ia32-x64.h#L489-L504
        let non_nan_mask = self.cmp_eq(self);
        let non_nan = self & non_nan_mask;
        let flip_to_max: i32x4 = cast(self.cmp_ge(Self::splat(2147483648.0)));
        let cast: i32x4 = cast(convert_to_i32_m128i_from_m128(non_nan.sse));
        flip_to_max ^ cast
      } else if #[cfg(target_feature="simd128")] {
        cast(Self { simd: i32x4_trunc_sat_f32x4(f32x4_nearest(self.simd)) })
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        cast(unsafe {Self { neon: vreinterpretq_f32_s32(vcvtnq_s32_f32(self.neon)) }})
      } else {
        let rounded: [f32; 4] = cast(self.round());
        cast([
          rounded[0] as i32,
          rounded[1] as i32,
          rounded[2] as i32,
          rounded[3] as i32,
        ])
      }
    }
  }

  /// Truncates each lane into an integer. This is a faster implementation than
  /// `trunc_int`, but it doesn't handle out of range values or NaNs. For those
  /// values you get implementation defined behavior.
  #[inline]
  #[must_use]
  pub fn fast_trunc_int(self) -> i32x4 {
    pick! {
      if #[cfg(target_feature="sse2")] {
        cast(truncate_m128_to_m128i(self.sse))
      } else {
        self.trunc_int()
      }
    }
  }

  /// Truncates each lane into an integer. This saturates out of range values
  /// and turns NaNs into 0. Use `fast_trunc_int` for a faster implementation
  /// that doesn't handle out of range values or NaNs.
  #[inline]
  #[must_use]
  pub fn trunc_int(self) -> i32x4 {
    pick! {
      if #[cfg(target_feature="sse2")] {
        // Based on: https://github.com/v8/v8/blob/210987a552a2bf2a854b0baa9588a5959ff3979d/src/codegen/shared-ia32-x64/macro-assembler-shared-ia32-x64.h#L489-L504
        let non_nan_mask = self.cmp_eq(self);
        let non_nan = self & non_nan_mask;
        let flip_to_max: i32x4 = cast(self.cmp_ge(Self::splat(2147483648.0)));
        let cast: i32x4 = cast(truncate_m128_to_m128i(non_nan.sse));
        flip_to_max ^ cast
      } else if #[cfg(target_feature="simd128")] {
        cast(Self { simd: i32x4_trunc_sat_f32x4(self.simd) })
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        cast(unsafe {Self { neon: vreinterpretq_f32_s32(vcvtq_s32_f32(self.neon)) }})
      } else {
        let n: [f32;4] = cast(self);
        cast([
          n[0] as i32,
          n[1] as i32,
          n[2] as i32,
          n[3] as i32,
        ])
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn mul_add(self, m: Self, a: Self) -> Self {
    pick! {
      if #[cfg(all(target_feature="sse2",target_feature="fma"))] {
        Self { sse: fused_mul_add_m128(self.sse, m.sse, a.sse) }
      } else {
        (self * m) + a
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn mul_sub(self, m: Self, s: Self) -> Self {
    pick! {
      if #[cfg(all(target_feature="sse2",target_feature="fma"))] {
        Self { sse: fused_mul_sub_m128(self.sse, m.sse, s.sse) }
      } else {
        (self * m) - s
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn mul_neg_add(self, m: Self, a: Self) -> Self {
    pick! {
      if #[cfg(all(target_feature="sse2",target_feature="fma"))] {
        Self { sse: fused_mul_neg_add_m128(self.sse, m.sse, a.sse) }
      } else {
        a - (self * m)
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn mul_neg_sub(self, m: Self, a: Self) -> Self {
    pick! {
      if #[cfg(all(target_feature="sse2",target_feature="fma"))] {
        Self { sse: fused_mul_neg_sub_m128(self.sse, m.sse, a.sse) }
      } else {
        -(self * m) - a
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn flip_signs(self, signs: Self) -> Self {
    self ^ (signs & Self::from(-0.0))
  }

  #[inline]
  #[must_use]
  pub fn copysign(self, sign: Self) -> Self {
    let magnitude_mask = Self::from(f32::from_bits(u32::MAX >> 1));
    (self & magnitude_mask) | (sign & Self::from(-0.0))
  }

  #[inline]
  pub fn asin_acos(self) -> (Self, Self) {
    // Based on the Agner Fog "vector class library":
    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h
    const_f32_as_f32x4!(P4asinf, 4.2163199048E-2);
    const_f32_as_f32x4!(P3asinf, 2.4181311049E-2);
    const_f32_as_f32x4!(P2asinf, 4.5470025998E-2);
    const_f32_as_f32x4!(P1asinf, 7.4953002686E-2);
    const_f32_as_f32x4!(P0asinf, 1.6666752422E-1);

    let xa = self.abs();
    let big = xa.cmp_ge(f32x4::splat(0.5));

    let x1 = f32x4::splat(0.5) * (f32x4::ONE - xa);
    let x2 = xa * xa;
    let x3 = big.blend(x1, x2);

    let xb = x1.sqrt();

    let x4 = big.blend(xb, xa);

    let z = polynomial_4!(x3, P0asinf, P1asinf, P2asinf, P3asinf, P4asinf);
    let z = z.mul_add(x3 * x4, x4);

    let z1 = z + z;

    // acos
    let z3 = self.cmp_lt(f32x4::ZERO).blend(f32x4::PI - z1, z1);
    let z4 = f32x4::FRAC_PI_2 - z.flip_signs(self);
    let acos = big.blend(z3, z4);

    // asin
    let z3 = f32x4::FRAC_PI_2 - z1;
    let asin = big.blend(z3, z);
    let asin = asin.flip_signs(self);

    (asin, acos)
  }

  #[inline]
  pub fn asin(self) -> Self {
    // Based on the Agner Fog "vector class library":
    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h
    const_f32_as_f32x4!(P4asinf, 4.2163199048E-2);
    const_f32_as_f32x4!(P3asinf, 2.4181311049E-2);
    const_f32_as_f32x4!(P2asinf, 4.5470025998E-2);
    const_f32_as_f32x4!(P1asinf, 7.4953002686E-2);
    const_f32_as_f32x4!(P0asinf, 1.6666752422E-1);

    let xa = self.abs();
    let big = xa.cmp_ge(f32x4::splat(0.5));

    let x1 = f32x4::splat(0.5) * (f32x4::ONE - xa);
    let x2 = xa * xa;
    let x3 = big.blend(x1, x2);

    let xb = x1.sqrt();

    let x4 = big.blend(xb, xa);

    let z = polynomial_4!(x3, P0asinf, P1asinf, P2asinf, P3asinf, P4asinf);
    let z = z.mul_add(x3 * x4, x4);

    let z1 = z + z;

    // asin
    let z3 = f32x4::FRAC_PI_2 - z1;
    let asin = big.blend(z3, z);
    let asin = asin.flip_signs(self);

    asin
  }

  #[inline]
  #[must_use]
  pub fn acos(self) -> Self {
    // Based on the Agner Fog "vector class library":
    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h
    const_f32_as_f32x4!(P4asinf, 4.2163199048E-2);
    const_f32_as_f32x4!(P3asinf, 2.4181311049E-2);
    const_f32_as_f32x4!(P2asinf, 4.5470025998E-2);
    const_f32_as_f32x4!(P1asinf, 7.4953002686E-2);
    const_f32_as_f32x4!(P0asinf, 1.6666752422E-1);

    let xa = self.abs();
    let big = xa.cmp_ge(f32x4::splat(0.5));

    let x1 = f32x4::splat(0.5) * (f32x4::ONE - xa);
    let x2 = xa * xa;
    let x3 = big.blend(x1, x2);

    let xb = x1.sqrt();

    let x4 = big.blend(xb, xa);

    let z = polynomial_4!(x3, P0asinf, P1asinf, P2asinf, P3asinf, P4asinf);
    let z = z.mul_add(x3 * x4, x4);

    let z1 = z + z;

    // acos
    let z3 = self.cmp_lt(f32x4::ZERO).blend(f32x4::PI - z1, z1);
    let z4 = f32x4::FRAC_PI_2 - z.flip_signs(self);
    let acos = big.blend(z3, z4);

    acos
  }

  #[inline]
  pub fn atan(self) -> Self {
    // Based on the Agner Fog "vector class library":
    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h
    const_f32_as_f32x4!(P3atanf, 8.05374449538E-2);
    const_f32_as_f32x4!(P2atanf, -1.38776856032E-1);
    const_f32_as_f32x4!(P1atanf, 1.99777106478E-1);
    const_f32_as_f32x4!(P0atanf, -3.33329491539E-1);

    let t = self.abs();

    // small:  z = t / 1.0;
    // medium: z = (t-1.0) / (t+1.0);
    // big:    z = -1.0 / t;
    let notsmal = t.cmp_ge(Self::SQRT_2 - Self::ONE);
    let notbig = t.cmp_le(Self::SQRT_2 + Self::ONE);

    let mut s = notbig.blend(Self::FRAC_PI_4, Self::FRAC_PI_2);
    s = notsmal & s;

    let mut a = notbig & t;
    a = notsmal.blend(a - Self::ONE, a);
    let mut b = notbig & Self::ONE;
    b = notsmal.blend(b + t, b);
    let z = a / b;

    let zz = z * z;

    // Taylor expansion
    let mut re = polynomial_3!(zz, P0atanf, P1atanf, P2atanf, P3atanf);
    re = re.mul_add(zz * z, z) + s;

    // get sign bit
    re = (self.sign_bit()).blend(-re, re);

    re
  }

  #[inline]
  pub fn atan2(self, x: Self) -> Self {
    // Based on the Agner Fog "vector class library":
    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h
    const_f32_as_f32x4!(P3atanf, 8.05374449538E-2);
    const_f32_as_f32x4!(P2atanf, -1.38776856032E-1);
    const_f32_as_f32x4!(P1atanf, 1.99777106478E-1);
    const_f32_as_f32x4!(P0atanf, -3.33329491539E-1);

    let y = self;

    // move in first octant
    let x1 = x.abs();
    let y1 = y.abs();
    let swapxy = y1.cmp_gt(x1);
    // swap x and y if y1 > x1
    let mut x2 = swapxy.blend(y1, x1);
    let mut y2 = swapxy.blend(x1, y1);

    // check for special case: x and y are both +/- INF
    let both_infinite = x.is_inf() & y.is_inf();
    if both_infinite.any() {
      let minus_one = -Self::ONE;
      x2 = both_infinite.blend(x2 & minus_one, x2);
      y2 = both_infinite.blend(y2 & minus_one, y2);
    }

    // x = y = 0 will produce NAN. No problem, fixed below
    let t = y2 / x2;

    // small:  z = t / 1.0;
    // medium: z = (t-1.0) / (t+1.0);
    let notsmal = t.cmp_ge(Self::SQRT_2 - Self::ONE);

    let a = notsmal.blend(t - Self::ONE, t);
    let b = notsmal.blend(t + Self::ONE, Self::ONE);
    let s = notsmal & Self::FRAC_PI_4;
    let z = a / b;

    let zz = z * z;

    // Taylor expansion
    let mut re = polynomial_3!(zz, P0atanf, P1atanf, P2atanf, P3atanf);
    re = re.mul_add(zz * z, z) + s;

    // move back in place
    re = swapxy.blend(Self::FRAC_PI_2 - re, re);
    re = ((x | y).cmp_eq(Self::ZERO)).blend(Self::ZERO, re);
    re = (x.sign_bit()).blend(Self::PI - re, re);

    // get sign bit
    re = (y.sign_bit()).blend(-re, re);

    re
  }

  #[inline]
  #[must_use]
  pub fn sin_cos(self) -> (Self, Self) {
    // Based on the Agner Fog "vector class library":
    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h

    const_f32_as_f32x4!(DP1F, 0.78515625_f32 * 2.0);
    const_f32_as_f32x4!(DP2F, 2.4187564849853515625E-4_f32 * 2.0);
    const_f32_as_f32x4!(DP3F, 3.77489497744594108E-8_f32 * 2.0);

    const_f32_as_f32x4!(P0sinf, -1.6666654611E-1);
    const_f32_as_f32x4!(P1sinf, 8.3321608736E-3);
    const_f32_as_f32x4!(P2sinf, -1.9515295891E-4);

    const_f32_as_f32x4!(P0cosf, 4.166664568298827E-2);
    const_f32_as_f32x4!(P1cosf, -1.388731625493765E-3);
    const_f32_as_f32x4!(P2cosf, 2.443315711809948E-5);

    const_f32_as_f32x4!(TWO_OVER_PI, 2.0 / core::f32::consts::PI);

    let xa = self.abs();

    // Find quadrant
    let y = (xa * TWO_OVER_PI).round();
    let q: i32x4 = y.round_int();

    let x = y.mul_neg_add(DP3F, y.mul_neg_add(DP2F, y.mul_neg_add(DP1F, xa)));

    let x2 = x * x;
    let mut s = polynomial_2!(x2, P0sinf, P1sinf, P2sinf) * (x * x2) + x;
    let mut c = polynomial_2!(x2, P0cosf, P1cosf, P2cosf) * (x2 * x2)
      + f32x4::from(0.5).mul_neg_add(x2, f32x4::from(1.0));

    let swap = !(q & i32x4::from(1)).cmp_eq(i32x4::from(0));

    let mut overflow: f32x4 = cast(q.cmp_gt(i32x4::from(0x2000000)));
    overflow &= xa.is_finite();
    s = overflow.blend(f32x4::from(0.0), s);
    c = overflow.blend(f32x4::from(1.0), c);

    // calc sin
    let mut sin1 = cast::<_, f32x4>(swap).blend(c, s);
    let sign_sin: i32x4 = (q << 30) ^ cast::<_, i32x4>(self);
    sin1 = sin1.flip_signs(cast(sign_sin));

    // calc cos
    let mut cos1 = cast::<_, f32x4>(swap).blend(s, c);
    let sign_cos: i32x4 = ((q + i32x4::from(1)) & i32x4::from(2)) << 30;
    cos1 ^= cast::<_, f32x4>(sign_cos);

    (sin1, cos1)
  }

  #[inline]
  #[must_use]
  pub fn sin(self) -> Self {
    let (s, _) = self.sin_cos();
    s
  }
  #[inline]
  #[must_use]
  pub fn cos(self) -> Self {
    let (_, c) = self.sin_cos();
    c
  }
  #[inline]
  #[must_use]
  pub fn tan(self) -> Self {
    let (s, c) = self.sin_cos();
    s / c
  }
  #[inline]
  #[must_use]
  pub fn to_degrees(self) -> Self {
    const_f32_as_f32x4!(RAD_TO_DEG_RATIO, 180.0_f32 / core::f32::consts::PI);
    self * RAD_TO_DEG_RATIO
  }
  #[inline]
  #[must_use]
  pub fn to_radians(self) -> Self {
    const_f32_as_f32x4!(DEG_TO_RAD_RATIO, core::f32::consts::PI / 180.0_f32);
    self * DEG_TO_RAD_RATIO
  }
  #[inline]
  #[must_use]
  pub fn recip(self) -> Self {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: reciprocal_m128(self.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_div(f32x4_splat(1.0), self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vdivq_f32(vdupq_n_f32(1.0), self.neon) }}
      } else {
        Self { arr: [
          1.0 / self.arr[0],
          1.0 / self.arr[1],
          1.0 / self.arr[2],
          1.0 / self.arr[3],
        ]}
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn recip_sqrt(self) -> Self {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: reciprocal_sqrt_m128(self.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_div(f32x4_splat(1.0), f32x4_sqrt(self.simd)) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vdivq_f32(vdupq_n_f32(1.0), vsqrtq_f32(self.neon)) }}
      } else if #[cfg(feature="std")] {
        Self { arr: [
          1.0 / self.arr[0].sqrt(),
          1.0 / self.arr[1].sqrt(),
          1.0 / self.arr[2].sqrt(),
          1.0 / self.arr[3].sqrt(),
        ]}
      } else {
        Self { arr: [
          1.0 / software_sqrt(self.arr[0] as f64) as f32,
          1.0 / software_sqrt(self.arr[1] as f64) as f32,
          1.0 / software_sqrt(self.arr[2] as f64) as f32,
          1.0 / software_sqrt(self.arr[3] as f64) as f32,
        ]}
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn sqrt(self) -> Self {
    pick! {
      if #[cfg(target_feature="sse")] {
        Self { sse: sqrt_m128(self.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_sqrt(self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vsqrtq_f32(self.neon) }}
      } else if #[cfg(feature="std")] {
        Self { arr: [
          self.arr[0].sqrt(),
          self.arr[1].sqrt(),
          self.arr[2].sqrt(),
          self.arr[3].sqrt(),
        ]}
      } else {
        Self { arr: [
          software_sqrt(self.arr[0] as f64) as f32,
          software_sqrt(self.arr[1] as f64) as f32,
          software_sqrt(self.arr[2] as f64) as f32,
          software_sqrt(self.arr[3] as f64) as f32,
        ]}
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn move_mask(self) -> i32 {
    pick! {
      if #[cfg(target_feature="sse")] {
        move_mask_m128(self.sse)
      } else if #[cfg(target_feature="simd128")] {
        u32x4_bitmask(self.simd) as i32
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe
        {
          // set all to 1 if top bit is set, else 0
          let masked = vcltq_s32( vreinterpretq_s32_f32(self.neon), vdupq_n_s32(0));

          // select the right bit out of each lane
          let selectbit : uint32x4_t = core::mem::transmute([1u32, 2, 4, 8]);
          let r = vandq_u32(masked, selectbit);

          // horizontally add the 16-bit lanes
          vaddvq_u32(r) as i32
        }
      } else {
        (((self.arr[0].to_bits() as i32) < 0) as i32) << 0 |
        (((self.arr[1].to_bits() as i32) < 0) as i32) << 1 |
        (((self.arr[2].to_bits() as i32) < 0) as i32) << 2 |
        (((self.arr[3].to_bits() as i32) < 0) as i32) << 3
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn any(self) -> bool {
    pick! {
      if #[cfg(target_feature="simd128")] {
        v128_any_true(self.simd)
      } else {
        self.move_mask() != 0
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn all(self) -> bool {
    pick! {
      if #[cfg(target_feature="simd128")] {
        u32x4_all_true(self.simd)
      } else {
        // four lanes
        self.move_mask() == 0b1111
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn none(self) -> bool {
    !self.any()
  }

  #[inline]
  fn vm_pow2n(self) -> Self {
    const_f32_as_f32x4!(pow2_23, 8388608.0);
    const_f32_as_f32x4!(bias, 127.0);
    let a = self + (bias + pow2_23);
    let c = cast::<_, i32x4>(a) << 23;
    cast::<_, f32x4>(c)
  }

  /// Calculate the exponent of a packed `f32x4`
  #[inline]
  #[must_use]
  pub fn exp(self) -> Self {
    const_f32_as_f32x4!(P0, 1.0 / 2.0);
    const_f32_as_f32x4!(P1, 1.0 / 6.0);
    const_f32_as_f32x4!(P2, 1. / 24.);
    const_f32_as_f32x4!(P3, 1. / 120.);
    const_f32_as_f32x4!(P4, 1. / 720.);
    const_f32_as_f32x4!(P5, 1. / 5040.);
    const_f32_as_f32x4!(LN2D_HI, 0.693359375);
    const_f32_as_f32x4!(LN2D_LO, -2.12194440e-4);
    let max_x = f32x4::from(87.3);
    let r = (self * Self::LOG2_E).round();
    let x = r.mul_neg_add(LN2D_HI, self);
    let x = r.mul_neg_add(LN2D_LO, x);
    let z = polynomial_5!(x, P0, P1, P2, P3, P4, P5);
    let x2 = x * x;
    let z = z.mul_add(x2, x);
    let n2 = Self::vm_pow2n(r);
    let z = (z + Self::ONE) * n2;
    // check for overflow
    let in_range = self.abs().cmp_lt(max_x);
    let in_range = in_range & self.is_finite();
    in_range.blend(z, Self::ZERO)
  }

  #[inline]
  fn exponent(self) -> f32x4 {
    const_f32_as_f32x4!(pow2_23, 8388608.0);
    const_f32_as_f32x4!(bias, 127.0);
    let a = cast::<_, u32x4>(self);
    let b = a >> 23;
    let c = b | cast::<_, u32x4>(pow2_23);
    let d = cast::<_, f32x4>(c);
    let e = d - (pow2_23 + bias);
    e
  }

  #[inline]
  fn fraction_2(self) -> Self {
    let t1 = cast::<_, u32x4>(self);
    let t2 = cast::<_, u32x4>(
      (t1 & u32x4::from(0x007FFFFF)) | u32x4::from(0x3F000000),
    );
    cast::<_, f32x4>(t2)
  }
  #[inline]
  fn is_zero_or_subnormal(self) -> Self {
    let t = cast::<_, i32x4>(self);
    let t = t & i32x4::splat(0x7F800000);
    i32x4::round_float(t.cmp_eq(i32x4::splat(0)))
  }
  #[inline]
  fn infinity() -> Self {
    cast::<_, f32x4>(i32x4::splat(0x7F800000))
  }
  #[inline]
  fn nan_log() -> Self {
    cast::<_, f32x4>(i32x4::splat(0x7FC00000 | 0x101 & 0x003FFFFF))
  }
  #[inline]
  fn nan_pow() -> Self {
    cast::<_, f32x4>(i32x4::splat(0x7FC00000 | 0x101 & 0x003FFFFF))
  }
  #[inline]
  pub fn sign_bit(self) -> Self {
    let t1 = cast::<_, i32x4>(self);
    let t2 = t1 >> 31;
    !cast::<_, f32x4>(t2).cmp_eq(f32x4::ZERO)
  }

  /// horizontal add of all the elements of the vector
  #[inline]
  #[must_use]
  pub fn reduce_add(self) -> f32 {
    let arr: [f32; 4] = cast(self);
    arr.iter().sum()
  }

  /// Natural log (ln(x))
  #[inline]
  #[must_use]
  pub fn ln(self) -> Self {
    const_f32_as_f32x4!(HALF, 0.5);
    const_f32_as_f32x4!(P0, 3.3333331174E-1);
    const_f32_as_f32x4!(P1, -2.4999993993E-1);
    const_f32_as_f32x4!(P2, 2.0000714765E-1);
    const_f32_as_f32x4!(P3, -1.6668057665E-1);
    const_f32_as_f32x4!(P4, 1.4249322787E-1);
    const_f32_as_f32x4!(P5, -1.2420140846E-1);
    const_f32_as_f32x4!(P6, 1.1676998740E-1);
    const_f32_as_f32x4!(P7, -1.1514610310E-1);
    const_f32_as_f32x4!(P8, 7.0376836292E-2);
    const_f32_as_f32x4!(LN2F_HI, 0.693359375);
    const_f32_as_f32x4!(LN2F_LO, -2.12194440e-4);
    const_f32_as_f32x4!(VM_SMALLEST_NORMAL, 1.17549435E-38);

    let x1 = self;
    let x = Self::fraction_2(x1);
    let e = Self::exponent(x1);
    let mask = x.cmp_gt(Self::SQRT_2 * HALF);
    let x = (!mask).blend(x + x, x);
    let fe = mask.blend(e + Self::ONE, e);
    let x = x - Self::ONE;
    let res = polynomial_8!(x, P0, P1, P2, P3, P4, P5, P6, P7, P8);
    let x2 = x * x;
    let res = x2 * x * res;
    let res = fe.mul_add(LN2F_LO, res);
    let res = res + x2.mul_neg_add(HALF, x);
    let res = fe.mul_add(LN2F_HI, res);
    let overflow = !self.is_finite();
    let underflow = x1.cmp_lt(VM_SMALLEST_NORMAL);
    let mask = overflow | underflow;
    if !mask.any() {
      res
    } else {
      let is_zero = self.is_zero_or_subnormal();
      let res = underflow.blend(Self::nan_log(), res);
      let res = is_zero.blend(Self::infinity(), res);
      let res = overflow.blend(self, res);
      res
    }
  }

  #[inline]
  #[must_use]
  pub fn log2(self) -> Self {
    Self::ln(self) * Self::LOG2_E
  }
  #[inline]
  #[must_use]
  pub fn log10(self) -> Self {
    Self::ln(self) * Self::LOG10_E
  }

  #[inline]
  #[must_use]
  pub fn pow_f32x4(self, y: f32x4) -> Self {
    const_f32_as_f32x4!(ln2f_hi, 0.693359375);
    const_f32_as_f32x4!(ln2f_lo, -2.12194440e-4);
    const_f32_as_f32x4!(P0logf, 3.3333331174E-1);
    const_f32_as_f32x4!(P1logf, -2.4999993993E-1);
    const_f32_as_f32x4!(P2logf, 2.0000714765E-1);
    const_f32_as_f32x4!(P3logf, -1.6668057665E-1);
    const_f32_as_f32x4!(P4logf, 1.4249322787E-1);
    const_f32_as_f32x4!(P5logf, -1.2420140846E-1);
    const_f32_as_f32x4!(P6logf, 1.1676998740E-1);
    const_f32_as_f32x4!(P7logf, -1.1514610310E-1);
    const_f32_as_f32x4!(P8logf, 7.0376836292E-2);

    const_f32_as_f32x4!(p2expf, 1.0 / 2.0); // coefficients for Taylor expansion of exp
    const_f32_as_f32x4!(p3expf, 1.0 / 6.0);
    const_f32_as_f32x4!(p4expf, 1.0 / 24.0);
    const_f32_as_f32x4!(p5expf, 1.0 / 120.0);
    const_f32_as_f32x4!(p6expf, 1.0 / 720.0);
    const_f32_as_f32x4!(p7expf, 1.0 / 5040.0);

    let x1 = self.abs();
    let x = x1.fraction_2();

    let mask = x.cmp_gt(f32x4::SQRT_2 * f32x4::HALF);
    let x = (!mask).blend(x + x, x);

    let x = x - f32x4::ONE;
    let x2 = x * x;
    let lg1 = polynomial_8!(
      x, P0logf, P1logf, P2logf, P3logf, P4logf, P5logf, P6logf, P7logf, P8logf
    );
    let lg1 = lg1 * x2 * x;

    let ef = x1.exponent();
    let ef = mask.blend(ef + f32x4::ONE, ef);

    let e1 = (ef * y).round();
    let yr = ef.mul_sub(y, e1);

    let lg = f32x4::HALF.mul_neg_add(x2, x) + lg1;
    let x2_err = (f32x4::HALF * x).mul_sub(x, f32x4::HALF * x2);
    let lg_err = f32x4::HALF.mul_add(x2, lg - x) - lg1;

    let e2 = (lg * y * f32x4::LOG2_E).round();
    let v = lg.mul_sub(y, e2 * ln2f_hi);
    let v = e2.mul_neg_add(ln2f_lo, v);
    let v = v - (lg_err + x2_err).mul_sub(y, yr * f32x4::LN_2);

    let x = v;
    let e3 = (x * f32x4::LOG2_E).round();
    let x = e3.mul_neg_add(f32x4::LN_2, x);
    let x2 = x * x;
    let z = x2.mul_add(
      polynomial_5!(x, p2expf, p3expf, p4expf, p5expf, p6expf, p7expf),
      x + f32x4::ONE,
    );

    let ee = e1 + e2 + e3;
    let ei = cast::<_, i32x4>(ee.round_int());
    let ej = cast::<_, i32x4>(ei + (cast::<_, i32x4>(z) >> 23));

    let overflow = cast::<_, f32x4>(ej.cmp_gt(i32x4::splat(0x0FF)))
      | (ee.cmp_gt(f32x4::splat(300.0)));
    let underflow = cast::<_, f32x4>(ej.cmp_lt(i32x4::splat(0x000)))
      | (ee.cmp_lt(f32x4::splat(-300.0)));

    // Add exponent by integer addition
    let z = cast::<_, f32x4>(cast::<_, i32x4>(z) + (ei << 23));

    // Check for overflow/underflow
    let z = if (overflow | underflow).any() {
      let z = underflow.blend(f32x4::ZERO, z);
      overflow.blend(Self::infinity(), z)
    } else {
      z
    };

    // Check for self == 0
    let x_zero = self.is_zero_or_subnormal();
    let z = x_zero.blend(
      y.cmp_lt(f32x4::ZERO).blend(
        Self::infinity(),
        y.cmp_eq(f32x4::ZERO).blend(f32x4::ONE, f32x4::ZERO),
      ),
      z,
    );

    let x_sign = self.sign_bit();
    let z = if x_sign.any() {
      // Y into an integer
      let yi = y.cmp_eq(y.round());
      // Is y odd?
      let y_odd = cast::<_, i32x4>(y.round_int() << 31).round_float();

      let z1 =
        yi.blend(z | y_odd, self.cmp_eq(Self::ZERO).blend(z, Self::nan_pow()));
      x_sign.blend(z1, z)
    } else {
      z
    };

    let x_finite = self.is_finite();
    let y_finite = y.is_finite();
    let e_finite = ee.is_finite();
    if (x_finite & y_finite & (e_finite | x_zero)).all() {
      return z;
    }

    (self.is_nan() | y.is_nan()).blend(self + y, z)
  }

  #[inline]
  pub fn powf(self, y: f32) -> Self {
    Self::pow_f32x4(self, f32x4::splat(y))
  }

  /// Transpose matrix of 4x4 `f32` matrix. Currently only accelerated on SSE.
  #[must_use]
  #[inline]
  pub fn transpose(data: [f32x4; 4]) -> [f32x4; 4] {
    pick! {
      if #[cfg(target_feature="sse")] {
        let mut e0 = data[0];
        let mut e1 = data[1];
        let mut e2 = data[2];
        let mut e3 = data[3];

        transpose_four_m128(&mut e0.sse, &mut e1.sse, &mut e2.sse, &mut e3.sse);

        [e0, e1, e2, e3]
      } else {
        #[inline(always)]
        fn transpose_column(data: &[f32x4; 4], index: usize) -> f32x4 {
          f32x4::new([
            data[0].as_array_ref()[index],
            data[1].as_array_ref()[index],
            data[2].as_array_ref()[index],
            data[3].as_array_ref()[index],
          ])
        }

        [
          transpose_column(&data, 0),
          transpose_column(&data, 1),
          transpose_column(&data, 2),
          transpose_column(&data, 3),
        ]
      }
    }
  }

  #[inline]
  pub fn to_array(self) -> [f32; 4] {
    cast(self)
  }

  #[inline]
  pub fn as_array_ref(&self) -> &[f32; 4] {
    cast_ref(self)
  }

  #[inline]
  pub fn as_array_mut(&mut self) -> &mut [f32; 4] {
    cast_mut(self)
  }

  #[inline]
  pub fn from_i32x4(v: i32x4) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: convert_to_m128_from_i32_m128i(v.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f32x4_convert_i32x4(v.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
        Self { neon: unsafe { vcvtq_f32_s32(v.neon) }}
      } else {
        Self { arr: [
            v.as_array_ref()[0] as f32,
            v.as_array_ref()[1] as f32,
            v.as_array_ref()[2] as f32,
            v.as_array_ref()[3] as f32,
          ] }
      }
    }
  }
}
