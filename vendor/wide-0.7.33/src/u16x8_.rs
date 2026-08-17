use super::*;

pick! {
  if #[cfg(target_feature="sse2")] {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(16))]
    pub struct u16x8 { pub(crate) sse: m128i }
  } else if #[cfg(target_feature="simd128")] {
    use core::arch::wasm32::*;

    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct u16x8 { pub(crate) simd: v128 }

    impl Default for u16x8 {
      fn default() -> Self {
        Self::splat(0)
      }
    }

    impl PartialEq for u16x8 {
      fn eq(&self, other: &Self) -> bool {
        u16x8_all_true(u16x8_eq(self.simd, other.simd))
      }
    }

    impl Eq for u16x8 { }
  } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
      use core::arch::aarch64::*;
      #[repr(C)]
      #[derive(Copy, Clone)]
      pub struct u16x8 { pub(crate) neon : uint16x8_t }

      impl Default for u16x8 {
        #[inline]
        #[must_use]
        fn default() -> Self {
          Self::splat(0)
        }
      }

      impl PartialEq for u16x8 {
        #[inline]
        #[must_use]
        fn eq(&self, other: &Self) -> bool {
          unsafe { vminvq_u16(vceqq_u16(self.neon, other.neon))==u16::MAX }
        }
      }

      impl Eq for u16x8 { }
  } else {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(16))]
    pub struct u16x8 { pub(crate) arr: [u16;8] }
  }
}

int_uint_consts!(u16, 8, u16x8, 128);

unsafe impl Zeroable for u16x8 {}
unsafe impl Pod for u16x8 {}

impl Add for u16x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: add_i16_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u16x8_add(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vaddq_u16(self.neon, rhs.neon) } }
      } else {
        Self { arr: [
          self.arr[0].wrapping_add(rhs.arr[0]),
          self.arr[1].wrapping_add(rhs.arr[1]),
          self.arr[2].wrapping_add(rhs.arr[2]),
          self.arr[3].wrapping_add(rhs.arr[3]),
          self.arr[4].wrapping_add(rhs.arr[4]),
          self.arr[5].wrapping_add(rhs.arr[5]),
          self.arr[6].wrapping_add(rhs.arr[6]),
          self.arr[7].wrapping_add(rhs.arr[7]),
        ]}
      }
    }
  }
}

impl Sub for u16x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: sub_i16_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u16x8_sub(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vsubq_u16(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].wrapping_sub(rhs.arr[0]),
          self.arr[1].wrapping_sub(rhs.arr[1]),
          self.arr[2].wrapping_sub(rhs.arr[2]),
          self.arr[3].wrapping_sub(rhs.arr[3]),
          self.arr[4].wrapping_sub(rhs.arr[4]),
          self.arr[5].wrapping_sub(rhs.arr[5]),
          self.arr[6].wrapping_sub(rhs.arr[6]),
          self.arr[7].wrapping_sub(rhs.arr[7]),
        ]}
      }
    }
  }
}

impl Mul for u16x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: mul_i16_keep_low_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u16x8_mul(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vmulq_u16(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].wrapping_mul(rhs.arr[0]),
          self.arr[1].wrapping_mul(rhs.arr[1]),
          self.arr[2].wrapping_mul(rhs.arr[2]),
          self.arr[3].wrapping_mul(rhs.arr[3]),
          self.arr[4].wrapping_mul(rhs.arr[4]),
          self.arr[5].wrapping_mul(rhs.arr[5]),
          self.arr[6].wrapping_mul(rhs.arr[6]),
          self.arr[7].wrapping_mul(rhs.arr[7]),
        ]}
      }
    }
  }
}

impl Add<u16> for u16x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: u16) -> Self::Output {
    self.add(Self::splat(rhs))
  }
}

impl Sub<u16> for u16x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: u16) -> Self::Output {
    self.sub(Self::splat(rhs))
  }
}

impl Mul<u16> for u16x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: u16) -> Self::Output {
    self.mul(Self::splat(rhs))
  }
}

impl Add<u16x8> for u16 {
  type Output = u16x8;
  #[inline]
  #[must_use]
  fn add(self, rhs: u16x8) -> Self::Output {
    u16x8::splat(self).add(rhs)
  }
}

impl Sub<u16x8> for u16 {
  type Output = u16x8;
  #[inline]
  #[must_use]
  fn sub(self, rhs: u16x8) -> Self::Output {
    u16x8::splat(self).sub(rhs)
  }
}

impl Mul<u16x8> for u16 {
  type Output = u16x8;
  #[inline]
  #[must_use]
  fn mul(self, rhs: u16x8) -> Self::Output {
    u16x8::splat(self).mul(rhs)
  }
}

