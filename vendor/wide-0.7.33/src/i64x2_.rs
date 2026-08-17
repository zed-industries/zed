use super::*;

pick! {
  if #[cfg(target_feature="sse2")] {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(16))]
    pub struct i64x2 { pub(crate) sse: m128i }
  } else if #[cfg(target_feature="simd128")] {
    use core::arch::wasm32::*;

    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct i64x2 { pub(crate) simd: v128 }

    impl Default for i64x2 {
      fn default() -> Self {
        Self::splat(0)
      }
    }

    impl PartialEq for i64x2 {
      fn eq(&self, other: &Self) -> bool {
        u64x2_all_true(i64x2_eq(self.simd, other.simd))
      }
    }

    impl Eq for i64x2 { }
  } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
    use core::arch::aarch64::*;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct i64x2 { pub(crate) neon : int64x2_t }

    impl Default for i64x2 {
      #[inline]
      #[must_use]
      fn default() -> Self {
        unsafe { Self { neon: vdupq_n_s64(0)} }
      }
    }

    impl PartialEq for i64x2 {
      #[inline]
      #[must_use]
      fn eq(&self, other: &Self) -> bool {
        unsafe {
          vgetq_lane_s64(self.neon,0) == vgetq_lane_s64(other.neon,0) && vgetq_lane_s64(self.neon,1) == vgetq_lane_s64(other.neon,1)
        }
      }
    }

    impl Eq for i64x2 { }
  } else {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(16))]
    pub struct i64x2 { arr: [i64;2] }
  }
}

int_uint_consts!(i64, 2, i64x2, 128);

unsafe impl Zeroable for i64x2 {}
unsafe impl Pod for i64x2 {}

impl Add for i64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: add_i64_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i64x2_add(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vaddq_s64(self.neon, rhs.neon) } }
      } else {
        Self { arr: [
          self.arr[0].wrapping_add(rhs.arr[0]),
          self.arr[1].wrapping_add(rhs.arr[1]),
        ]}
      }
    }
  }
}

impl Sub for i64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: sub_i64_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i64x2_sub(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vsubq_s64(self.neon, rhs.neon) } }
      } else {
        Self { arr: [
          self.arr[0].wrapping_sub(rhs.arr[0]),
          self.arr[1].wrapping_sub(rhs.arr[1]),
        ]}
      }
    }
  }
}

//we should try to implement this on sse2
impl Mul for i64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="simd128")] {
        Self { simd: i64x2_mul(self.simd, rhs.simd) }
      } else {
        let arr1: [i64; 2] = cast(self);
        let arr2: [i64; 2] = cast(rhs);
        cast([
          arr1[0].wrapping_mul(arr2[0]),
          arr1[1].wrapping_mul(arr2[1]),
        ])
      }
    }
  }
}

impl Add<i64> for i64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: i64) -> Self::Output {
    self.add(Self::splat(rhs))
  }
}

impl Sub<i64> for i64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: i64) -> Self::Output {
    self.sub(Self::splat(rhs))
  }
}

impl Mul<i64> for i64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: i64) -> Self::Output {
    self.mul(Self::splat(rhs))
  }
}

impl Add<i64x2> for i64 {
  type Output = i64x2;
  #[inline]
  #[must_use]
  fn add(self, rhs: i64x2) -> Self::Output {
    i64x2::splat(self).add(rhs)
  }
}

impl Sub<i64x2> for i64 {
  type Output = i64x2;
  #[inline]
  #[must_use]
  fn sub(self, rhs: i64x2) -> Self::Output {
    i64x2::splat(self).sub(rhs)
  }
}

impl Mul<i64x2> for i64 {
  type Output = i64x2;
  #[inline]
  #[must_use]
  fn mul(self, rhs: i64x2) -> Self::Output {
    i64x2::splat(self).mul(rhs)
  }
}

