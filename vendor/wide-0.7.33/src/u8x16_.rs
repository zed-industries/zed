use super::*;

pick! {
  if #[cfg(target_feature="sse2")] {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(16))]
    pub struct u8x16 { pub(crate) sse: m128i }
  } else if #[cfg(target_feature="simd128")] {
    use core::arch::wasm32::*;

    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct u8x16 { pub(crate) simd: v128 }

    impl Default for u8x16 {
      fn default() -> Self {
        Self::splat(0)
      }
    }

    impl PartialEq for u8x16 {
      fn eq(&self, other: &Self) -> bool {
        u8x16_all_true(u8x16_eq(self.simd, other.simd))
      }
    }

    impl Eq for u8x16 { }
  } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
    use core::arch::aarch64::*;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct u8x16 { pub(crate) neon : uint8x16_t }

    impl Default for u8x16 {
      #[inline]
      #[must_use]
      fn default() -> Self {
        Self::splat(0)
      }
    }

    impl PartialEq for u8x16 {
      #[inline]
      #[must_use]
      fn eq(&self, other: &Self) -> bool {
        unsafe { vminvq_u8(vceqq_u8(self.neon, other.neon))==u8::MAX }
      }
    }

    impl Eq for u8x16 { }
  } else {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(16))]
    pub struct u8x16 { pub(crate) arr: [u8;16] }
  }
}

int_uint_consts!(u8, 16, u8x16, 128);

unsafe impl Zeroable for u8x16 {}
unsafe impl Pod for u8x16 {}

impl Add for u8x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: add_i8_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u8x16_add(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vaddq_u8(self.neon, rhs.neon) } }
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
          self.arr[8].wrapping_add(rhs.arr[8]),
          self.arr[9].wrapping_add(rhs.arr[9]),
          self.arr[10].wrapping_add(rhs.arr[10]),
          self.arr[11].wrapping_add(rhs.arr[11]),
          self.arr[12].wrapping_add(rhs.arr[12]),
          self.arr[13].wrapping_add(rhs.arr[13]),
          self.arr[14].wrapping_add(rhs.arr[14]),
          self.arr[15].wrapping_add(rhs.arr[15]),
        ]}
      }
    }
  }
}

impl Sub for u8x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: sub_i8_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u8x16_sub(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vsubq_u8(self.neon, rhs.neon) }}
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
          self.arr[8].wrapping_sub(rhs.arr[8]),
          self.arr[9].wrapping_sub(rhs.arr[9]),
          self.arr[10].wrapping_sub(rhs.arr[10]),
          self.arr[11].wrapping_sub(rhs.arr[11]),
          self.arr[12].wrapping_sub(rhs.arr[12]),
          self.arr[13].wrapping_sub(rhs.arr[13]),
          self.arr[14].wrapping_sub(rhs.arr[14]),
          self.arr[15].wrapping_sub(rhs.arr[15]),
        ]}
      }
    }
  }
}

impl Add<u8> for u8x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: u8) -> Self::Output {
    self.add(Self::splat(rhs))
  }
}

impl Sub<u8> for u8x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: u8) -> Self::Output {
    self.sub(Self::splat(rhs))
  }
}

impl Add<u8x16> for u8 {
  type Output = u8x16;
  #[inline]
  #[must_use]
  fn add(self, rhs: u8x16) -> Self::Output {
    u8x16::splat(self).add(rhs)
  }
}

impl Sub<u8x16> for u8 {
  type Output = u8x16;
  #[inline]
  #[must_use]
  fn sub(self, rhs: u8x16) -> Self::Output {
    u8x16::splat(self).sub(rhs)
  }
}

impl BitAnd for u8x16 {
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
        unsafe {Self { neon: vandq_u8(self.neon, rhs.neon) }}
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
          self.arr[8].bitand(rhs.arr[8]),
          self.arr[9].bitand(rhs.arr[9]),
          self.arr[10].bitand(rhs.arr[10]),
          self.arr[11].bitand(rhs.arr[11]),
          self.arr[12].bitand(rhs.arr[12]),
          self.arr[13].bitand(rhs.arr[13]),
          self.arr[14].bitand(rhs.arr[14]),
          self.arr[15].bitand(rhs.arr[15]),
        ]}
      }
    }
  }
}