impl BitAnd for u16x8 {
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
        unsafe {Self { neon: vandq_u16(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].bitand(rhs.arr[0]),
          self.arr[1].bitand(rhs.arr[1]),
          self.arr[2].bitand(rhs.arr[2]),
          self.arr[3].bitand(rhs.arr[3]),
          self.arr[4].bitand(rhs.arr[4]),
          self.arr[5].bitand(rhs.arr[5]),
          self.arr[6].bitand(rhs.arr[6]),
          self.arr[7].bitand(rhs.arr[7]),
        ]}
      }
    }
  }
}

impl BitOr for u16x8 {
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
        unsafe {Self { neon: vorrq_u16(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].bitor(rhs.arr[0]),
          self.arr[1].bitor(rhs.arr[1]),
          self.arr[2].bitor(rhs.arr[2]),
          self.arr[3].bitor(rhs.arr[3]),
          self.arr[4].bitor(rhs.arr[4]),
          self.arr[5].bitor(rhs.arr[5]),
          self.arr[6].bitor(rhs.arr[6]),
          self.arr[7].bitor(rhs.arr[7]),
        ]}
      }
    }
  }
}

impl BitXor for u16x8 {
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
        unsafe {Self { neon: veorq_u16(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].bitxor(rhs.arr[0]),
          self.arr[1].bitxor(rhs.arr[1]),
          self.arr[2].bitxor(rhs.arr[2]),
          self.arr[3].bitxor(rhs.arr[3]),
          self.arr[4].bitxor(rhs.arr[4]),
          self.arr[5].bitxor(rhs.arr[5]),
          self.arr[6].bitxor(rhs.arr[6]),
          self.arr[7].bitxor(rhs.arr[7]),
        ]}
      }
    }
  }
}

macro_rules! impl_shl_t_for_u16x8 {
  ($($shift_type:ty),+ $(,)?) => {
    $(impl Shl<$shift_type> for u16x8 {
      type Output = Self;
      /// Shifts all lanes by the value given.
      #[inline]
      #[must_use]
      fn shl(self, rhs: $shift_type) -> Self::Output {
        pick! {
          if #[cfg(target_feature="sse2")] {
            let shift = cast([rhs as u64, 0]);
            Self { sse: shl_all_u16_m128i(self.sse, shift) }
          } else if #[cfg(target_feature="simd128")] {
            Self { simd: u16x8_shl(self.simd, rhs as u32) }
          } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
            unsafe {Self { neon: vshlq_u16(self.neon, vmovq_n_s16(rhs as i16)) }}
          } else {
            let u = rhs as u64;
            Self { arr: [
              self.arr[0] << u,
              self.arr[1] << u,
              self.arr[2] << u,
              self.arr[3] << u,
              self.arr[4] << u,
              self.arr[5] << u,
              self.arr[6] << u,
              self.arr[7] << u,
            ]}
          }
        }
      }
    })+
  };
}
impl_shl_t_for_u16x8!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

macro_rules! impl_shr_t_for_u16x8 {
  ($($shift_type:ty),+ $(,)?) => {
    $(impl Shr<$shift_type> for u16x8 {
      type Output = Self;
      /// Shifts all lanes by the value given.
      #[inline]
      #[must_use]
      fn shr(self, rhs: $shift_type) -> Self::Output {
        pick! {
          if #[cfg(target_feature="sse2")] {
            let shift = cast([rhs as u64, 0]);
            Self { sse: shr_all_u16_m128i(self.sse, shift) }
          } else if #[cfg(target_feature="simd128")] {
            Self { simd: u16x8_shr(self.simd, rhs as u32) }
          } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
            unsafe {Self { neon: vshlq_u16(self.neon, vmovq_n_s16( -(rhs as i16))) }}
          } else {
            let u = rhs as u64;
            Self { arr: [
              self.arr[0] >> u,
              self.arr[1] >> u,
              self.arr[2] >> u,
              self.arr[3] >> u,
              self.arr[4] >> u,
              self.arr[5] >> u,
              self.arr[6] >> u,
              self.arr[7] >> u,
            ]}
          }
        }
      }
    })+
  };
}
impl_shr_t_for_u16x8!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

impl CmpEq for u16x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_eq(self, rhs: Self) -> Self::Output {
    Self::cmp_eq(self, rhs)
  }
}

