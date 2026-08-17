use super::*;

pick! {
  if #[cfg(target_feature="sse2")] {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(16))]
    pub struct i8x16 { pub(crate) sse: m128i }
  } else if #[cfg(target_feature="simd128")] {
    use core::arch::wasm32::*;

    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct i8x16 { pub(crate) simd: v128 }

    impl Default for i8x16 {
      fn default() -> Self {
        Self::splat(0)
      }
    }

    impl PartialEq for i8x16 {
      fn eq(&self, other: &Self) -> bool {
        u8x16_all_true(i8x16_eq(self.simd, other.simd))
      }
    }

    impl Eq for i8x16 { }
  } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
    use core::arch::aarch64::*;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct i8x16 { pub(crate) neon : int8x16_t }

    impl Default for i8x16 {
      #[inline]
      #[must_use]
      fn default() -> Self {
        Self::splat(0)
      }
    }

    impl PartialEq for i8x16 {
      #[inline]
      #[must_use]
      fn eq(&self, other: &Self) -> bool {
        unsafe { vminvq_u8(vceqq_s8(self.neon, other.neon))==u8::MAX }
      }
    }

    impl Eq for i8x16 { }
  } else {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(16))]
    pub struct i8x16 { arr: [i8;16] }
  }
}

int_uint_consts!(i8, 16, i8x16, 128);

unsafe impl Zeroable for i8x16 {}
unsafe impl Pod for i8x16 {}

impl Add for i8x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: add_i8_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i8x16_add(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vaddq_s8(self.neon, rhs.neon) } }
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

impl Sub for i8x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: sub_i8_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i8x16_sub(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vsubq_s8(self.neon, rhs.neon) }}
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

impl Add<i8> for i8x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: i8) -> Self::Output {
    self.add(Self::splat(rhs))
  }
}

impl Sub<i8> for i8x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: i8) -> Self::Output {
    self.sub(Self::splat(rhs))
  }
}

impl Add<i8x16> for i8 {
  type Output = i8x16;
  #[inline]
  #[must_use]
  fn add(self, rhs: i8x16) -> Self::Output {
    i8x16::splat(self).add(rhs)
  }
}

impl Sub<i8x16> for i8 {
  type Output = i8x16;
  #[inline]
  #[must_use]
  fn sub(self, rhs: i8x16) -> Self::Output {
    i8x16::splat(self).sub(rhs)
  }
}

impl BitAnd for i8x16 {
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
        unsafe {Self { neon: vandq_s8(self.neon, rhs.neon) }}
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

impl BitOr for i8x16 {
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
        unsafe {Self { neon: vorrq_s8(self.neon, rhs.neon) }}
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

impl BitXor for i8x16 {
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
        unsafe {Self { neon: veorq_s8(self.neon, rhs.neon) }}
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

impl CmpEq for i8x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_eq(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_eq_mask_i8_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i8x16_eq(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_s8_u8(vceqq_s8(self.neon, rhs.neon)) }}
      } else {
        Self { arr: [
          if self.arr[0] == rhs.arr[0] { -1 } else { 0 },
          if self.arr[1] == rhs.arr[1] { -1 } else { 0 },
          if self.arr[2] == rhs.arr[2] { -1 } else { 0 },
          if self.arr[3] == rhs.arr[3] { -1 } else { 0 },
          if self.arr[4] == rhs.arr[4] { -1 } else { 0 },
          if self.arr[5] == rhs.arr[5] { -1 } else { 0 },
          if self.arr[6] == rhs.arr[6] { -1 } else { 0 },
          if self.arr[7] == rhs.arr[7] { -1 } else { 0 },
          if self.arr[8] == rhs.arr[8] { -1 } else { 0 },
          if self.arr[9] == rhs.arr[9] { -1 } else { 0 },
          if self.arr[10] == rhs.arr[10] { -1 } else { 0 },
          if self.arr[11] == rhs.arr[11] { -1 } else { 0 },
          if self.arr[12] == rhs.arr[12] { -1 } else { 0 },
          if self.arr[13] == rhs.arr[13] { -1 } else { 0 },
          if self.arr[14] == rhs.arr[14] { -1 } else { 0 },
          if self.arr[15] == rhs.arr[15] { -1 } else { 0 },
        ]}
      }
    }
  }
}