impl BitAnd for i64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitand(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: bitand_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: v128_and(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vandq_s64(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].bitand(rhs.arr[0]),
          self.arr[1].bitand(rhs.arr[1]),
        ]}
      }
    }
  }
}

impl BitOr for i64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitor(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: bitor_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: v128_or(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vorrq_s64(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].bitor(rhs.arr[0]),
          self.arr[1].bitor(rhs.arr[1]),
        ]}
      }
    }
  }
}

impl BitXor for i64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitxor(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: bitxor_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: v128_xor(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: veorq_s64(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].bitxor(rhs.arr[0]),
          self.arr[1].bitxor(rhs.arr[1]),
        ]}
      }
    }
  }
}

macro_rules! impl_shl_t_for_i64x2 {
  ($($shift_type:ty),+ $(,)?) => {
    $(impl Shl<$shift_type> for i64x2 {
      type Output = Self;
      /// Shifts all lanes by the value given.
      #[inline]
      #[must_use]
      fn shl(self, rhs: $shift_type) -> Self::Output {
        pick! {
          if #[cfg(target_feature="sse2")] {
            let shift = cast([rhs as u64, 0]);
            Self { sse: shl_all_u64_m128i(self.sse, shift) }
          } else if #[cfg(target_feature="simd128")] {
            Self { simd: i64x2_shl(self.simd, rhs as u32) }
          } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
            unsafe {Self { neon: vshlq_s64(self.neon, vmovq_n_s64(rhs as i64)) }}
          } else {
            let u = rhs as u64;
            Self { arr: [
              self.arr[0] << u,
              self.arr[1] << u,
            ]}
          }
        }
      }
    })+
  };
}
impl_shl_t_for_i64x2!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

macro_rules! impl_shr_t_for_i64x2 {
  ($($shift_type:ty),+ $(,)?) => {
    $(impl Shr<$shift_type> for i64x2 {
      type Output = Self;
      /// Shifts all lanes by the value given.
      #[inline]
      #[must_use]
      fn shr(self, rhs: $shift_type) -> Self::Output {
        pick! {
          if #[cfg(target_feature="simd128")] {
            Self { simd: i64x2_shr(self.simd, rhs as u32) }
          } else {
            let u = rhs as u64;
            let arr: [i64; 2] = cast(self);
            cast([
              arr[0] >> u,
              arr[1] >> u,
            ])
          }
        }
      }
    })+
  };
}

impl_shr_t_for_i64x2!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

impl CmpEq for i64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_eq(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse4.1")] {
        Self { sse: cmp_eq_mask_i64_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i64x2_eq(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_s64_u64(vceqq_s64(self.neon, rhs.neon)) }}
      } else {
        let s: [i64;2] = cast(self);
        let r: [i64;2] = cast(rhs);
        cast([
          if s[0] == r[0] { -1_i64 } else { 0 },
          if s[1] == r[1] { -1_i64 } else { 0 },
        ])
      }
    }
  }
}

impl CmpGt for i64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_gt(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse4.2")] {
        Self { sse: cmp_gt_mask_i64_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i64x2_gt(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_s64_u64(vcgtq_s64(self.neon, rhs.neon)) }}
      } else {
        let s: [i64;2] = cast(self);
        let r: [i64;2] = cast(rhs);
        cast([
          if s[0] > r[0] { -1_i64 } else { 0 },
          if s[1] > r[1] { -1_i64 } else { 0 },
        ])
      }
    }
  }
}

impl CmpLt for i64x2 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_lt(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse4.2")] {
        Self { sse: !cmp_gt_mask_i64_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i64x2_lt(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_s64_u64(vcltq_s64(self.neon, rhs.neon)) }}
      } else {
        let s: [i64;2] = cast(self);
        let r: [i64;2] = cast(rhs);
        cast([
          if s[0] < r[0] { -1_i64 } else { 0 },
          if s[1] < r[1] { -1_i64 } else { 0 },
        ])
      }
    }
  }
}