impl u16x8 {
  #[inline]
  #[must_use]
  pub const fn new(array: [u16; 8]) -> Self {
    unsafe { core::mem::transmute(array) }
  }
  #[inline]
  #[must_use]
  pub fn cmp_eq(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_eq_mask_i16_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u16x8_eq(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vceqq_u16(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          if self.arr[0] == rhs.arr[0] { u16::MAX } else { 0 },
          if self.arr[1] == rhs.arr[1] { u16::MAX } else { 0 },
          if self.arr[2] == rhs.arr[2] { u16::MAX } else { 0 },
          if self.arr[3] == rhs.arr[3] { u16::MAX } else { 0 },
          if self.arr[4] == rhs.arr[4] { u16::MAX } else { 0 },
          if self.arr[5] == rhs.arr[5] { u16::MAX } else { 0 },
          if self.arr[6] == rhs.arr[6] { u16::MAX } else { 0 },
          if self.arr[7] == rhs.arr[7] { u16::MAX } else { 0 },
        ]}
      }
    }
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
        unsafe {Self { neon: vbslq_u16(self.neon, t.neon, f.neon) }}
      } else {
        generic_bit_blend(self, t, f)
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn max(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse4.1")] {
        Self { sse: max_u16_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u16x8_max(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vmaxq_u16(self.neon, rhs.neon) }}
      } else {
        let arr: [u16; 8] = cast(self);
        let rhs: [u16; 8] = cast(rhs);
        cast([
          arr[0].max(rhs[0]),
          arr[1].max(rhs[1]),
          arr[2].max(rhs[2]),
          arr[3].max(rhs[3]),
          arr[4].max(rhs[4]),
          arr[5].max(rhs[5]),
          arr[6].max(rhs[6]),
          arr[7].max(rhs[7]),
        ])
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn min(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse4.1")] {
        Self { sse: min_u16_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u16x8_min(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vminq_u16(self.neon, rhs.neon) }}
      } else {
        let arr: [u16; 8] = cast(self);
        let rhs: [u16; 8] = cast(rhs);
        cast([
          arr[0].min(rhs[0]),
          arr[1].min(rhs[1]),
          arr[2].min(rhs[2]),
          arr[3].min(rhs[3]),
          arr[4].min(rhs[4]),
          arr[5].min(rhs[5]),
          arr[6].min(rhs[6]),
          arr[7].min(rhs[7]),
        ])
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn saturating_add(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: add_saturating_u16_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u16x8_add_sat(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vqaddq_u16(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].saturating_add(rhs.arr[0]),
          self.arr[1].saturating_add(rhs.arr[1]),
          self.arr[2].saturating_add(rhs.arr[2]),
          self.arr[3].saturating_add(rhs.arr[3]),
          self.arr[4].saturating_add(rhs.arr[4]),
          self.arr[5].saturating_add(rhs.arr[5]),
          self.arr[6].saturating_add(rhs.arr[6]),
          self.arr[7].saturating_add(rhs.arr[7]),
        ]}
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn saturating_sub(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: sub_saturating_u16_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u16x8_sub_sat(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vqsubq_u16(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].saturating_sub(rhs.arr[0]),
          self.arr[1].saturating_sub(rhs.arr[1]),
          self.arr[2].saturating_sub(rhs.arr[2]),
          self.arr[3].saturating_sub(rhs.arr[3]),
          self.arr[4].saturating_sub(rhs.arr[4]),
          self.arr[5].saturating_sub(rhs.arr[5]),
          self.arr[6].saturating_sub(rhs.arr[6]),
          self.arr[7].saturating_sub(rhs.arr[7]),
        ]}
      }
    }
  }

  /// Unpack the lower half of the input and zero expand it to `u16` values.
  #[inline]
  #[must_use]
  pub fn from_u8x16_low(u: u8x16) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self{ sse: unpack_low_i8_m128i(u.sse, m128i::zeroed()) }
      } else {
        let u_arr: [u8; 16] = cast(u);
        cast([
          u_arr[0] as u16,
          u_arr[1] as u16,
          u_arr[2] as u16,
          u_arr[3] as u16,
          u_arr[4] as u16,
          u_arr[5] as u16,
          u_arr[6] as u16,
          u_arr[7] as u16,
        ])
      }
    }
  }

  /// Unpack the upper half of the input and zero expand it to `u16` values.
  #[inline]
  #[must_use]
  pub fn from_u8x16_high(u: u8x16) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self{ sse: unpack_high_i8_m128i(u.sse, m128i::zeroed()) }
      } else {
        let u_arr: [u8; 16] = cast(u);
        cast([
          u_arr[8] as u16,
          u_arr[9] as u16,
          u_arr[10] as u16,
          u_arr[11] as u16,
          u_arr[12] as u16,
          u_arr[13] as u16,
          u_arr[14] as u16,
          u_arr[15] as u16,
        ])
      }
    }
  }

  /// multiplies two u16x8 and returns the result as a widened u32x8
  #[inline]
  #[must_use]
  pub fn mul_widen(self, rhs: Self) -> u32x8 {
    pick! {
      if #[cfg(target_feature="avx2")] {
        let a = convert_to_i32_m256i_from_u16_m128i(self.sse);
        let b = convert_to_i32_m256i_from_u16_m128i(rhs.sse);
        u32x8 { avx2: mul_i32_keep_low_m256i(a,b) }
      } else if #[cfg(target_feature="sse2")] {
         let low = mul_i16_keep_low_m128i(self.sse, rhs.sse);
         let high = mul_u16_keep_high_m128i(self.sse, rhs.sse);
         u32x8 {
          a: u32x4 { sse:unpack_low_i16_m128i(low, high) },
          b: u32x4 { sse:unpack_high_i16_m128i(low, high) }
        }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
         let lhs_low = unsafe { vget_low_u16(self.neon) };
         let rhs_low = unsafe { vget_low_u16(rhs.neon) };

         let lhs_high = unsafe { vget_high_u16(self.neon) };
         let rhs_high = unsafe { vget_high_u16(rhs.neon) };

         let low = unsafe { vmull_u16(lhs_low, rhs_low) };
         let high = unsafe { vmull_u16(lhs_high, rhs_high) };

         u32x8 { a: u32x4 { neon: low }, b: u32x4 {neon: high } }
       } else {
        let a = self.as_array_ref();
        let b = rhs.as_array_ref();
         u32x8::new([
           u32::from(a[0]) * u32::from(b[0]),
           u32::from(a[1]) * u32::from(b[1]),
           u32::from(a[2]) * u32::from(b[2]),
           u32::from(a[3]) * u32::from(b[3]),
           u32::from(a[4]) * u32::from(b[4]),
           u32::from(a[5]) * u32::from(b[5]),
           u32::from(a[6]) * u32::from(b[6]),
           u32::from(a[7]) * u32::from(b[7]),
         ])
       }
    }
  }

  /// Multiples two `u16x8` and return the high part of intermediate `u32x8`
  #[inline]
  #[must_use]
  pub fn mul_keep_high(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: mul_u16_keep_high_m128i(self.sse, rhs.sse) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
        let lhs_low = unsafe { vget_low_u16(self.neon) };
        let rhs_low = unsafe { vget_low_u16(rhs.neon) };

        let lhs_high = unsafe { vget_high_u16(self.neon) };
        let rhs_high = unsafe { vget_high_u16(rhs.neon) };

        let low = unsafe { vmull_u16(lhs_low, rhs_low) };
        let high = unsafe { vmull_u16(lhs_high, rhs_high) };

        u16x8 { neon: unsafe { vuzpq_u16(vreinterpretq_u16_u32(low), vreinterpretq_u16_u32(high)).1 } }
      } else if #[cfg(target_feature="simd128")] {
        let low =  u32x4_extmul_low_u16x8(self.simd, rhs.simd);
        let high = u32x4_extmul_high_u16x8(self.simd, rhs.simd);

        Self { simd: u16x8_shuffle::<1, 3, 5, 7, 9, 11, 13, 15>(low, high) }
      } else {
        u16x8::new([
          ((u32::from(rhs.as_array_ref()[0]) * u32::from(self.as_array_ref()[0])) >> 16) as u16,
          ((u32::from(rhs.as_array_ref()[1]) * u32::from(self.as_array_ref()[1])) >> 16) as u16,
          ((u32::from(rhs.as_array_ref()[2]) * u32::from(self.as_array_ref()[2])) >> 16) as u16,
          ((u32::from(rhs.as_array_ref()[3]) * u32::from(self.as_array_ref()[3])) >> 16) as u16,
          ((u32::from(rhs.as_array_ref()[4]) * u32::from(self.as_array_ref()[4])) >> 16) as u16,
          ((u32::from(rhs.as_array_ref()[5]) * u32::from(self.as_array_ref()[5])) >> 16) as u16,
          ((u32::from(rhs.as_array_ref()[6]) * u32::from(self.as_array_ref()[6])) >> 16) as u16,
          ((u32::from(rhs.as_array_ref()[7]) * u32::from(self.as_array_ref()[7])) >> 16) as u16,
        ])
      }
    }
  }

  #[inline]
  pub fn to_array(self) -> [u16; 8] {
    cast(self)
  }

  #[inline]
  pub fn as_array_ref(&self) -> &[u16; 8] {
    cast_ref(self)
  }

  #[inline]
  pub fn as_array_mut(&mut self) -> &mut [u16; 8] {
    cast_mut(self)
  }
}