impl CmpGt for i8x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_gt(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_gt_mask_i8_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i8x16_gt(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_s8_u8(vcgtq_s8(self.neon, rhs.neon)) }}
      } else {
        Self { arr: [
          if self.arr[0] > rhs.arr[0] { -1 } else { 0 },
          if self.arr[1] > rhs.arr[1] { -1 } else { 0 },
          if self.arr[2] > rhs.arr[2] { -1 } else { 0 },
          if self.arr[3] > rhs.arr[3] { -1 } else { 0 },
          if self.arr[4] > rhs.arr[4] { -1 } else { 0 },
          if self.arr[5] > rhs.arr[5] { -1 } else { 0 },
          if self.arr[6] > rhs.arr[6] { -1 } else { 0 },
          if self.arr[7] > rhs.arr[7] { -1 } else { 0 },
          if self.arr[8] > rhs.arr[8] { -1 } else { 0 },
          if self.arr[9] > rhs.arr[9] { -1 } else { 0 },
          if self.arr[10] > rhs.arr[10] { -1 } else { 0 },
          if self.arr[11] > rhs.arr[11] { -1 } else { 0 },
          if self.arr[12] > rhs.arr[12] { -1 } else { 0 },
          if self.arr[13] > rhs.arr[13] { -1 } else { 0 },
          if self.arr[14] > rhs.arr[14] { -1 } else { 0 },
          if self.arr[15] > rhs.arr[15] { -1 } else { 0 },
        ]}
      }
    }
  }
}

impl CmpLt for i8x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_lt(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_lt_mask_i8_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i8x16_lt(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_s8_u8(vcltq_s8(self.neon, rhs.neon)) }}
      } else {
        Self { arr: [
          if self.arr[0] < rhs.arr[0] { -1 } else { 0 },
          if self.arr[1] < rhs.arr[1] { -1 } else { 0 },
          if self.arr[2] < rhs.arr[2] { -1 } else { 0 },
          if self.arr[3] < rhs.arr[3] { -1 } else { 0 },
          if self.arr[4] < rhs.arr[4] { -1 } else { 0 },
          if self.arr[5] < rhs.arr[5] { -1 } else { 0 },
          if self.arr[6] < rhs.arr[6] { -1 } else { 0 },
          if self.arr[7] < rhs.arr[7] { -1 } else { 0 },
          if self.arr[8] < rhs.arr[8] { -1 } else { 0 },
          if self.arr[9] < rhs.arr[9] { -1 } else { 0 },
          if self.arr[10] < rhs.arr[10] { -1 } else { 0 },
          if self.arr[11] < rhs.arr[11] { -1 } else { 0 },
          if self.arr[12] < rhs.arr[12] { -1 } else { 0 },
          if self.arr[13] < rhs.arr[13] { -1 } else { 0 },
          if self.arr[14] < rhs.arr[14] { -1 } else { 0 },
          if self.arr[15] < rhs.arr[15] { -1 } else { 0 },
        ]}
      }
    }
  }
}

impl i8x16 {
  #[inline]
  #[must_use]
  pub const fn new(array: [i8; 16]) -> Self {
    unsafe { core::mem::transmute(array) }
  }

