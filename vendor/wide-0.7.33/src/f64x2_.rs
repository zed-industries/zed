use super::*;

pick! {
  if #[cfg(target_feature="sse2")] {
    #[derive(Default, Clone, Copy, PartialEq)]
    #[repr(C, align(16))]
    pub struct f64x2 { pub(crate) sse: m128d }
  } else if #[cfg(target_feature="simd128")] {
    use core::arch::wasm32::*;

    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct f64x2 { pub(crate) simd: v128 }

    impl Default for f64x2 {
      fn default() -> Self {
        Self::splat(0.0)
      }
    }

    impl PartialEq for f64x2 {
      fn eq(&self, other: &Self) -> bool {
        u64x2_all_true(f64x2_eq(self.simd, other.simd))
      }
    }
  } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
    use core::arch::aarch64::*;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct f64x2 { pub(crate) neon: float64x2_t }

    impl Default for f64x2 {
      #[inline]
      #[must_use]
      fn default() -> Self {
        unsafe { Self { neon: vdupq_n_f64(0.0)} }
      }
    }

    impl PartialEq for f64x2 {
      #[inline]
      #[must_use]
      fn eq(&self, other: &Self) -> bool {
        unsafe
        { let e = vceqq_f64(self.neon, other.neon);
          vgetq_lane_u64(e,0) == u64::MAX && vgetq_lane_u64(e,1) == u64::MAX
        }
      }

    }
  } else {
    #[derive(Default, Clone, Copy, PartialEq)]
    #[repr(C, align(16))]
    pub struct f64x2 { pub(crate) arr: [f64;2] }
  }
}

macro_rules! const_f64_as_f64x2 {
  ($i:ident, $f:expr) => {
    #[allow(non_upper_case_globals)]
    pub const $i: f64x2 = f64x2::new([$f; 2]);
  };
}

impl f64x2 {
  const_f64_as_f64x2!(ONE, 1.0);
  const_f64_as_f64x2!(ZERO, 0.0);
  const_f64_as_f64x2!(HALF, 0.5);
  const_f64_as_f64x2!(E, core::f64::consts::E);
  const_f64_as_f64x2!(FRAC_1_PI, core::f64::consts::FRAC_1_PI);
  const_f64_as_f64x2!(FRAC_2_PI, core::f64::consts::FRAC_2_PI);
  const_f64_as_f64x2!(FRAC_2_SQRT_PI, core::f64::consts::FRAC_2_SQRT_PI);
  const_f64_as_f64x2!(FRAC_1_SQRT_2, core::f64::consts::FRAC_1_SQRT_2);
  const_f64_as_f64x2!(FRAC_PI_2, core::f64::consts::FRAC_PI_2);
  const_f64_as_f64x2!(FRAC_PI_3, core::f64::consts::FRAC_PI_3);
  const_f64_as_f64x2!(FRAC_PI_4, core::f64::consts::FRAC_PI_4);
  const_f64_as_f64x2!(FRAC_PI_6, core::f64::consts::FRAC_PI_6);
  const_f64_as_f64x2!(FRAC_PI_8, core::f64::consts::FRAC_PI_8);
  const_f64_as_f64x2!(LN_2, core::f64::consts::LN_2);
  const_f64_as_f64x2!(LN_10, core::f64::consts::LN_10);
  const_f64_as_f64x2!(LOG2_E, core::f64::consts::LOG2_E);
  const_f64_as_f64x2!(LOG10_E, core::f64::consts::LOG10_E);
  const_f64_as_f64x2!(LOG10_2, core::f64::consts::LOG10_2);
  const_f64_as_f64x2!(LOG2_10, core::f64::consts::LOG2_10);
  const_f64_as_f64x2!(PI, core::f64::consts::PI);
  const_f64_as_f64x2!(SQRT_2, core::f64::consts::SQRT_2);
  const_f64_as_f64x2!(TAU, core::f64::consts::TAU);
}

unsafe impl Zeroable for f64x2 {}
unsafe impl Pod for f64x2 {}

impl Add for f64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: add_m128d(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f64x2_add(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vaddq_f64(self.neon, rhs.neon) } }
      } else {
        Self { arr: [
          self.arr[0] + rhs.arr[0],
          self.arr[1] + rhs.arr[1],
        ]}
      }
    }
  }
}

impl Sub for f64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: sub_m128d(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f64x2_sub(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vsubq_f64(self.neon, rhs.neon) } }
      } else {
        Self { arr: [
          self.arr[0] - rhs.arr[0],
          self.arr[1] - rhs.arr[1],
        ]}
      }
    }
  }
}

impl Mul for f64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: mul_m128d(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f64x2_mul(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vmulq_f64(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0] * rhs.arr[0],
          self.arr[1] * rhs.arr[1],
        ]}
      }
    }
  }
}

impl Div for f64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn div(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: div_m128d(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f64x2_div(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vdivq_f64(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0] / rhs.arr[0],
          self.arr[1] / rhs.arr[1],
        ]}
      }
    }
  }
}

impl Add<f64> for f64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: f64) -> Self::Output {
    self.add(Self::splat(rhs))
  }
}

impl Sub<f64> for f64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: f64) -> Self::Output {
    self.sub(Self::splat(rhs))
  }
}

impl Mul<f64> for f64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: f64) -> Self::Output {
    self.mul(Self::splat(rhs))
  }
}

impl Div<f64> for f64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn div(self, rhs: f64) -> Self::Output {
    self.div(Self::splat(rhs))
  }
}

impl Add<f64x2> for f64 {
  type Output = f64x2;
  #[inline]
  #[must_use]
  fn add(self, rhs: f64x2) -> Self::Output {
    f64x2::splat(self).add(rhs)
  }
}

impl Sub<f64x2> for f64 {
  type Output = f64x2;
  #[inline]
  #[must_use]
  fn sub(self, rhs: f64x2) -> Self::Output {
    f64x2::splat(self).sub(rhs)
  }
}

impl Mul<f64x2> for f64 {
  type Output = f64x2;
  #[inline]
  #[must_use]
  fn mul(self, rhs: f64x2) -> Self::Output {
    f64x2::splat(self).mul(rhs)
  }
}

impl Div<f64x2> for f64 {
  type Output = f64x2;
  #[inline]
  #[must_use]
  fn div(self, rhs: f64x2) -> Self::Output {
    f64x2::splat(self).div(rhs)
  }
}