impl BitOr for u8x16 {
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
        unsafe {Self { neon: vorrq_u8(self.neon, rhs.neon) }}
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
          self.arr[8].bitor(rhs.arr[8]),
          self.arr[9].bitor(rhs.arr[9]),
          self.arr[10].bitor(rhs.arr[10]),
          self.arr[11].bitor(rhs.arr[11]),
          self.arr[12].bitor(rhs.arr[12]),
          self.arr[13].bitor(rhs.arr[13]),
          self.arr[14].bitor(rhs.arr[14]),
          self.arr[15].bitor(rhs.arr[15]),
        ]}
      }
    }
  }
}

impl BitXor for u8x16 {
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
        unsafe {Self { neon: veorq_u8(self.neon, rhs.neon) }}
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
          self.arr[8].bitxor(rhs.arr[8]),
          self.arr[9].bitxor(rhs.arr[9]),
          self.arr[10].bitxor(rhs.arr[10]),
          self.arr[11].bitxor(rhs.arr[11]),
          self.arr[12].bitxor(rhs.arr[12]),
          self.arr[13].bitxor(rhs.arr[13]),
          self.arr[14].bitxor(rhs.arr[14]),
          self.arr[15].bitxor(rhs.arr[15]),
        ]}
      }
    }
  }
}

impl CmpEq for u8x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_eq(self, rhs: Self) -> Self::Output {
    Self::cmp_eq(self, rhs)
  }
}