  /// converts `i16` to `i8`, saturating values that are too large
  #[inline]
  #[must_use]
  pub fn from_i16x16_saturate(v: i16x16) -> i8x16 {
    pick! {
      if #[cfg(target_feature="avx2")] {
        i8x16 { sse: pack_i16_to_i8_m128i( extract_m128i_from_m256i::<0>(v.avx2), extract_m128i_from_m256i::<1>(v.avx2))  }
      } else if #[cfg(target_feature="sse2")] {
        i8x16 { sse: pack_i16_to_i8_m128i( v.a.sse, v.b.sse ) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
        use core::arch::aarch64::*;

        unsafe {
          i8x16 { neon: vcombine_s8(vqmovn_s16(v.a.neon), vqmovn_s16(v.b.neon)) }
        }
      } else if #[cfg(target_feature="simd128")] {
        use core::arch::wasm32::*;

        i8x16 { simd: i8x16_narrow_i16x8(v.a.simd, v.b.simd) }
      } else {
        fn clamp(a : i16) -> i8 {
            if a < i8::MIN as i16 {
              i8::MIN
            }
            else if a > i8::MAX as i16 {
              i8::MAX
            } else {
                a as i8
            }
        }

        i8x16::new([
          clamp(v.as_array_ref()[0]),
          clamp(v.as_array_ref()[1]),
          clamp(v.as_array_ref()[2]),
          clamp(v.as_array_ref()[3]),
          clamp(v.as_array_ref()[4]),
          clamp(v.as_array_ref()[5]),
          clamp(v.as_array_ref()[6]),
          clamp(v.as_array_ref()[7]),
          clamp(v.as_array_ref()[8]),
          clamp(v.as_array_ref()[9]),
          clamp(v.as_array_ref()[10]),
          clamp(v.as_array_ref()[11]),
          clamp(v.as_array_ref()[12]),
          clamp(v.as_array_ref()[13]),
          clamp(v.as_array_ref()[14]),
          clamp(v.as_array_ref()[15]),
        ])
      }
    }
  }

  /// converts `i16` to `i8`, truncating the upper bits if they are set
  #[inline]
  #[must_use]
  pub fn from_i16x16_truncate(v: i16x16) -> i8x16 {
    pick! {
      if #[cfg(target_feature="avx2")] {
        let a = v.avx2.bitand(set_splat_i16_m256i(0xff));
        i8x16 { sse: pack_i16_to_u8_m128i( extract_m128i_from_m256i::<0>(a), extract_m128i_from_m256i::<1>(a))  }
      } else if #[cfg(target_feature="sse2")] {
        let mask = set_splat_i16_m128i(0xff);
        i8x16 { sse: pack_i16_to_u8_m128i( v.a.sse.bitand(mask), v.b.sse.bitand(mask) ) }
      } else {
        // no super good intrinsics on other platforms... plain old codegen does a reasonable job
        i8x16::new([
          v.as_array_ref()[0] as i8,
          v.as_array_ref()[1] as i8,
          v.as_array_ref()[2] as i8,
          v.as_array_ref()[3] as i8,
          v.as_array_ref()[4] as i8,
          v.as_array_ref()[5] as i8,
          v.as_array_ref()[6] as i8,
          v.as_array_ref()[7] as i8,
          v.as_array_ref()[8] as i8,
          v.as_array_ref()[9] as i8,
          v.as_array_ref()[10] as i8,
          v.as_array_ref()[11] as i8,
          v.as_array_ref()[12] as i8,
          v.as_array_ref()[13] as i8,
          v.as_array_ref()[14] as i8,
          v.as_array_ref()[15] as i8,
        ])
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
        unsafe {Self { neon: vbslq_s8(vreinterpretq_u8_s8(self.neon), t.neon, f.neon) }}
      } else {
        generic_bit_blend(self, t, f)
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn abs(self) -> Self {
    pick! {
      if #[cfg(target_feature="ssse3")] {
        Self { sse: abs_i8_m128i(self.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i8x16_abs(self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vabsq_s8(self.neon) }}
      } else {
        let arr: [i8; 16] = cast(self);
        cast([
          arr[0].wrapping_abs(),
          arr[1].wrapping_abs(),
          arr[2].wrapping_abs(),
          arr[3].wrapping_abs(),
          arr[4].wrapping_abs(),
          arr[5].wrapping_abs(),
          arr[6].wrapping_abs(),
          arr[7].wrapping_abs(),
          arr[8].wrapping_abs(),
          arr[9].wrapping_abs(),
          arr[10].wrapping_abs(),
          arr[11].wrapping_abs(),
          arr[12].wrapping_abs(),
          arr[13].wrapping_abs(),
          arr[14].wrapping_abs(),
          arr[15].wrapping_abs(),
        ])
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn unsigned_abs(self) -> u8x16 {
    pick! {
      if #[cfg(target_feature="ssse3")] {
        u8x16 { sse: abs_i8_m128i(self.sse) }
      } else if #[cfg(target_feature="simd128")] {
        u8x16 { simd: i8x16_abs(self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { u8x16 { neon: vreinterpretq_u8_s8(vabsq_s8(self.neon)) }}
      } else {
        let arr: [i8; 16] = cast(self);
        cast(
          [
            arr[0].unsigned_abs(),
            arr[1].unsigned_abs(),
            arr[2].unsigned_abs(),
            arr[3].unsigned_abs(),
            arr[4].unsigned_abs(),
            arr[5].unsigned_abs(),
            arr[6].unsigned_abs(),
            arr[7].unsigned_abs(),
            arr[8].unsigned_abs(),
            arr[9].unsigned_abs(),
            arr[10].unsigned_abs(),
            arr[11].unsigned_abs(),
            arr[12].unsigned_abs(),
            arr[13].unsigned_abs(),
            arr[14].unsigned_abs(),
            arr[15].unsigned_abs(),
            ])
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn max(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse4.1")] {
        Self { sse: max_i8_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i8x16_max(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vmaxq_s8(self.neon, rhs.neon) }}
      } else {
        self.cmp_lt(rhs).blend(rhs, self)
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn min(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse4.1")] {
        Self { sse: min_i8_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i8x16_min(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vminq_s8(self.neon, rhs.neon) }}
      } else {
        self.cmp_lt(rhs).blend(self, rhs)
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn from_slice_unaligned(input: &[i8]) -> Self {
    assert!(input.len() >= 16);

    pick! {
      if #[cfg(target_feature="sse2")] {
        unsafe { Self { sse: load_unaligned_m128i( &*(input.as_ptr() as * const [u8;16]) ) } }
      } else if #[cfg(target_feature="simd128")] {
        unsafe { Self { simd: v128_load(input.as_ptr() as *const v128 ) } }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vld1q_s8( input.as_ptr() as *const i8 ) } }
      } else {
        // 2018 edition doesn't have try_into
        unsafe { Self::new( *(input.as_ptr() as * const [i8;16]) ) }
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn move_mask(self) -> i32 {
    pick! {
      if #[cfg(target_feature="sse2")] {
        move_mask_i8_m128i(self.sse)
      } else if #[cfg(target_feature="simd128")] {
        i8x16_bitmask(self.simd) as i32
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe
        {
          // set all to 1 if top bit is set, else 0
          let masked = vcltq_s8(self.neon, vdupq_n_s8(0));

          // select the right bit out of each lane
          let selectbit : uint8x16_t = core::mem::transmute([1u8, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128]);
          let out = vandq_u8(masked, selectbit);

          // interleave the lanes so that a 16-bit sum accumulates the bits in the right order
          let table : uint8x16_t = core::mem::transmute([0u8, 8, 1, 9, 2, 10, 3, 11, 4, 12, 5, 13, 6, 14, 7, 15]);
          let r = vqtbl1q_u8(out, table);

          // horizontally add the 16-bit lanes
          vaddvq_u16(vreinterpretq_u16_u8(r)) as i32
        }
       } else {
        ((self.arr[0] < 0) as i32) << 0 |
        ((self.arr[1] < 0) as i32) << 1 |
        ((self.arr[2] < 0) as i32) << 2 |
        ((self.arr[3] < 0) as i32) << 3 |
        ((self.arr[4] < 0) as i32) << 4 |
        ((self.arr[5] < 0) as i32) << 5 |
        ((self.arr[6] < 0) as i32) << 6 |
        ((self.arr[7] < 0) as i32) << 7 |
        ((self.arr[8] < 0) as i32) << 8 |
        ((self.arr[9] < 0) as i32) << 9 |
        ((self.arr[10] < 0) as i32) << 10 |
        ((self.arr[11] < 0) as i32) << 11 |
        ((self.arr[12] < 0) as i32) << 12 |
        ((self.arr[13] < 0) as i32) << 13 |
        ((self.arr[14] < 0) as i32) << 14 |
        ((self.arr[15] < 0) as i32) << 15
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn any(self) -> bool {
    pick! {
      if #[cfg(target_feature="sse2")] {
        move_mask_i8_m128i(self.sse) != 0
      } else if #[cfg(target_feature="simd128")] {
        u8x16_bitmask(self.simd) != 0
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
        unsafe {
          vminvq_s8(self.neon) < 0
        }
      } else {
        let v : [u64;2] = cast(self);
        ((v[0] | v[1]) & 0x80808080808080) != 0
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn all(self) -> bool {
    pick! {
      if #[cfg(target_feature="sse2")] {
        move_mask_i8_m128i(self.sse) == 0b1111_1111_1111_1111
      } else if #[cfg(target_feature="simd128")] {
        u8x16_bitmask(self.simd) == 0b1111_1111_1111_1111
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
        unsafe {
          vmaxvq_s8(self.neon) < 0
        }
      } else {
        let v : [u64;2] = cast(self);
        (v[0] & v[1] & 0x80808080808080) == 0x80808080808080
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
    pick! {
      if #[cfg(target_feature="ssse3")] {
        Self { sse: shuffle_av_i8z_all_m128i(self.sse, add_saturating_u8_m128i(rhs.sse, set_splat_i8_m128i(0x70))) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i8x16_swizzle(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
        unsafe { Self { neon: vqtbl1q_s8(self.neon, vreinterpretq_u8_s8(rhs.neon)) } }
      } else {
        let idxs = rhs.to_array();
        let arr = self.to_array();
        let mut out = [0i8;16];
        for i in 0..16 {
          let idx = idxs[i] as usize;
          if idx >= 16 {
            out[i] = 0;
          } else {
            out[i] = arr[idx];
          }
        }
        Self::new(out)
      }
    }
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
  pub fn swizzle_relaxed(self, rhs: i8x16) -> i8x16 {
    pick! {
      if #[cfg(target_feature="ssse3")] {
        Self { sse: shuffle_av_i8z_all_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i8x16_swizzle(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
        unsafe { Self { neon: vqtbl1q_s8(self.neon, vreinterpretq_u8_s8(rhs.neon)) } }
      } else {
        let idxs = rhs.to_array();
        let arr = self.to_array();
        let mut out = [0i8;16];
        for i in 0..16 {
          let idx = idxs[i] as usize;
          if idx >= 16 {
            out[i] = 0;
          } else {
            out[i] = arr[idx];
          }
        }
        Self::new(out)
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn none(self) -> bool {
    !self.any()
  }

  #[inline]
  #[must_use]
  pub fn saturating_add(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: add_saturating_i8_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i8x16_add_sat(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vqaddq_s8(self.neon, rhs.neon) }}
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
        Self { sse: sub_saturating_i8_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i8x16_sub_sat(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vqsubq_s8(self.neon, rhs.neon) } }
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

  #[inline]
  pub fn to_array(self) -> [i8; 16] {
    cast(self)
  }

  #[inline]
  pub fn as_array_ref(&self) -> &[i8; 16] {
    cast_ref(self)
  }

  #[inline]
  pub fn as_array_mut(&mut self) -> &mut [i8; 16] {
    cast_mut(self)
  }
}