impl BitAnd for f64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitand(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: bitand_m128d(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: v128_and(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f64_u64(vandq_u64(vreinterpretq_u64_f64(self.neon), vreinterpretq_u64_f64(rhs.neon))) }}
      } else {
        Self { arr: [
          f64::from_bits(self.arr[0].to_bits() & rhs.arr[0].to_bits()),
          f64::from_bits(self.arr[1].to_bits() & rhs.arr[1].to_bits()),
        ]}
      }
    }
  }
}

impl BitOr for f64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitor(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: bitor_m128d(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: v128_or(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f64_u64(vorrq_u64(vreinterpretq_u64_f64(self.neon), vreinterpretq_u64_f64(rhs.neon))) }}
      } else {
        Self { arr: [
          f64::from_bits(self.arr[0].to_bits() | rhs.arr[0].to_bits()),
          f64::from_bits(self.arr[1].to_bits() | rhs.arr[1].to_bits()),
        ]}
      }
    }
  }
}

impl BitXor for f64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitxor(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: bitxor_m128d(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: v128_xor(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f64_u64(veorq_u64(vreinterpretq_u64_f64(self.neon), vreinterpretq_u64_f64(rhs.neon))) }}
      } else {
        Self { arr: [
          f64::from_bits(self.arr[0].to_bits() ^ rhs.arr[0].to_bits()),
          f64::from_bits(self.arr[1].to_bits() ^ rhs.arr[1].to_bits()),
        ]}
      }
    }
  }
}

impl CmpEq for f64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_eq(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_eq_mask_m128d(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f64x2_eq(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f64_u64(vceqq_f64(self.neon, rhs.neon)) }}
      } else {
        Self { arr: [
          if self.arr[0] == rhs.arr[0] { f64::from_bits(u64::MAX) } else { 0.0 },
          if self.arr[1] == rhs.arr[1] { f64::from_bits(u64::MAX) } else { 0.0 },
        ]}
      }
    }
  }
}

impl CmpGe for f64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_ge(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_ge_mask_m128d(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f64x2_ge(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f64_u64(vcgeq_f64(self.neon, rhs.neon)) }}
      } else {
        Self { arr: [
          if self.arr[0] >= rhs.arr[0] { f64::from_bits(u64::MAX) } else { 0.0 },
          if self.arr[1] >= rhs.arr[1] { f64::from_bits(u64::MAX) } else { 0.0 },
        ]}
      }
    }
  }
}

impl CmpGt for f64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_gt(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { sse: cmp_op_mask_m128d::<{cmp_op!(GreaterThanOrdered)}>(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_gt_mask_m128d(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f64x2_gt(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f64_u64(vcgtq_f64(self.neon, rhs.neon)) }}
      } else {
        Self { arr: [
          if self.arr[0] > rhs.arr[0] { f64::from_bits(u64::MAX) } else { 0.0 },
          if self.arr[1] > rhs.arr[1] { f64::from_bits(u64::MAX) } else { 0.0 },
        ]}
      }
    }
  }
}

impl CmpNe for f64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_ne(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_neq_mask_m128d(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f64x2_ne(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f64_u64(vceqq_f64(self.neon, rhs.neon)) }.not() }
      } else {
        Self { arr: [
          if self.arr[0] != rhs.arr[0] { f64::from_bits(u64::MAX) } else { 0.0 },
          if self.arr[1] != rhs.arr[1] { f64::from_bits(u64::MAX) } else { 0.0 },
        ]}
      }
    }
  }
}

impl CmpLe for f64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_le(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_le_mask_m128d(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f64x2_le(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f64_u64(vcleq_f64(self.neon, rhs.neon)) }}
      } else {
        Self { arr: [
          if self.arr[0] <= rhs.arr[0] { f64::from_bits(u64::MAX) } else { 0.0 },
          if self.arr[1] <= rhs.arr[1] { f64::from_bits(u64::MAX) } else { 0.0 },
        ]}
      }
    }
  }
}

impl CmpLt for f64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_lt(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_lt_mask_m128d(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f64x2_lt(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f64_u64(vcltq_f64(self.neon, rhs.neon)) }}
      } else {
        Self { arr: [
          if self.arr[0] < rhs.arr[0] { f64::from_bits(u64::MAX) } else { 0.0 },
          if self.arr[1] < rhs.arr[1] { f64::from_bits(u64::MAX) } else { 0.0 },
        ]}
      }
    }
  }
}