impl i64x2 {
  #[inline]
  #[must_use]
  pub const fn new(array: [i64; 2]) -> Self {
    unsafe { core::mem::transmute(array) }
  }
  #[inline]
  #[must_use]
  pub fn blend(self, t: Self, f: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse4.1")] {
        Self { sse: blend_varying_i8_m128i(f.sse, t.sse, self.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: v128_bitselect(t.simd, f.simd, self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vbslq_s64(vreinterpretq_u64_s64(self.neon), t.neon, f.neon) }}
      } else {
        generic_bit_blend(self, t, f)
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn abs(self) -> Self {
    pick! {
      // x86 doesn't have this builtin
      if #[cfg(target_feature="simd128")] {
        Self { simd: i64x2_abs(self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vabsq_s64(self.neon) }}
      } else {
        let arr: [i64; 2] = cast(self);
        cast(
          [
            arr[0].wrapping_abs(),
            arr[1].wrapping_abs(),
          ])
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn unsigned_abs(self) -> u64x2 {
    pick! {
      // x86 doesn't have this builtin
      if #[cfg(target_feature="simd128")] {
        u64x2 { simd: i64x2_abs(self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {u64x2 { neon: vreinterpretq_u64_s64(vabsq_s64(self.neon)) }}
      } else {
        let arr: [i64; 2] = cast(self);
        cast(
          [
            arr[0].unsigned_abs(),
            arr[1].unsigned_abs(),
          ])
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn round_float(self) -> f64x2 {
    let arr: [i64; 2] = cast(self);
    cast([arr[0] as f64, arr[1] as f64])
  }

  /// returns the bit mask for each high bit set in the vector with the lowest
  /// lane being the lowest bit
  #[inline]
  #[must_use]
  pub fn move_mask(self) -> i32 {
    pick! {
      if #[cfg(target_feature="sse")] {
        // use f64 move_mask since it is the same size as i64
        move_mask_m128d(cast(self.sse))
      } else if #[cfg(target_feature="simd128")] {
        i64x2_bitmask(self.simd) as i32
      } else {
        // nothing amazingly efficient for neon
        let arr: [u64; 2] = cast(self);
        (arr[0] >> 63 | ((arr[1] >> 62) & 2)) as i32
      }
    }
  }

  /// true if any high bits are set for any value in the vector
  #[inline]
  #[must_use]
  pub fn any(self) -> bool {
    pick! {
      if #[cfg(target_feature="sse")] {
        // use f64 move_mask since it is the same size as i64
        move_mask_m128d(cast(self.sse)) != 0
      } else if #[cfg(target_feature="simd128")] {
        i64x2_bitmask(self.simd) != 0
      } else {
        let v : [u64;2] = cast(self);
        ((v[0] | v[1]) & 0x8000000000000000) != 0
      }
    }
  }

  /// true if all high bits are set for every value in the vector
  #[inline]
  #[must_use]
  pub fn all(self) -> bool {
    pick! {
      if #[cfg(target_feature="avx2")] {
        // use f64 move_mask since it is the same size as i64
        move_mask_m128d(cast(self.sse)) == 0b11
      }  else if #[cfg(target_feature="simd128")] {
        i64x2_bitmask(self.simd) == 0b11
      } else {
        let v : [u64;2] = cast(self);
        ((v[0] & v[1]) & 0x8000000000000000) == 0x8000000000000000
      }
    }
  }

  /// true if no high bits are set for any values of the vector
  #[inline]
  #[must_use]
  pub fn none(self) -> bool {
    !self.any()
  }

  #[inline]
  pub fn to_array(self) -> [i64; 2] {
    cast(self)
  }

  #[inline]
  pub fn as_array_ref(&self) -> &[i64; 2] {
    cast_ref(self)
  }

  #[inline]
  pub fn as_array_mut(&mut self) -> &mut [i64; 2] {
    cast_mut(self)
  }
}