impl u8x16 {
  #[inline]
  #[must_use]
  pub const fn new(array: [u8; 16]) -> Self {
    unsafe { core::mem::transmute(array) }
  }
  #[inline]
  #[must_use]
  pub fn cmp_eq(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_eq_mask_i8_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u8x16_eq(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vceqq_u8(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          if self.arr[0] == rhs.arr[0] { u8::MAX } else { 0 },
          if self.arr[1] == rhs.arr[1] { u8::MAX } else { 0 },
          if self.arr[2] == rhs.arr[2] { u8::MAX } else { 0 },
          if self.arr[3] == rhs.arr[3] { u8::MAX } else { 0 },
          if self.arr[4] == rhs.arr[4] { u8::MAX } else { 0 },
          if self.arr[5] == rhs.arr[5] { u8::MAX } else { 0 },
          if self.arr[6] == rhs.arr[6] { u8::MAX } else { 0 },
          if self.arr[7] == rhs.arr[7] { u8::MAX } else { 0 },
          if self.arr[8] == rhs.arr[8] { u8::MAX } else { 0 },
          if self.arr[9] == rhs.arr[9] { u8::MAX } else { 0 },
          if self.arr[10] == rhs.arr[10] { u8::MAX } else { 0 },
          if self.arr[11] == rhs.arr[11] { u8::MAX } else { 0 },
          if self.arr[12] == rhs.arr[12] { u8::MAX } else { 0 },
          if self.arr[13] == rhs.arr[13] { u8::MAX } else { 0 },
          if self.arr[14] == rhs.arr[14] { u8::MAX } else { 0 },
          if self.arr[15] == rhs.arr[15] { u8::MAX } else { 0 },
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
        unsafe {Self { neon: vbslq_u8(self.neon, t.neon, f.neon) }}
      } else {
        generic_bit_blend(self, t, f)
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn max(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: max_u8_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u8x16_max(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vmaxq_u8(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].max(rhs.arr[0]),
          self.arr[1].max(rhs.arr[1]),
          self.arr[2].max(rhs.arr[2]),
          self.arr[3].max(rhs.arr[3]),
          self.arr[4].max(rhs.arr[4]),
          self.arr[5].max(rhs.arr[5]),
          self.arr[6].max(rhs.arr[6]),
          self.arr[7].max(rhs.arr[7]),
          self.arr[8].max(rhs.arr[8]),
          self.arr[9].max(rhs.arr[9]),
          self.arr[10].max(rhs.arr[10]),
          self.arr[11].max(rhs.arr[11]),
          self.arr[12].max(rhs.arr[12]),
          self.arr[13].max(rhs.arr[13]),
          self.arr[14].max(rhs.arr[14]),
          self.arr[15].max(rhs.arr[15]),
        ]}
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn min(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: min_u8_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u8x16_min(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vminq_u8(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].min(rhs.arr[0]),
          self.arr[1].min(rhs.arr[1]),
          self.arr[2].min(rhs.arr[2]),
          self.arr[3].min(rhs.arr[3]),
          self.arr[4].min(rhs.arr[4]),
          self.arr[5].min(rhs.arr[5]),
          self.arr[6].min(rhs.arr[6]),
          self.arr[7].min(rhs.arr[7]),
          self.arr[8].min(rhs.arr[8]),
          self.arr[9].min(rhs.arr[9]),
          self.arr[10].min(rhs.arr[10]),
          self.arr[11].min(rhs.arr[11]),
          self.arr[12].min(rhs.arr[12]),
          self.arr[13].min(rhs.arr[13]),
          self.arr[14].min(rhs.arr[14]),
          self.arr[15].min(rhs.arr[15]),
        ]}
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn saturating_add(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: add_saturating_u8_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u8x16_add_sat(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vqaddq_u8(self.neon, rhs.neon) }}
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
          self.arr[8].saturating_add(rhs.arr[8]),
          self.arr[9].saturating_add(rhs.arr[9]),
          self.arr[10].saturating_add(rhs.arr[10]),
          self.arr[11].saturating_add(rhs.arr[11]),
          self.arr[12].saturating_add(rhs.arr[12]),
          self.arr[13].saturating_add(rhs.arr[13]),
          self.arr[14].saturating_add(rhs.arr[14]),
          self.arr[15].saturating_add(rhs.arr[15]),
        ]}
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn saturating_sub(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: sub_saturating_u8_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u8x16_sub_sat(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vqsubq_u8(self.neon, rhs.neon) } }
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
          self.arr[8].saturating_sub(rhs.arr[8]),
          self.arr[9].saturating_sub(rhs.arr[9]),
          self.arr[10].saturating_sub(rhs.arr[10]),
          self.arr[11].saturating_sub(rhs.arr[11]),
          self.arr[12].saturating_sub(rhs.arr[12]),
          self.arr[13].saturating_sub(rhs.arr[13]),
          self.arr[14].saturating_sub(rhs.arr[14]),
          self.arr[15].saturating_sub(rhs.arr[15]),
        ]}
      }
    }
  }

  /// Unpack and interleave low lanes of two `u8x16`
  #[inline]
  #[must_use]
  pub fn unpack_low(lhs: u8x16, rhs: u8x16) -> u8x16 {
    pick! {
        if #[cfg(target_feature = "sse2")] {
            u8x16 { sse: unpack_low_i8_m128i(lhs.sse, rhs.sse) }
        } else if #[cfg(target_feature = "simd128")] {
          u8x16 { simd: u8x16_shuffle::<0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23>(lhs.simd, rhs.simd) }
        } else if #[cfg(all(target_feature = "neon", target_arch = "aarch64"))] {
            let lhs = unsafe { vget_low_u8(lhs.neon) };
            let rhs = unsafe { vget_low_u8(rhs.neon) };

            let zipped = unsafe { vzip_u8(lhs, rhs) };
            u8x16 { neon: unsafe { vcombine_u8(zipped.0, zipped.1) } }
        } else {
            u8x16::new([
                lhs.as_array_ref()[0], rhs.as_array_ref()[0],
                lhs.as_array_ref()[1], rhs.as_array_ref()[1],
                lhs.as_array_ref()[2], rhs.as_array_ref()[2],
                lhs.as_array_ref()[3], rhs.as_array_ref()[3],
                lhs.as_array_ref()[4], rhs.as_array_ref()[4],
                lhs.as_array_ref()[5], rhs.as_array_ref()[5],
                lhs.as_array_ref()[6], rhs.as_array_ref()[6],
                lhs.as_array_ref()[7], rhs.as_array_ref()[7],
            ])
        }
    }
  }

  /// Unpack and interleave high lanes of two `u8x16`
  #[inline]
  #[must_use]
  pub fn unpack_high(lhs: u8x16, rhs: u8x16) -> u8x16 {
    pick! {
        if #[cfg(target_feature = "sse2")] {
            u8x16 { sse: unpack_high_i8_m128i(lhs.sse, rhs.sse) }
        } else if #[cfg(target_feature = "simd128")] {
            u8x16 { simd: u8x16_shuffle::<8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31>(lhs.simd, rhs.simd) }
        } else if #[cfg(all(target_feature = "neon", target_arch = "aarch64"))] {
            let lhs = unsafe { vget_high_u8(lhs.neon) };
            let rhs = unsafe { vget_high_u8(rhs.neon) };

            let zipped = unsafe { vzip_u8(lhs, rhs) };
            u8x16 { neon: unsafe { vcombine_u8(zipped.0, zipped.1) } }
        } else {
            u8x16::new([
                lhs.as_array_ref()[8], rhs.as_array_ref()[8],
                lhs.as_array_ref()[9], rhs.as_array_ref()[9],
                lhs.as_array_ref()[10], rhs.as_array_ref()[10],
                lhs.as_array_ref()[11], rhs.as_array_ref()[11],
                lhs.as_array_ref()[12], rhs.as_array_ref()[12],
                lhs.as_array_ref()[13], rhs.as_array_ref()[13],
                lhs.as_array_ref()[14], rhs.as_array_ref()[14],
                lhs.as_array_ref()[15], rhs.as_array_ref()[15],
            ])
        }
    }
  }

  /// Pack and saturate two `i16x8` to `u8x16`
  #[inline]
  #[must_use]
  pub fn narrow_i16x8(lhs: i16x8, rhs: i16x8) -> Self {
    pick! {
        if #[cfg(target_feature = "sse2")] {
            u8x16 { sse: pack_i16_to_u8_m128i(lhs.sse, rhs.sse) }
        } else if #[cfg(target_feature = "simd128")] {
            u8x16 { simd: u8x16_narrow_i16x8(lhs.simd, rhs.simd) }
        } else if #[cfg(all(target_feature = "neon", target_arch = "aarch64"))] {
            let lhs = unsafe { vqmovun_s16(lhs.neon) };
            let rhs = unsafe { vqmovun_s16(rhs.neon) };
            u8x16 { neon: unsafe { vcombine_u8(lhs, rhs) } }
        } else {
            fn clamp(a: i16) -> u8 {
                  if a < u8::MIN as i16 {
                      u8::MIN
                  } else if a > u8::MAX as i16 {
                      u8::MAX
                  } else {
                      a as u8
                  }
            }

            Self { arr: [
                clamp(lhs.as_array_ref()[0]),
                clamp(lhs.as_array_ref()[1]),
                clamp(lhs.as_array_ref()[2]),
                clamp(lhs.as_array_ref()[3]),
                clamp(lhs.as_array_ref()[4]),
                clamp(lhs.as_array_ref()[5]),
                clamp(lhs.as_array_ref()[6]),
                clamp(lhs.as_array_ref()[7]),
                clamp(rhs.as_array_ref()[0]),
                clamp(rhs.as_array_ref()[1]),
                clamp(rhs.as_array_ref()[2]),
                clamp(rhs.as_array_ref()[3]),
                clamp(rhs.as_array_ref()[4]),
                clamp(rhs.as_array_ref()[5]),
                clamp(rhs.as_array_ref()[6]),
                clamp(rhs.as_array_ref()[7]),
            ]}
        }
    }
  }

  /// Returns a new vector where each element is based on the index values in
  /// `rhs`.
  ///
  /// * Index values in the range `[0, 15]` select the i-th element of `self`.
  /// * Index values that are out of range will cause that output lane to be
  ///   `0`.
  #[inline]
  pub fn swizzle(self, rhs: i8x16) -> i8x16 {
    cast(i8x16::swizzle(cast(self), rhs))
  }

  /// Works like [`swizzle`](Self::swizzle) with the following additional
  /// details
  ///
  /// * Indices in the range `[0, 15]` will select the i-th element of `self`.
  /// * If the high bit of any index is set (meaning that the index is
  ///   negative), then the corresponding output lane is guaranteed to be zero.
  /// * Otherwise the output lane is either `0` or `self[rhs[i] % 16]`,
  ///   depending on the implementation.
  #[inline]
  pub fn swizzle_relaxed(self, rhs: u8x16) -> u8x16 {
    cast(i8x16::swizzle_relaxed(cast(self), cast(rhs)))
  }

  #[inline]
  #[must_use]
  pub fn move_mask(self) -> i32 {
    i8x16::move_mask(cast(self))
  }

  #[inline]
  #[must_use]
  pub fn any(self) -> bool {
    i8x16::any(cast(self))
  }

  #[inline]
  #[must_use]
  pub fn all(self) -> bool {
    i8x16::all(cast(self))
  }

  #[inline]
  #[must_use]
  pub fn none(self) -> bool {
    i8x16::none(cast(self))
  }

  #[inline]
  pub fn to_array(self) -> [u8; 16] {
    cast(self)
  }

  #[inline]
  pub fn as_array_ref(&self) -> &[u8; 16] {
    cast_ref(self)
  }

  #[inline]
  pub fn as_array_mut(&mut self) -> &mut [u8; 16] {
    cast_mut(self)
  }
}