impl f64x2 {
  #[inline]
  #[must_use]
  pub const fn new(array: [f64; 2]) -> Self {
    unsafe { core::mem::transmute(array) }
  }
  #[inline]
  #[must_use]
  pub fn blend(self, t: Self, f: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse4.1")] {
        Self { sse: blend_varying_m128d(f.sse, t.sse, self.sse) }
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
        Self { simd: f64x2_abs(self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vabsq_f64(self.neon) }}
      } else {
        let non_sign_bits = f64x2::from(f64::from_bits(i64::MAX as u64));
        self & non_sign_bits
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn floor(self) -> Self {
    pick! {
      if #[cfg(target_feature="simd128")] {
        Self { simd: f64x2_floor(self.simd) }
      } else if #[cfg(target_feature="sse4.1")] {
        Self { sse: floor_m128d(self.sse) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vrndmq_f64(self.neon) }}
      } else if #[cfg(feature="std")] {
        let base: [f64; 2] = cast(self);
        cast(base.map(|val| val.floor()))
      } else {
        let base: [f64; 2] = cast(self);
        let rounded: [f64; 2] = cast(self.round());
        cast([
          if base[0] < rounded[0] { rounded[0] - 1.0 } else { rounded[0] },
          if base[1] < rounded[1] { rounded[1] - 1.0 } else { rounded[1] },
        ])
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn ceil(self) -> Self {
    pick! {
      if #[cfg(target_feature="simd128")] {
        Self { simd: f64x2_ceil(self.simd) }
      } else if #[cfg(target_feature="sse4.1")] {
        Self { sse: ceil_m128d(self.sse) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vrndpq_f64(self.neon) }}
      } else if #[cfg(feature="std")] {
        let base: [f64; 2] = cast(self);
        cast(base.map(|val| val.ceil()))
      } else {
        let base: [f64; 2] = cast(self);
        let rounded: [f64; 2] = cast(self.round());
        cast([
          if base[0] > rounded[0] { rounded[0] + 1.0 } else { rounded[0] },
          if base[1] > rounded[1] { rounded[1] + 1.0 } else { rounded[1] },
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
      if #[cfg(target_feature="sse2")] {
        Self { sse: max_m128d(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self {
          simd: f64x2_pmax(self.simd, rhs.simd),
        }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vmaxq_f64(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          if self.arr[0] < rhs.arr[0] { rhs.arr[0] } else { self.arr[0] },
          if self.arr[1] < rhs.arr[1] { rhs.arr[1] } else { self.arr[1] },
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
      if #[cfg(target_feature="sse2")] {
        // max_m128d seems to do rhs < self ? self : rhs. So if there's any NaN
        // involved, it chooses rhs, so we need to specifically check rhs for
        // NaN.
        rhs.is_nan().blend(self, Self { sse: max_m128d(self.sse, rhs.sse) })
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
            f64x2_pmax(self.simd, rhs.simd),
            f64x2_ne(self.simd, self.simd), // NaN check
          )
        }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vmaxnmq_f64(self.neon, rhs.neon) }}
            } else {
        Self { arr: [
          self.arr[0].max(rhs.arr[0]),
          self.arr[1].max(rhs.arr[1]),
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
      if #[cfg(target_feature="sse2")] {
        Self { sse: min_m128d(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self {
          simd: f64x2_pmin(self.simd, rhs.simd),
        }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vminq_f64(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          if self.arr[0] < rhs.arr[0] { self.arr[0] } else { rhs.arr[0] },
          if self.arr[1] < rhs.arr[1] { self.arr[1] } else { rhs.arr[1] },
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
      if #[cfg(target_feature="sse2")] {
        // min_m128d seems to do rhs < self ? rhs : self. So if there's any NaN
        // involved, it chooses rhs, so we need to specifically check rhs for
        // NaN.
        rhs.is_nan().blend(self, Self { sse: min_m128d(self.sse, rhs.sse) })
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
            f64x2_pmin(self.simd, rhs.simd),
            f64x2_ne(self.simd, self.simd), // NaN check
          )
        }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vminnmq_f64(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].min(rhs.arr[0]),
          self.arr[1].min(rhs.arr[1]),
        ]}
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn is_nan(self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_unord_mask_m128d(self.sse, self.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f64x2_ne(self.simd, self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f64_u64(vceqq_f64(self.neon, self.neon)) }.not() }
      } else {
        Self { arr: [
          if self.arr[0].is_nan() { f64::from_bits(u64::MAX) } else { 0.0 },
          if self.arr[1].is_nan() { f64::from_bits(u64::MAX) } else { 0.0 },
        ]}
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn is_finite(self) -> Self {
    let shifted_exp_mask = u64x2::from(0xFFE0000000000000);
    let u: u64x2 = cast(self);
    let shift_u = u << 1_u64;
    let out = !(shift_u & shifted_exp_mask).cmp_eq(shifted_exp_mask);
    cast(out)
  }
  #[inline]
  #[must_use]
  pub fn is_inf(self) -> Self {
    let shifted_inf = u64x2::from(0xFFE0000000000000);
    let u: u64x2 = cast(self);
    let shift_u = u << 1_u64;
    let out = (shift_u).cmp_eq(shifted_inf);
    cast(out)
  }

  #[inline]
  #[must_use]
  pub fn round(self) -> Self {
    pick! {
      if #[cfg(target_feature="sse4.1")] {
        Self { sse: round_m128d::<{round_op!(Nearest)}>(self.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f64x2_nearest(self.simd) }
      } else {
        let sign_mask = f64x2::from(-0.0);
        let magic = f64x2::from(f64::from_bits(0x43300000_00000000));
        let sign = self & sign_mask;
        let signed_magic = magic | sign;
        self + signed_magic - signed_magic
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn round_int(self) -> i64x2 {
    let rounded: [f64; 2] = cast(self.round());
    cast([rounded[0] as i64, rounded[1] as i64])
  }
  #[inline]
  #[must_use]
  pub fn mul_add(self, m: Self, a: Self) -> Self {
    pick! {
      if #[cfg(all(target_feature="fma"))] {
        Self { sse: fused_mul_add_m128d(self.sse, m.sse, a.sse) }
      } else {
        (self * m) + a
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn mul_sub(self, m: Self, a: Self) -> Self {
    pick! {
      if #[cfg(all(target_feature="fma"))] {
        Self { sse: fused_mul_sub_m128d(self.sse, m.sse, a.sse) }
      } else {
        (self * m) - a
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn mul_neg_add(self, m: Self, a: Self) -> Self {
    pick! {
        if #[cfg(all(target_feature="fma"))] {
          Self { sse: fused_mul_neg_add_m128d(self.sse, m.sse, a.sse) }
        } else {
          a - (self * m)
        }
    }
  }

  #[inline]
  #[must_use]
  pub fn mul_neg_sub(self, m: Self, a: Self) -> Self {
    pick! {
        if #[cfg(all(target_feature="fma"))] {
          Self { sse: fused_mul_neg_sub_m128d(self.sse, m.sse, a.sse) }
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
    let magnitude_mask = Self::from(f64::from_bits(u64::MAX >> 1));
    (self & magnitude_mask) | (sign & Self::from(-0.0))
  }

  #[inline]
  pub fn asin_acos(self) -> (Self, Self) {
    // Based on the Agner Fog "vector class library":
    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h
    const_f64_as_f64x2!(R4asin, 2.967721961301243206100E-3);
    const_f64_as_f64x2!(R3asin, -5.634242780008963776856E-1);
    const_f64_as_f64x2!(R2asin, 6.968710824104713396794E0);
    const_f64_as_f64x2!(R1asin, -2.556901049652824852289E1);
    const_f64_as_f64x2!(R0asin, 2.853665548261061424989E1);

    const_f64_as_f64x2!(S3asin, -2.194779531642920639778E1);
    const_f64_as_f64x2!(S2asin, 1.470656354026814941758E2);
    const_f64_as_f64x2!(S1asin, -3.838770957603691357202E2);
    const_f64_as_f64x2!(S0asin, 3.424398657913078477438E2);

    const_f64_as_f64x2!(P5asin, 4.253011369004428248960E-3);
    const_f64_as_f64x2!(P4asin, -6.019598008014123785661E-1);
    const_f64_as_f64x2!(P3asin, 5.444622390564711410273E0);
    const_f64_as_f64x2!(P2asin, -1.626247967210700244449E1);
    const_f64_as_f64x2!(P1asin, 1.956261983317594739197E1);
    const_f64_as_f64x2!(P0asin, -8.198089802484824371615E0);

    const_f64_as_f64x2!(Q4asin, -1.474091372988853791896E1);
    const_f64_as_f64x2!(Q3asin, 7.049610280856842141659E1);
    const_f64_as_f64x2!(Q2asin, -1.471791292232726029859E2);
    const_f64_as_f64x2!(Q1asin, 1.395105614657485689735E2);
    const_f64_as_f64x2!(Q0asin, -4.918853881490881290097E1);

    let xa = self.abs();

    let big = xa.cmp_ge(f64x2::splat(0.625));

    let x1 = big.blend(f64x2::splat(1.0) - xa, xa * xa);

    let x2 = x1 * x1;
    let x3 = x2 * x1;
    let x4 = x2 * x2;
    let x5 = x4 * x1;

    let do_big = big.any();
    let do_small = !big.all();

    let mut rx = f64x2::default();
    let mut sx = f64x2::default();
    let mut px = f64x2::default();
    let mut qx = f64x2::default();

    if do_big {
      rx = x3.mul_add(R3asin, x2 * R2asin)
        + x4.mul_add(R4asin, x1.mul_add(R1asin, R0asin));
      sx =
        x3.mul_add(S3asin, x4) + x2.mul_add(S2asin, x1.mul_add(S1asin, S0asin));
    }
    if do_small {
      px = x3.mul_add(P3asin, P0asin)
        + x4.mul_add(P4asin, x1 * P1asin)
        + x5.mul_add(P5asin, x2 * P2asin);
      qx = x4.mul_add(Q4asin, x5)
        + x3.mul_add(Q3asin, x1 * Q1asin)
        + x2.mul_add(Q2asin, Q0asin);
    };

    let vx = big.blend(rx, px);
    let wx = big.blend(sx, qx);

    let y1 = vx / wx * x1;

    let mut z1 = f64x2::default();
    let mut z2 = f64x2::default();
    if do_big {
      let xb = (x1 + x1).sqrt();
      z1 = xb.mul_add(y1, xb);
    }

    if do_small {
      z2 = xa.mul_add(y1, xa);
    }

    // asin
    let z3 = f64x2::FRAC_PI_2 - z1;
    let asin = big.blend(z3, z2);
    let asin = asin.flip_signs(self);

    // acos
    let z3 = self.cmp_lt(f64x2::ZERO).blend(f64x2::PI - z1, z1);
    let z4 = f64x2::FRAC_PI_2 - z2.flip_signs(self);
    let acos = big.blend(z3, z4);

    (asin, acos)
  }

  #[inline]
  pub fn acos(self) -> Self {
    // Based on the Agner Fog "vector class library":
    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h
    const_f64_as_f64x2!(R4asin, 2.967721961301243206100E-3);
    const_f64_as_f64x2!(R3asin, -5.634242780008963776856E-1);
    const_f64_as_f64x2!(R2asin, 6.968710824104713396794E0);
    const_f64_as_f64x2!(R1asin, -2.556901049652824852289E1);
    const_f64_as_f64x2!(R0asin, 2.853665548261061424989E1);

    const_f64_as_f64x2!(S3asin, -2.194779531642920639778E1);
    const_f64_as_f64x2!(S2asin, 1.470656354026814941758E2);
    const_f64_as_f64x2!(S1asin, -3.838770957603691357202E2);
    const_f64_as_f64x2!(S0asin, 3.424398657913078477438E2);

    const_f64_as_f64x2!(P5asin, 4.253011369004428248960E-3);
    const_f64_as_f64x2!(P4asin, -6.019598008014123785661E-1);
    const_f64_as_f64x2!(P3asin, 5.444622390564711410273E0);
    const_f64_as_f64x2!(P2asin, -1.626247967210700244449E1);
    const_f64_as_f64x2!(P1asin, 1.956261983317594739197E1);
    const_f64_as_f64x2!(P0asin, -8.198089802484824371615E0);

    const_f64_as_f64x2!(Q4asin, -1.474091372988853791896E1);
    const_f64_as_f64x2!(Q3asin, 7.049610280856842141659E1);
    const_f64_as_f64x2!(Q2asin, -1.471791292232726029859E2);
    const_f64_as_f64x2!(Q1asin, 1.395105614657485689735E2);
    const_f64_as_f64x2!(Q0asin, -4.918853881490881290097E1);

    let xa = self.abs();

    let big = xa.cmp_ge(f64x2::splat(0.625));

    let x1 = big.blend(f64x2::splat(1.0) - xa, xa * xa);

    let x2 = x1 * x1;
    let x3 = x2 * x1;
    let x4 = x2 * x2;
    let x5 = x4 * x1;

    let do_big = big.any();
    let do_small = !big.all();

    let mut rx = f64x2::default();
    let mut sx = f64x2::default();
    let mut px = f64x2::default();
    let mut qx = f64x2::default();

    if do_big {
      rx = x3.mul_add(R3asin, x2 * R2asin)
        + x4.mul_add(R4asin, x1.mul_add(R1asin, R0asin));
      sx =
        x3.mul_add(S3asin, x4) + x2.mul_add(S2asin, x1.mul_add(S1asin, S0asin));
    }
    if do_small {
      px = x3.mul_add(P3asin, P0asin)
        + x4.mul_add(P4asin, x1 * P1asin)
        + x5.mul_add(P5asin, x2 * P2asin);
      qx = x4.mul_add(Q4asin, x5)
        + x3.mul_add(Q3asin, x1 * Q1asin)
        + x2.mul_add(Q2asin, Q0asin);
    };

    let vx = big.blend(rx, px);
    let wx = big.blend(sx, qx);

    let y1 = vx / wx * x1;

    let mut z1 = f64x2::default();
    let mut z2 = f64x2::default();
    if do_big {
      let xb = (x1 + x1).sqrt();
      z1 = xb.mul_add(y1, xb);
    }

    if do_small {
      z2 = xa.mul_add(y1, xa);
    }

    // acos
    let z3 = self.cmp_lt(f64x2::ZERO).blend(f64x2::PI - z1, z1);
    let z4 = f64x2::FRAC_PI_2 - z2.flip_signs(self);
    let acos = big.blend(z3, z4);

    acos
  }

  #[inline]
  pub fn asin(self) -> Self {
    // Based on the Agner Fog "vector class library":
    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h
    const_f64_as_f64x2!(R4asin, 2.967721961301243206100E-3);
    const_f64_as_f64x2!(R3asin, -5.634242780008963776856E-1);
    const_f64_as_f64x2!(R2asin, 6.968710824104713396794E0);
    const_f64_as_f64x2!(R1asin, -2.556901049652824852289E1);
    const_f64_as_f64x2!(R0asin, 2.853665548261061424989E1);

    const_f64_as_f64x2!(S3asin, -2.194779531642920639778E1);
    const_f64_as_f64x2!(S2asin, 1.470656354026814941758E2);
    const_f64_as_f64x2!(S1asin, -3.838770957603691357202E2);
    const_f64_as_f64x2!(S0asin, 3.424398657913078477438E2);

    const_f64_as_f64x2!(P5asin, 4.253011369004428248960E-3);
    const_f64_as_f64x2!(P4asin, -6.019598008014123785661E-1);
    const_f64_as_f64x2!(P3asin, 5.444622390564711410273E0);
    const_f64_as_f64x2!(P2asin, -1.626247967210700244449E1);
    const_f64_as_f64x2!(P1asin, 1.956261983317594739197E1);
    const_f64_as_f64x2!(P0asin, -8.198089802484824371615E0);

    const_f64_as_f64x2!(Q4asin, -1.474091372988853791896E1);
    const_f64_as_f64x2!(Q3asin, 7.049610280856842141659E1);
    const_f64_as_f64x2!(Q2asin, -1.471791292232726029859E2);
    const_f64_as_f64x2!(Q1asin, 1.395105614657485689735E2);
    const_f64_as_f64x2!(Q0asin, -4.918853881490881290097E1);

    let xa = self.abs();

    let big = xa.cmp_ge(f64x2::splat(0.625));

    let x1 = big.blend(f64x2::splat(1.0) - xa, xa * xa);

    let x2 = x1 * x1;
    let x3 = x2 * x1;
    let x4 = x2 * x2;
    let x5 = x4 * x1;

    let do_big = big.any();
    let do_small = !big.all();

    let mut rx = f64x2::default();
    let mut sx = f64x2::default();
    let mut px = f64x2::default();
    let mut qx = f64x2::default();

    if do_big {
      rx = x3.mul_add(R3asin, x2 * R2asin)
        + x4.mul_add(R4asin, x1.mul_add(R1asin, R0asin));
      sx =
        x3.mul_add(S3asin, x4) + x2.mul_add(S2asin, x1.mul_add(S1asin, S0asin));
    }
    if do_small {
      px = x3.mul_add(P3asin, P0asin)
        + x4.mul_add(P4asin, x1 * P1asin)
        + x5.mul_add(P5asin, x2 * P2asin);
      qx = x4.mul_add(Q4asin, x5)
        + x3.mul_add(Q3asin, x1 * Q1asin)
        + x2.mul_add(Q2asin, Q0asin);
    };

    let vx = big.blend(rx, px);
    let wx = big.blend(sx, qx);

    let y1 = vx / wx * x1;

    let mut z1 = f64x2::default();
    let mut z2 = f64x2::default();
    if do_big {
      let xb = (x1 + x1).sqrt();
      z1 = xb.mul_add(y1, xb);
    }

    if do_small {
      z2 = xa.mul_add(y1, xa);
    }

    // asin
    let z3 = f64x2::FRAC_PI_2 - z1;
    let asin = big.blend(z3, z2);
    let asin = asin.flip_signs(self);

    asin
  }

  #[inline]
  pub fn atan(self) -> Self {
    // Based on the Agner Fog "vector class library":
    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h
    const_f64_as_f64x2!(MORE_BITS, 6.123233995736765886130E-17);
    const_f64_as_f64x2!(MORE_BITS_O2, 6.123233995736765886130E-17 * 0.5);
    const_f64_as_f64x2!(T3PO8, core::f64::consts::SQRT_2 + 1.0);

    const_f64_as_f64x2!(P4atan, -8.750608600031904122785E-1);
    const_f64_as_f64x2!(P3atan, -1.615753718733365076637E1);
    const_f64_as_f64x2!(P2atan, -7.500855792314704667340E1);
    const_f64_as_f64x2!(P1atan, -1.228866684490136173410E2);
    const_f64_as_f64x2!(P0atan, -6.485021904942025371773E1);

    const_f64_as_f64x2!(Q4atan, 2.485846490142306297962E1);
    const_f64_as_f64x2!(Q3atan, 1.650270098316988542046E2);
    const_f64_as_f64x2!(Q2atan, 4.328810604912902668951E2);
    const_f64_as_f64x2!(Q1atan, 4.853903996359136964868E2);
    const_f64_as_f64x2!(Q0atan, 1.945506571482613964425E2);

    let t = self.abs();

    // small:  t < 0.66
    // medium: t <= t <= 2.4142 (1+sqrt(2))
    // big:    t > 2.4142
    let notbig = t.cmp_le(T3PO8);
    let notsmal = t.cmp_ge(Self::splat(0.66));

    let mut s = notbig.blend(Self::FRAC_PI_4, Self::FRAC_PI_2);
    s = notsmal & s;
    let mut fac = notbig.blend(MORE_BITS_O2, MORE_BITS);
    fac = notsmal & fac;

    // small:  z = t / 1.0;
    // medium: z = (t-1.0) / (t+1.0);
    // big:    z = -1.0 / t;
    let mut a = notbig & t;
    a = notsmal.blend(a - Self::ONE, a);
    let mut b = notbig & Self::ONE;
    b = notsmal.blend(b + t, b);
    let z = a / b;

    let zz = z * z;

    let px = polynomial_4!(zz, P0atan, P1atan, P2atan, P3atan, P4atan);
    let qx = polynomial_5n!(zz, Q0atan, Q1atan, Q2atan, Q3atan, Q4atan);

    let mut re = (px / qx).mul_add(z * zz, z);
    re += s + fac;

    // get sign bit
    re = (self.sign_bit()).blend(-re, re);

    re
  }

  #[inline]
  pub fn atan2(self, x: Self) -> Self {
    // Based on the Agner Fog "vector class library":
    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h
    const_f64_as_f64x2!(MORE_BITS, 6.123233995736765886130E-17);
    const_f64_as_f64x2!(MORE_BITS_O2, 6.123233995736765886130E-17 * 0.5);
    const_f64_as_f64x2!(T3PO8, core::f64::consts::SQRT_2 + 1.0);

    const_f64_as_f64x2!(P4atan, -8.750608600031904122785E-1);
    const_f64_as_f64x2!(P3atan, -1.615753718733365076637E1);
    const_f64_as_f64x2!(P2atan, -7.500855792314704667340E1);
    const_f64_as_f64x2!(P1atan, -1.228866684490136173410E2);
    const_f64_as_f64x2!(P0atan, -6.485021904942025371773E1);

    const_f64_as_f64x2!(Q4atan, 2.485846490142306297962E1);
    const_f64_as_f64x2!(Q3atan, 1.650270098316988542046E2);
    const_f64_as_f64x2!(Q2atan, 4.328810604912902668951E2);
    const_f64_as_f64x2!(Q1atan, 4.853903996359136964868E2);
    const_f64_as_f64x2!(Q0atan, 1.945506571482613964425E2);

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

    // x = y = 0 gives NAN here
    let t = y2 / x2;

    // small:  t < 0.66
    // medium: t <= t <= 2.4142 (1+sqrt(2))
    // big:    t > 2.4142
    let notbig = t.cmp_le(T3PO8);
    let notsmal = t.cmp_ge(Self::splat(0.66));

    let mut s = notbig.blend(Self::FRAC_PI_4, Self::FRAC_PI_2);
    s = notsmal & s;
    let mut fac = notbig.blend(MORE_BITS_O2, MORE_BITS);
    fac = notsmal & fac;

    // small:  z = t / 1.0;
    // medium: z = (t-1.0) / (t+1.0);
    // big:    z = -1.0 / t;
    let mut a = notbig & t;
    a = notsmal.blend(a - Self::ONE, a);
    let mut b = notbig & Self::ONE;
    b = notsmal.blend(b + t, b);
    let z = a / b;

    let zz = z * z;

    let px = polynomial_4!(zz, P0atan, P1atan, P2atan, P3atan, P4atan);
    let qx = polynomial_5n!(zz, Q0atan, Q1atan, Q2atan, Q3atan, Q4atan);

    let mut re = (px / qx).mul_add(z * zz, z);
    re += s + fac;

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

    const_f64_as_f64x2!(P0sin, -1.66666666666666307295E-1);
    const_f64_as_f64x2!(P1sin, 8.33333333332211858878E-3);
    const_f64_as_f64x2!(P2sin, -1.98412698295895385996E-4);
    const_f64_as_f64x2!(P3sin, 2.75573136213857245213E-6);
    const_f64_as_f64x2!(P4sin, -2.50507477628578072866E-8);
    const_f64_as_f64x2!(P5sin, 1.58962301576546568060E-10);

    const_f64_as_f64x2!(P0cos, 4.16666666666665929218E-2);
    const_f64_as_f64x2!(P1cos, -1.38888888888730564116E-3);
    const_f64_as_f64x2!(P2cos, 2.48015872888517045348E-5);
    const_f64_as_f64x2!(P3cos, -2.75573141792967388112E-7);
    const_f64_as_f64x2!(P4cos, 2.08757008419747316778E-9);
    const_f64_as_f64x2!(P5cos, -1.13585365213876817300E-11);

    const_f64_as_f64x2!(DP1, 7.853981554508209228515625E-1 * 2.);
    const_f64_as_f64x2!(DP2, 7.94662735614792836714E-9 * 2.);
    const_f64_as_f64x2!(DP3, 3.06161699786838294307E-17 * 2.);

    const_f64_as_f64x2!(TWO_OVER_PI, 2.0 / core::f64::consts::PI);

    let xa = self.abs();

    let y = (xa * TWO_OVER_PI).round();
    let q = y.round_int();

    let x = y.mul_neg_add(DP3, y.mul_neg_add(DP2, y.mul_neg_add(DP1, xa)));

    let x2 = x * x;
    let mut s = polynomial_5!(x2, P0sin, P1sin, P2sin, P3sin, P4sin, P5sin);
    let mut c = polynomial_5!(x2, P0cos, P1cos, P2cos, P3cos, P4cos, P5cos);
    s = (x * x2).mul_add(s, x);
    c =
      (x2 * x2).mul_add(c, x2.mul_neg_add(f64x2::from(0.5), f64x2::from(1.0)));

    let swap = !((q & i64x2::from(1)).cmp_eq(i64x2::from(0)));

    let mut overflow: f64x2 = cast(q.cmp_gt(i64x2::from(0x80000000000000)));
    overflow &= xa.is_finite();
    s = overflow.blend(f64x2::from(0.0), s);
    c = overflow.blend(f64x2::from(1.0), c);

    // calc sin
    let mut sin1 = cast::<_, f64x2>(swap).blend(c, s);
    let sign_sin: i64x2 = (q << 62) ^ cast::<_, i64x2>(self);
    sin1 = sin1.flip_signs(cast(sign_sin));

    // calc cos
    let mut cos1 = cast::<_, f64x2>(swap).blend(s, c);
    let sign_cos: i64x2 = ((q + i64x2::from(1)) & i64x2::from(2)) << 62;
    cos1 ^= cast::<_, f64x2>(sign_cos);

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
    const_f64_as_f64x2!(RAD_TO_DEG_RATIO, 180.0_f64 / core::f64::consts::PI);
    self * RAD_TO_DEG_RATIO
  }
  #[inline]
  #[must_use]
  pub fn to_radians(self) -> Self {
    const_f64_as_f64x2!(DEG_TO_RAD_RATIO, core::f64::consts::PI / 180.0_f64);
    self * DEG_TO_RAD_RATIO
  }
  #[inline]
  #[must_use]
  pub fn sqrt(self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: sqrt_m128d(self.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f64x2_sqrt(self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vsqrtq_f64(self.neon) }}
      } else if #[cfg(feature="std")] {
        Self { arr: [
          self.arr[0].sqrt(),
          self.arr[1].sqrt(),
        ]}
      } else {
        Self { arr: [
          software_sqrt(self.arr[0]),
          software_sqrt(self.arr[1]),
        ]}
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn move_mask(self) -> i32 {
    pick! {
      if #[cfg(target_feature="sse2")] {
        move_mask_m128d(self.sse)
      } else if #[cfg(target_feature="simd128")] {
        u64x2_bitmask(self.simd) as i32
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe
        {
          let e = vreinterpretq_u64_f64(self.neon);

          (vgetq_lane_u64(e,0) >> 63 | ((vgetq_lane_u64(e,1) >> 62) & 0x2)) as i32
        }
      } else {
        (((self.arr[0].to_bits() as i64) < 0) as i32) << 0 |
        (((self.arr[1].to_bits() as i64) < 0) as i32) << 1
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
        u64x2_all_true(self.simd)
      } else {
        // two lanes
        self.move_mask() == 0b11
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
    const_f64_as_f64x2!(pow2_52, 4503599627370496.0);
    const_f64_as_f64x2!(bias, 1023.0);
    let a = self + (bias + pow2_52);
    let c = cast::<_, i64x2>(a) << 52;
    cast::<_, f64x2>(c)
  }

  /// Calculate the exponent of a packed `f64x2`
  #[inline]
  #[must_use]
  pub fn exp(self) -> Self {
    const_f64_as_f64x2!(P2, 1.0 / 2.0);
    const_f64_as_f64x2!(P3, 1.0 / 6.0);
    const_f64_as_f64x2!(P4, 1. / 24.);
    const_f64_as_f64x2!(P5, 1. / 120.);
    const_f64_as_f64x2!(P6, 1. / 720.);
    const_f64_as_f64x2!(P7, 1. / 5040.);
    const_f64_as_f64x2!(P8, 1. / 40320.);
    const_f64_as_f64x2!(P9, 1. / 362880.);
    const_f64_as_f64x2!(P10, 1. / 3628800.);
    const_f64_as_f64x2!(P11, 1. / 39916800.);
    const_f64_as_f64x2!(P12, 1. / 479001600.);
    const_f64_as_f64x2!(P13, 1. / 6227020800.);
    const_f64_as_f64x2!(LN2D_HI, 0.693145751953125);
    const_f64_as_f64x2!(LN2D_LO, 1.42860682030941723212E-6);
    let max_x = f64x2::from(708.39);
    let r = (self * Self::LOG2_E).round();
    let x = r.mul_neg_add(LN2D_HI, self);
    let x = r.mul_neg_add(LN2D_LO, x);
    let z =
      polynomial_13!(x, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13);
    let n2 = Self::vm_pow2n(r);
    let z = (z + Self::ONE) * n2;
    // check for overflow
    let in_range = self.abs().cmp_lt(max_x);
    let in_range = in_range & self.is_finite();
    in_range.blend(z, Self::ZERO)
  }

  #[inline]
  fn exponent(self) -> f64x2 {
    const_f64_as_f64x2!(pow2_52, 4503599627370496.0);
    const_f64_as_f64x2!(bias, 1023.0);
    let a = cast::<_, u64x2>(self);
    let b = a >> 52;
    let c = b | cast::<_, u64x2>(pow2_52);
    let d = cast::<_, f64x2>(c);
    let e = d - (pow2_52 + bias);
    e
  }

  #[inline]
  fn fraction_2(self) -> Self {
    let t1 = cast::<_, u64x2>(self);
    let t2 = cast::<_, u64x2>(
      (t1 & u64x2::from(0x000FFFFFFFFFFFFF)) | u64x2::from(0x3FE0000000000000),
    );
    cast::<_, f64x2>(t2)
  }

  #[inline]
  fn is_zero_or_subnormal(self) -> Self {
    let t = cast::<_, i64x2>(self);
    let t = t & i64x2::splat(0x7FF0000000000000);
    i64x2::round_float(t.cmp_eq(i64x2::splat(0)))
  }

  #[inline]
  fn infinity() -> Self {
    cast::<_, f64x2>(i64x2::splat(0x7FF0000000000000))
  }

  #[inline]
  fn nan_log() -> Self {
    cast::<_, f64x2>(i64x2::splat(0x7FF8000000000000 | 0x101 << 29))
  }

  #[inline]
  fn nan_pow() -> Self {
    cast::<_, f64x2>(i64x2::splat(0x7FF8000000000000 | 0x101 << 29))
  }

  #[inline]
  fn sign_bit(self) -> Self {
    let t1 = cast::<_, i64x2>(self);
    let t2 = t1 >> 63;
    !cast::<_, f64x2>(t2).cmp_eq(f64x2::ZERO)
  }

  /// horizontal add of all the elements of the vector
  #[inline]
  #[must_use]
  pub fn reduce_add(self) -> f64 {
    pick! {
      if #[cfg(target_feature="ssse3")] {
        let a = add_horizontal_m128d(self.sse, self.sse);
        a.to_array()[0]
      } else if #[cfg(any(target_feature="sse2", target_feature="simd128"))] {
        let a: [f64;2] = cast(self);
        a.iter().sum()
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { vgetq_lane_f64(self.neon,0) + vgetq_lane_f64(self.neon,1) }
      } else {
        self.arr.iter().sum()
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn ln(self) -> Self {
    const_f64_as_f64x2!(P0, 7.70838733755885391666E0);
    const_f64_as_f64x2!(P1, 1.79368678507819816313E1);
    const_f64_as_f64x2!(P2, 1.44989225341610930846E1);
    const_f64_as_f64x2!(P3, 4.70579119878881725854E0);
    const_f64_as_f64x2!(P4, 4.97494994976747001425E-1);
    const_f64_as_f64x2!(P5, 1.01875663804580931796E-4);

    const_f64_as_f64x2!(Q0, 2.31251620126765340583E1);
    const_f64_as_f64x2!(Q1, 7.11544750618563894466E1);
    const_f64_as_f64x2!(Q2, 8.29875266912776603211E1);
    const_f64_as_f64x2!(Q3, 4.52279145837532221105E1);
    const_f64_as_f64x2!(Q4, 1.12873587189167450590E1);
    const_f64_as_f64x2!(LN2F_HI, 0.693359375);
    const_f64_as_f64x2!(LN2F_LO, -2.12194440e-4);
    const_f64_as_f64x2!(VM_SQRT2, 1.414213562373095048801);
    const_f64_as_f64x2!(VM_SMALLEST_NORMAL, 1.17549435E-38);

    let x1 = self;
    let x = Self::fraction_2(x1);
    let e = Self::exponent(x1);
    let mask = x.cmp_gt(VM_SQRT2 * f64x2::HALF);
    let x = (!mask).blend(x + x, x);
    let fe = mask.blend(e + Self::ONE, e);
    let x = x - Self::ONE;
    let px = polynomial_5!(x, P0, P1, P2, P3, P4, P5);
    let x2 = x * x;
    let px = x2 * x * px;
    let qx = polynomial_5n!(x, Q0, Q1, Q2, Q3, Q4);
    let res = px / qx;
    let res = fe.mul_add(LN2F_LO, res);
    let res = res + x2.mul_neg_add(f64x2::HALF, x);
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
  pub fn pow_f64x2(self, y: Self) -> Self {
    const_f64_as_f64x2!(ln2d_hi, 0.693145751953125);
    const_f64_as_f64x2!(ln2d_lo, 1.42860682030941723212E-6);
    const_f64_as_f64x2!(P0log, 2.0039553499201281259648E1);
    const_f64_as_f64x2!(P1log, 5.7112963590585538103336E1);
    const_f64_as_f64x2!(P2log, 6.0949667980987787057556E1);
    const_f64_as_f64x2!(P3log, 2.9911919328553073277375E1);
    const_f64_as_f64x2!(P4log, 6.5787325942061044846969E0);
    const_f64_as_f64x2!(P5log, 4.9854102823193375972212E-1);
    const_f64_as_f64x2!(P6log, 4.5270000862445199635215E-5);
    const_f64_as_f64x2!(Q0log, 6.0118660497603843919306E1);
    const_f64_as_f64x2!(Q1log, 2.1642788614495947685003E2);
    const_f64_as_f64x2!(Q2log, 3.0909872225312059774938E2);
    const_f64_as_f64x2!(Q3log, 2.2176239823732856465394E2);
    const_f64_as_f64x2!(Q4log, 8.3047565967967209469434E1);
    const_f64_as_f64x2!(Q5log, 1.5062909083469192043167E1);

    // Taylor expansion constants
    const_f64_as_f64x2!(p2, 1.0 / 2.0); // coefficients for Taylor expansion of exp
    const_f64_as_f64x2!(p3, 1.0 / 6.0);
    const_f64_as_f64x2!(p4, 1.0 / 24.0);
    const_f64_as_f64x2!(p5, 1.0 / 120.0);
    const_f64_as_f64x2!(p6, 1.0 / 720.0);
    const_f64_as_f64x2!(p7, 1.0 / 5040.0);
    const_f64_as_f64x2!(p8, 1.0 / 40320.0);
    const_f64_as_f64x2!(p9, 1.0 / 362880.0);
    const_f64_as_f64x2!(p10, 1.0 / 3628800.0);
    const_f64_as_f64x2!(p11, 1.0 / 39916800.0);
    const_f64_as_f64x2!(p12, 1.0 / 479001600.0);
    const_f64_as_f64x2!(p13, 1.0 / 6227020800.0);

    let x1 = self.abs();
    let x = x1.fraction_2();
    let mask = x.cmp_gt(f64x2::SQRT_2 * f64x2::HALF);
    let x = (!mask).blend(x + x, x);
    let x = x - f64x2::ONE;
    let x2 = x * x;
    let px = polynomial_6!(x, P0log, P1log, P2log, P3log, P4log, P5log, P6log);
    let px = px * x * x2;
    let qx = polynomial_6n!(x, Q0log, Q1log, Q2log, Q3log, Q4log, Q5log);
    let lg1 = px / qx;

    let ef = x1.exponent();
    let ef = mask.blend(ef + f64x2::ONE, ef);
    let e1 = (ef * y).round();
    let yr = ef.mul_sub(y, e1);

    let lg = f64x2::HALF.mul_neg_add(x2, x) + lg1;
    let x2err = (f64x2::HALF * x).mul_sub(x, f64x2::HALF * x2);
    let lg_err = f64x2::HALF.mul_add(x2, lg - x) - lg1;

    let e2 = (lg * y * f64x2::LOG2_E).round();
    let v = lg.mul_sub(y, e2 * ln2d_hi);
    let v = e2.mul_neg_add(ln2d_lo, v);
    let v = v - (lg_err + x2err).mul_sub(y, yr * f64x2::LN_2);

    let x = v;
    let e3 = (x * f64x2::LOG2_E).round();
    let x = e3.mul_neg_add(f64x2::LN_2, x);
    let z =
      polynomial_13m!(x, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13)
        + f64x2::ONE;
    let ee = e1 + e2 + e3;
    let ei = cast::<_, i64x2>(ee.round_int());
    let ej = cast::<_, i64x2>(ei + (cast::<_, i64x2>(z) >> 52));

    let overflow = cast::<_, f64x2>(!ej.cmp_lt(i64x2::splat(0x07FF)))
      | ee.cmp_gt(f64x2::splat(3000.0));
    let underflow = cast::<_, f64x2>(!ej.cmp_gt(i64x2::splat(0x000)))
      | ee.cmp_lt(f64x2::splat(-3000.0));

    // Add exponent by integer addition
    let z = cast::<_, f64x2>(cast::<_, i64x2>(z) + (ei << 52));

    // Check for overflow/underflow
    let z = if (overflow | underflow).any() {
      let z = underflow.blend(f64x2::ZERO, z);
      overflow.blend(Self::infinity(), z)
    } else {
      z
    };

    // Check for self == 0
    let x_zero = self.is_zero_or_subnormal();
    let z = x_zero.blend(
      y.cmp_lt(f64x2::ZERO).blend(
        Self::infinity(),
        y.cmp_eq(f64x2::ZERO).blend(f64x2::ONE, f64x2::ZERO),
      ),
      z,
    );

    let x_sign = self.sign_bit();
    let z = if x_sign.any() {
      // Y into an integer
      let yi = y.cmp_eq(y.round());
      // Is y odd?
      let y_odd = cast::<_, i64x2>(y.round_int() << 63).round_float();

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
  pub fn powf(self, y: f64) -> Self {
    Self::pow_f64x2(self, f64x2::splat(y))
  }

  #[inline]
  pub fn to_array(self) -> [f64; 2] {
    cast(self)
  }

  #[inline]
  pub fn as_array_ref(&self) -> &[f64; 2] {
    cast_ref(self)
  }

  #[inline]
  pub fn as_array_mut(&mut self) -> &mut [f64; 2] {
    cast_mut(self)
  }

  /// Converts the lower two `i32` lanes to two `f64` lanes (and dropping the
  /// higher two `i32` lanes)
  #[inline]
  pub fn from_i32x4_lower2(v: i32x4) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: convert_to_m128d_from_lower2_i32_m128i(v.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: f64x2_convert_low_i32x4(v.simd)}
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
        Self { neon: unsafe { vcvtq_f64_s64(vmovl_s32(vget_low_s32(v.neon))) }}
      } else {
        Self { arr: [
            v.as_array_ref()[0] as f64,
            v.as_array_ref()[1] as f64,
        ]}
      }
    }
  }
}

impl From<i32x4> for f64x2 {
  /// Converts the lower two `i32` lanes to two `f64` lanes (and dropping the
  /// higher two `i32` lanes)
  #[inline]
  fn from(v: i32x4) -> Self {
    Self::from_i32x4_lower2(v)
  }
}

impl Not for f64x2 {
  type Output = Self;
  #[inline]
  fn not(self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: self.sse.not() }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: v128_not(self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_f64_u32(vmvnq_u32(vreinterpretq_u32_f64(self.neon))) }}
      } else {
        Self { arr: [
          f64::from_bits(!self.arr[0].to_bits()),
          f64::from_bits(!self.arr[1].to_bits()),
        ]}
      }
    }
  }
}
