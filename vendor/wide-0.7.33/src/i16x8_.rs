use super::*;

pick! {
  if #[cfg(target_feature="sse2")] {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(16))]
    pub struct i16x8 { pub(crate) sse: m128i }
  } else if #[cfg(target_feature="simd128")] {
    use core::arch::wasm32::*;

    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct i16x8 { pub(crate) simd: v128 }

    impl Default for i16x8 {
      fn default() -> Self {
        Self::splat(0)
      }
    }

    impl PartialEq for i16x8 {
      fn eq(&self, other: &Self) -> bool {
        u16x8_all_true(i16x8_eq(self.simd, other.simd))
      }
    }

    impl Eq for i16x8 { }
  } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
    use core::arch::aarch64::*;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct i16x8 { pub(crate) neon : int16x8_t }

    impl Default for i16x8 {
      #[inline]
      #[must_use]
      fn default() -> Self {
        Self::splat(0)
      }
    }

    impl PartialEq for i16x8 {
      #[inline]
      #[must_use]
      fn eq(&self, other: &Self) -> bool {
        unsafe { vminvq_u16(vceqq_s16(self.neon, other.neon))==u16::MAX }
      }
    }

    impl Eq for i16x8 { }
  } else {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(16))]
    pub struct i16x8 { pub(crate) arr: [i16;8] }
  }
}

int_uint_consts!(i16, 8, i16x8, 128);

unsafe impl Zeroable for i16x8 {}
unsafe impl Pod for i16x8 {}

impl Add for i16x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: add_i16_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i16x8_add(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vaddq_s16(self.neon, rhs.neon) } }
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

impl Sub for i16x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: sub_i16_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i16x8_sub(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vsubq_s16(self.neon, rhs.neon) }}
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

impl Mul for i16x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: mul_i16_keep_low_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i16x8_mul(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vmulq_s16(self.neon, rhs.neon) }}
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

impl Add<i16> for i16x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: i16) -> Self::Output {
    self.add(Self::splat(rhs))
  }
}

impl Sub<i16> for i16x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: i16) -> Self::Output {
    self.sub(Self::splat(rhs))
  }
}

impl Mul<i16> for i16x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: i16) -> Self::Output {
    self.mul(Self::splat(rhs))
  }
}

impl Add<i16x8> for i16 {
  type Output = i16x8;
  #[inline]
  #[must_use]
  fn add(self, rhs: i16x8) -> Self::Output {
    i16x8::splat(self).add(rhs)
  }
}

impl Sub<i16x8> for i16 {
  type Output = i16x8;
  #[inline]
  #[must_use]
  fn sub(self, rhs: i16x8) -> Self::Output {
    i16x8::splat(self).sub(rhs)
  }
}

impl Mul<i16x8> for i16 {
  type Output = i16x8;
  #[inline]
  #[must_use]
  fn mul(self, rhs: i16x8) -> Self::Output {
    i16x8::splat(self).mul(rhs)
  }
}

impl BitAnd for i16x8 {
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
        unsafe {Self { neon: vandq_s16(self.neon, rhs.neon) }}
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

impl BitOr for i16x8 {
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
        unsafe {Self { neon: vorrq_s16(self.neon, rhs.neon) }}
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

impl BitXor for i16x8 {
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
        unsafe {Self { neon: veorq_s16(self.neon, rhs.neon) }}
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

macro_rules! impl_shl_t_for_i16x8 {
  ($($shift_type:ty),+ $(,)?) => {
    $(impl Shl<$shift_type> for i16x8 {
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
            Self { simd: i16x8_shl(self.simd, rhs as u32) }
          } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
            unsafe {Self { neon: vshlq_s16(self.neon, vmovq_n_s16(rhs as i16)) }}
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
impl_shl_t_for_i16x8!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

macro_rules! impl_shr_t_for_i16x8 {
  ($($shift_type:ty),+ $(,)?) => {
    $(impl Shr<$shift_type> for i16x8 {
      type Output = Self;
      /// Shifts all lanes by the value given.
      #[inline]
      #[must_use]
      fn shr(self, rhs: $shift_type) -> Self::Output {
        pick! {
          if #[cfg(target_feature="sse2")] {
            let shift = cast([rhs as u64, 0]);
            Self { sse: shr_all_i16_m128i(self.sse, shift) }
          } else if #[cfg(target_feature="simd128")] {
            Self { simd: i16x8_shr(self.simd, rhs as u32) }
          } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
            unsafe {Self { neon: vshlq_s16(self.neon, vmovq_n_s16( -(rhs as i16))) }}
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
impl_shr_t_for_i16x8!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

impl CmpEq for i16x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_eq(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_eq_mask_i16_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i16x8_eq(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_s16_u16(vceqq_s16(self.neon, rhs.neon)) }}
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
        ]}
      }
    }
  }
}

impl CmpGt for i16x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_gt(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_gt_mask_i16_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i16x8_gt(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_s16_u16(vcgtq_s16(self.neon, rhs.neon)) }}
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
        ]}
      }
    }
  }
}

impl CmpLt for i16x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_lt(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_lt_mask_i16_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i16x8_lt(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_s16_u16(vcltq_s16(self.neon, rhs.neon)) }}
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
        ]}
      }
    }
  }
}

impl i16x8 {
  #[inline]
  #[must_use]
  pub const fn new(array: [i16; 8]) -> Self {
    unsafe { core::mem::transmute(array) }
  }

  #[inline]
  #[must_use]
  pub fn move_mask(self) -> i32 {
    pick! {
      if #[cfg(target_feature="sse2")] {
        move_mask_i8_m128i( pack_i16_to_i8_m128i(self.sse,self.sse)) & 0xff
      } else if #[cfg(target_feature="simd128")] {
        i16x8_bitmask(self.simd) as i32
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe
        {
          // set all to 1 if top bit is set, else 0
          let masked = vcltq_s16(self.neon, vdupq_n_s16(0));

          // select the right bit out of each lane
          let selectbit : uint16x8_t = core::mem::transmute([1u16, 2, 4, 8, 16, 32, 64, 128]);
          let r = vandq_u16(masked, selectbit);

          // horizontally add the 16-bit lanes
          vaddvq_u16(r) as i32
         }
       } else {
        ((self.arr[0] < 0) as i32) << 0 |
        ((self.arr[1] < 0) as i32) << 1 |
        ((self.arr[2] < 0) as i32) << 2 |
        ((self.arr[3] < 0) as i32) << 3 |
        ((self.arr[4] < 0) as i32) << 4 |
        ((self.arr[5] < 0) as i32) << 5 |
        ((self.arr[6] < 0) as i32) << 6 |
        ((self.arr[7] < 0) as i32) << 7
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn any(self) -> bool {
    pick! {
      if #[cfg(target_feature="sse2")] {
        (move_mask_i8_m128i(self.sse) & 0b1010101010101010) != 0
      } else if #[cfg(target_feature="simd128")] {
        u16x8_bitmask(self.simd) != 0
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
        unsafe {
          vminvq_s16(self.neon) < 0
        }
      } else {
        let v : [u64;2] = cast(self);
        ((v[0] | v[1]) & 0x8000800080008000) != 0
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn all(self) -> bool {
    pick! {
      if #[cfg(target_feature="sse2")] {
        (move_mask_i8_m128i(self.sse) & 0b1010101010101010) == 0b1010101010101010
      } else if #[cfg(target_feature="simd128")] {
        u16x8_bitmask(self.simd) == 0b11111111
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
        unsafe {
          vmaxvq_s16(self.neon) < 0
        }
      } else {
        let v : [u64;2] = cast(self);
        (v[0] & v[1] & 0x8000800080008000) == 0x8000800080008000
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn none(self) -> bool {
    !self.any()
  }

  /// Unpack the lower half of the input and expand it to `i16` values.
  #[inline]
  #[must_use]
  pub fn from_u8x16_low(u: u8x16) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self{ sse: unpack_low_i8_m128i(u.sse, m128i::zeroed()) }
      } else {
        let u_arr: [u8; 16] = cast(u);
        cast([
          u_arr[0] as u16 as i16,
          u_arr[1] as u16 as i16,
          u_arr[2] as u16 as i16,
          u_arr[3] as u16 as i16,
          u_arr[4] as u16 as i16,
          u_arr[5] as u16 as i16,
          u_arr[6] as u16 as i16,
          u_arr[7] as u16 as i16,
        ])
      }
    }
  }

  /// Unpack the upper half of the input and expand it to `i16` values.
  #[inline]
  #[must_use]
  pub fn from_u8x16_high(u: u8x16) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self{ sse: unpack_high_i8_m128i(u.sse, m128i::zeroed()) }
      } else {
        let u_arr: [u8; 16] = cast(u);
        cast([
          u_arr[8] as u16 as i16,
          u_arr[9] as u16 as i16,
          u_arr[10] as u16 as i16,
          u_arr[11] as u16 as i16,
          u_arr[12] as u16 as i16,
          u_arr[13] as u16 as i16,
          u_arr[14] as u16 as i16,
          u_arr[15] as u16 as i16,
        ])
      }
    }
  }

  /// returns low `i16` of `i32`, saturating values that are too large
  #[inline]
  #[must_use]
  pub fn from_i32x8_saturate(v: i32x8) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        i16x8 { sse: pack_i32_to_i16_m128i( extract_m128i_from_m256i::<0>(v.avx2), extract_m128i_from_m256i::<1>(v.avx2))  }
      } else if #[cfg(target_feature="sse2")] {
        i16x8 { sse: pack_i32_to_i16_m128i( v.a.sse, v.b.sse ) }
      } else if #[cfg(target_feature="simd128")] {
        use core::arch::wasm32::*;

        i16x8 { simd: i16x8_narrow_i32x4(v.a.simd, v.b.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
        use core::arch::aarch64::*;

        unsafe {
          i16x8 { neon: vcombine_s16(vqmovn_s32(v.a.neon), vqmovn_s32(v.b.neon)) }
        }
      } else {
        fn clamp(a : i32) -> i16 {
            if a < i16::MIN as i32 {
                i16::MIN
            }
            else if a > i16::MAX as i32 {
                i16::MAX
            } else {
                a as i16
            }
        }

        i16x8::new([
          clamp(v.as_array_ref()[0]),
          clamp(v.as_array_ref()[1]),
          clamp(v.as_array_ref()[2]),
          clamp(v.as_array_ref()[3]),
          clamp(v.as_array_ref()[4]),
          clamp(v.as_array_ref()[5]),
          clamp(v.as_array_ref()[6]),
          clamp(v.as_array_ref()[7]),
        ])
      }
    }
  }

  /// returns low `i16` of `i32`, truncating the upper bits if they are set
  #[inline]
  #[must_use]
  pub fn from_i32x8_truncate(v: i32x8) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        let a = v.avx2.bitand(set_splat_i32_m256i(0xffff));
        i16x8 { sse: pack_i32_to_u16_m128i( extract_m128i_from_m256i::<0>(a), extract_m128i_from_m256i::<1>(a) ) }
      } else if #[cfg(target_feature="sse2")] {
        let a = shr_imm_i32_m128i::<16>(shl_imm_u32_m128i::<16>(v.a.sse));
        let b = shr_imm_i32_m128i::<16>(shl_imm_u32_m128i::<16>(v.b.sse));

        i16x8 { sse: pack_i32_to_i16_m128i( a, b)  }
      } else {
      i16x8::new([
        v.as_array_ref()[0] as i16,
        v.as_array_ref()[1] as i16,
        v.as_array_ref()[2] as i16,
        v.as_array_ref()[3] as i16,
        v.as_array_ref()[4] as i16,
        v.as_array_ref()[5] as i16,
        v.as_array_ref()[6] as i16,
        v.as_array_ref()[7] as i16,
      ])
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn from_slice_unaligned(input: &[i16]) -> Self {
    assert!(input.len() >= 8);

    pick! {
      if #[cfg(target_feature="sse2")] {
        unsafe { Self { sse: load_unaligned_m128i( &*(input.as_ptr() as * const [u8;16]) ) } }
      } else if #[cfg(target_feature="simd128")] {
        unsafe { Self { simd: v128_load(input.as_ptr() as *const v128 ) } }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vld1q_s16( input.as_ptr() as *const i16 ) } }
      } else {
        // 2018 edition doesn't have try_into
        unsafe { Self::new( *(input.as_ptr() as * const [i16;8]) ) }
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
        unsafe {Self { neon: vbslq_s16(vreinterpretq_u16_s16(self.neon), t.neon, f.neon) }}
      } else {
        generic_bit_blend(self, t, f)
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn is_negative(self) -> Self {
    self.cmp_lt(Self::zeroed())
  }

  /// horizontal add of all the elements of the vector
  #[inline]
  #[must_use]
  pub fn reduce_add(self) -> i16 {
    pick! {
      if #[cfg(target_feature="sse2")] {
        // there is a horizontal add instruction on ssse3, but apparently it is very slow on some AMD CPUs
        let hi64 = shuffle_ai_f32_all_m128i::<0b01_00_11_10>(self.sse);
        let sum64 = add_i16_m128i(self.sse, hi64);
        let hi32 = shuffle_ai_f32_all_m128i::<0b11_10_00_01>(sum64);
        let sum32 = add_i16_m128i(sum64, hi32);
        let lo16 = shr_imm_u32_m128i::<16>(sum32);
        let sum16 = add_i16_m128i(sum32, lo16);
        extract_i16_as_i32_m128i::<0>(sum16) as i16
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { vaddvq_s16(self.neon) }
      } else {
        let arr: [i16; 8] = cast(self);

        // most boring implementation possible so optimizer doesn't overthink this
        let mut r = arr[0];
        r = r.wrapping_add(arr[1]);
        r = r.wrapping_add(arr[2]);
        r = r.wrapping_add(arr[3]);
        r = r.wrapping_add(arr[4]);
        r = r.wrapping_add(arr[5]);
        r = r.wrapping_add(arr[6]);
        r.wrapping_add(arr[7])
      }
    }
  }

  /// horizontal min of all the elements of the vector
  #[inline]
  #[must_use]
  pub fn reduce_min(self) -> i16 {
    pick! {
        if #[cfg(target_feature="sse2")] {
          let hi64 = shuffle_ai_f32_all_m128i::<0b01_00_11_10>(self.sse);
          let sum64 = min_i16_m128i(self.sse, hi64);
          let hi32 = shuffle_ai_f32_all_m128i::<0b11_10_00_01>(sum64);
          let sum32 = min_i16_m128i(sum64, hi32);
          let lo16 = shr_imm_u32_m128i::<16>(sum32);
          let sum16 = min_i16_m128i(sum32, lo16);
          extract_i16_as_i32_m128i::<0>(sum16) as i16
        } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
          unsafe { vminvq_s16(self.neon) }
        } else {
        let arr: [i16; 8] = cast(self);

        // most boring implementation possible so optimizer doesn't overthink this
        let mut r = arr[0];
        r = r.min(arr[1]);
        r = r.min(arr[2]);
        r = r.min(arr[3]);
        r = r.min(arr[4]);
        r = r.min(arr[5]);
        r = r.min(arr[6]);
        r.min(arr[7])
      }
    }
  }

  /// horizontal max of all the elements of the vector
  #[inline]
  #[must_use]
  pub fn reduce_max(self) -> i16 {
    pick! {
        if #[cfg(target_feature="sse2")] {
          let hi64 = shuffle_ai_f32_all_m128i::<0b01_00_11_10>(self.sse);
          let sum64 = max_i16_m128i(self.sse, hi64);
          let hi32 = shuffle_ai_f32_all_m128i::<0b11_10_00_01>(sum64);
          let sum32 = max_i16_m128i(sum64, hi32);
          let lo16 = shr_imm_u32_m128i::<16>(sum32);
          let sum16 = max_i16_m128i(sum32, lo16);
          extract_i16_as_i32_m128i::<0>(sum16) as i16
        } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
          unsafe { vmaxvq_s16(self.neon) }
        } else {
        let arr: [i16; 8] = cast(self);

        // most boring implementation possible so optimizer doesn't overthink this
        let mut r = arr[0];
        r = r.max(arr[1]);
        r = r.max(arr[2]);
        r = r.max(arr[3]);
        r = r.max(arr[4]);
        r = r.max(arr[5]);
        r = r.max(arr[6]);
        r.max(arr[7])
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn abs(self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        let mask = shr_imm_i16_m128i::<15>(self.sse);
        Self { sse: bitxor_m128i(add_i16_m128i(self.sse, mask), mask) }
      } else if #[cfg(target_feature="ssse3")] {
        Self { sse: abs_i16_m128i(self.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i16x8_abs(self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vabsq_s16(self.neon) }}
      } else {
        let arr: [i16; 8] = cast(self);
        cast(
          [
            arr[0].wrapping_abs(),
            arr[1].wrapping_abs(),
            arr[2].wrapping_abs(),
            arr[3].wrapping_abs(),
            arr[4].wrapping_abs(),
            arr[5].wrapping_abs(),
            arr[6].wrapping_abs(),
            arr[7].wrapping_abs(),
          ])
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn unsigned_abs(self) -> u16x8 {
    pick! {
      if #[cfg(target_feature="sse2")] {
        let mask = shr_imm_i16_m128i::<15>(self.sse);
        u16x8 { sse: bitxor_m128i(add_i16_m128i(self.sse, mask), mask) }
      } else if #[cfg(target_feature="ssse3")] {
        u16x8 { sse: abs_i16_m128i(self.sse) }
      } else if #[cfg(target_feature="simd128")] {
        u16x8 { simd: i16x8_abs(self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {u16x8 { neon: vreinterpretq_u16_s16(vabsq_s16(self.neon)) }}
      } else {
        let arr: [i16; 8] = cast(self);
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
          ])
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn max(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: max_i16_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i16x8_max(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vmaxq_s16(self.neon, rhs.neon) }}
      } else {
        self.cmp_lt(rhs).blend(rhs, self)
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn min(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: min_i16_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i16x8_min(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vminq_s16(self.neon, rhs.neon) }}
      } else {
        self.cmp_lt(rhs).blend(self, rhs)
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn saturating_add(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: add_saturating_i16_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i16x8_add_sat(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vqaddq_s16(self.neon, rhs.neon) }}
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
        Self { sse: sub_saturating_i16_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i16x8_sub_sat(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vqsubq_s16(self.neon, rhs.neon) } }
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

  /// Calculates partial dot product.
  /// Multiplies packed signed 16-bit integers, producing intermediate signed
  /// 32-bit integers. Horizontally add adjacent pairs of intermediate 32-bit
  /// integers.
  #[inline]
  #[must_use]
  pub fn dot(self, rhs: Self) -> i32x4 {
    pick! {
      if #[cfg(target_feature="sse2")] {
        i32x4 { sse:  mul_i16_horizontal_add_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        i32x4 { simd: i32x4_dot_i16x8(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {
          let pl = vmull_s16(vget_low_s16(self.neon),  vget_low_s16(rhs.neon));
          let ph = vmull_high_s16(self.neon, rhs.neon);
          i32x4 { neon: vpaddq_s32(pl, ph) }
        }
      } else {
        i32x4 { arr: [
          (i32::from(self.arr[0]) * i32::from(rhs.arr[0])) + (i32::from(self.arr[1]) * i32::from(rhs.arr[1])),
          (i32::from(self.arr[2]) * i32::from(rhs.arr[2])) + (i32::from(self.arr[3]) * i32::from(rhs.arr[3])),
          (i32::from(self.arr[4]) * i32::from(rhs.arr[4])) + (i32::from(self.arr[5]) * i32::from(rhs.arr[5])),
          (i32::from(self.arr[6]) * i32::from(rhs.arr[6])) + (i32::from(self.arr[7]) * i32::from(rhs.arr[7])),
        ] }
      }
    }
  }

  /// Multiply and scale equivalent to `((self * rhs) + 0x4000) >> 15` on each
  /// lane, effectively multiplying by a 16 bit fixed point number between `-1`
  /// and `1`. This corresponds to the following instructions:
  /// - `vqrdmulhq_s16` instruction on neon
  /// - `i16x8_q15mulr_sat` on simd128
  /// - `_mm_mulhrs_epi16` on ssse3
  /// - emulated via `mul_i16_*` on sse2
  #[inline]
  #[must_use]
  pub fn mul_scale_round(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="ssse3")] {
        Self { sse:  mul_i16_scale_round_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="sse2")] {
        // unfortunately mul_i16_scale_round_m128i only got added in sse3
        let hi = mul_i16_keep_high_m128i(self.sse, rhs.sse);
        let lo = mul_i16_keep_low_m128i(self.sse, rhs.sse);
        let mut v1 = unpack_low_i16_m128i(lo, hi);
        let mut v2 = unpack_high_i16_m128i(lo, hi);
        let a = set_splat_i32_m128i(0x4000);
        v1 = shr_imm_i32_m128i::<15>(add_i32_m128i(v1, a));
        v2 = shr_imm_i32_m128i::<15>(add_i32_m128i(v2, a));
        let s = pack_i32_to_i16_m128i(v1, v2);
        Self { sse: s }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i16x8_q15mulr_sat(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vqrdmulhq_s16(self.neon, rhs.neon) } }
      } else {
        // compiler does a surprisingly good job of vectorizing this
        Self { arr: [
          ((i32::from(self.arr[0]) * i32::from(rhs.arr[0]) + 0x4000) >> 15) as i16,
          ((i32::from(self.arr[1]) * i32::from(rhs.arr[1]) + 0x4000) >> 15) as i16,
          ((i32::from(self.arr[2]) * i32::from(rhs.arr[2]) + 0x4000) >> 15) as i16,
          ((i32::from(self.arr[3]) * i32::from(rhs.arr[3]) + 0x4000) >> 15) as i16,
          ((i32::from(self.arr[4]) * i32::from(rhs.arr[4]) + 0x4000) >> 15) as i16,
          ((i32::from(self.arr[5]) * i32::from(rhs.arr[5]) + 0x4000) >> 15) as i16,
          ((i32::from(self.arr[6]) * i32::from(rhs.arr[6]) + 0x4000) >> 15) as i16,
          ((i32::from(self.arr[7]) * i32::from(rhs.arr[7]) + 0x4000) >> 15) as i16,
        ]}
      }
    }
  }

  /// Multiples two `i16x8` and return the high part of intermediate `i32x8`
  #[inline]
  #[must_use]
  pub fn mul_keep_high(lhs: Self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: mul_i16_keep_high_m128i(lhs.sse, rhs.sse) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
        let lhs_low = unsafe { vget_low_s16(lhs.neon) };
        let rhs_low = unsafe { vget_low_s16(rhs.neon) };

        let lhs_high = unsafe { vget_high_s16(lhs.neon) };
        let rhs_high = unsafe { vget_high_s16(rhs.neon) };

        let low = unsafe { vmull_s16(lhs_low, rhs_low) };
        let high = unsafe { vmull_s16(lhs_high, rhs_high) };

        i16x8 { neon: unsafe { vreinterpretq_s16_u16(vuzpq_u16(vreinterpretq_u16_s32(low), vreinterpretq_u16_s32(high)).1) } }
      } else if #[cfg(target_feature="simd128")] {
        let low =  i32x4_extmul_low_i16x8(lhs.simd, rhs.simd);
        let high = i32x4_extmul_high_i16x8(lhs.simd, rhs.simd);

        Self { simd: i16x8_shuffle::<1, 3, 5, 7, 9, 11, 13, 15>(low, high) }
      } else {
        i16x8::new([
          ((i32::from(rhs.as_array_ref()[0]) * i32::from(lhs.as_array_ref()[0])) >> 16) as i16,
          ((i32::from(rhs.as_array_ref()[1]) * i32::from(lhs.as_array_ref()[1])) >> 16) as i16,
          ((i32::from(rhs.as_array_ref()[2]) * i32::from(lhs.as_array_ref()[2])) >> 16) as i16,
          ((i32::from(rhs.as_array_ref()[3]) * i32::from(lhs.as_array_ref()[3])) >> 16) as i16,
          ((i32::from(rhs.as_array_ref()[4]) * i32::from(lhs.as_array_ref()[4])) >> 16) as i16,
          ((i32::from(rhs.as_array_ref()[5]) * i32::from(lhs.as_array_ref()[5])) >> 16) as i16,
          ((i32::from(rhs.as_array_ref()[6]) * i32::from(lhs.as_array_ref()[6])) >> 16) as i16,
          ((i32::from(rhs.as_array_ref()[7]) * i32::from(lhs.as_array_ref()[7])) >> 16) as i16,
        ])
      }
    }
  }

  /// multiplies two `i16x8` and returns the result as a widened `i32x8`
  #[inline]
  #[must_use]
  pub fn mul_widen(self, rhs: Self) -> i32x8 {
    pick! {
      if #[cfg(target_feature="avx2")] {
        let a = convert_to_i32_m256i_from_i16_m128i(self.sse);
        let b = convert_to_i32_m256i_from_i16_m128i(rhs.sse);
        i32x8 { avx2: mul_i32_keep_low_m256i(a,b) }
      } else if #[cfg(target_feature="sse2")] {
         let low = mul_i16_keep_low_m128i(self.sse, rhs.sse);
         let high = mul_i16_keep_high_m128i(self.sse, rhs.sse);
         i32x8 {
          a: i32x4 { sse:unpack_low_i16_m128i(low, high) },
          b: i32x4 { sse:unpack_high_i16_m128i(low, high) }
        }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
         let lhs_low = unsafe { vget_low_s16(self.neon) };
         let rhs_low = unsafe { vget_low_s16(rhs.neon) };

         let lhs_high = unsafe { vget_high_s16(self.neon) };
         let rhs_high = unsafe { vget_high_s16(rhs.neon) };

         let low = unsafe { vmull_s16(lhs_low, rhs_low) };
         let high = unsafe { vmull_s16(lhs_high, rhs_high) };

         i32x8 { a: i32x4 { neon: low }, b: i32x4 {neon: high } }
       } else {
        let a = self.as_array_ref();
        let b = rhs.as_array_ref();
         i32x8::new([
           i32::from(a[0]) * i32::from(b[0]),
           i32::from(a[1]) * i32::from(b[1]),
           i32::from(a[2]) * i32::from(b[2]),
           i32::from(a[3]) * i32::from(b[3]),
           i32::from(a[4]) * i32::from(b[4]),
           i32::from(a[5]) * i32::from(b[5]),
           i32::from(a[6]) * i32::from(b[6]),
           i32::from(a[7]) * i32::from(b[7]),
         ])
       }
    }
  }

  /// transpose matrix of 8x8 i16 matrix
  #[must_use]
  #[inline]
  pub fn transpose(data: [i16x8; 8]) -> [i16x8; 8] {
    pick! {
      if #[cfg(target_feature="sse2")] {
        let a1 = unpack_low_i16_m128i(data[0].sse, data[1].sse);
        let a2 = unpack_high_i16_m128i(data[0].sse, data[1].sse);
        let a3 = unpack_low_i16_m128i(data[2].sse, data[3].sse);
        let a4 = unpack_high_i16_m128i(data[2].sse, data[3].sse);
        let a5 = unpack_low_i16_m128i(data[4].sse, data[5].sse);
        let a6 = unpack_high_i16_m128i(data[4].sse, data[5].sse);
        let a7 = unpack_low_i16_m128i(data[6].sse, data[7].sse);
        let a8 = unpack_high_i16_m128i(data[6].sse, data[7].sse);

        let b1 = unpack_low_i32_m128i(a1, a3);
        let b2 = unpack_high_i32_m128i(a1, a3);
        let b3 = unpack_low_i32_m128i(a2, a4);
        let b4 = unpack_high_i32_m128i(a2, a4);
        let b5 = unpack_low_i32_m128i(a5, a7);
        let b6 = unpack_high_i32_m128i(a5, a7);
        let b7 = unpack_low_i32_m128i(a6, a8);
        let b8 = unpack_high_i32_m128i(a6, a8);

        [
          i16x8 { sse: unpack_low_i64_m128i(b1, b5) },
          i16x8 { sse: unpack_high_i64_m128i(b1, b5) },
          i16x8 { sse: unpack_low_i64_m128i(b2, b6) },
          i16x8 { sse: unpack_high_i64_m128i(b2, b6) },
          i16x8 { sse: unpack_low_i64_m128i(b3, b7) },
          i16x8 { sse: unpack_high_i64_m128i(b3, b7) },
          i16x8 { sse: unpack_low_i64_m128i(b4, b8) },
          i16x8 { sse: unpack_high_i64_m128i(b4, b8) } ,
        ]
     } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{

          #[inline] fn vtrq32(a : int16x8_t, b : int16x8_t) -> (int16x8_t, int16x8_t)
          {
              unsafe {
                let r = vtrnq_s32(vreinterpretq_s32_s16(a),vreinterpretq_s32_s16(b));
                (vreinterpretq_s16_s32(r.0), vreinterpretq_s16_s32(r.1))
              }
          }

        unsafe {
          let (q0,q2) = vtrq32(data[0].neon, data[2].neon);
          let (q1,q3) = vtrq32(data[1].neon, data[3].neon);
          let (q4,q6) = vtrq32(data[4].neon, data[6].neon);
          let (q5,q7) = vtrq32(data[5].neon, data[7].neon);

          let b1 = vtrnq_s16(q0, q1);
          let b2 = vtrnq_s16(q2, q3);
          let b3 = vtrnq_s16(q4, q5);
          let b4 = vtrnq_s16(q6, q7);

          // There is no vtrnq_s64 unfortunately, so there's this mess
          // which does a somewhat reasonable job, but not as good as the
          // assembly versions which just swap the 64 bit register aliases.
          [
            i16x8 { neon: vcombine_s16(vget_low_s16(b1.0), vget_low_s16(b3.0)) },
            i16x8 { neon: vcombine_s16(vget_low_s16(b1.1), vget_low_s16(b3.1)) },
            i16x8 { neon: vcombine_s16(vget_low_s16(b2.0), vget_low_s16(b4.0)) },
            i16x8 { neon: vcombine_s16(vget_low_s16(b2.1), vget_low_s16(b4.1)) },
            i16x8 { neon: vcombine_s16(vget_high_s16(b1.0), vget_high_s16(b3.0)) },
            i16x8 { neon: vcombine_s16(vget_high_s16(b1.1), vget_high_s16(b3.1)) },
            i16x8 { neon: vcombine_s16(vget_high_s16(b2.0), vget_high_s16(b4.0)) },
            i16x8 { neon: vcombine_s16(vget_high_s16(b2.1), vget_high_s16(b4.1)) },
          ]
        }
      } else if #[cfg(target_feature="simd128")] {
        #[inline] fn lo_i16(a : v128, b : v128) -> v128 { i16x8_shuffle::<0, 8, 1, 9, 2, 10, 3, 11>(a,b) }
        #[inline] fn hi_i16(a : v128, b : v128) -> v128 { i16x8_shuffle::<4, 12, 5, 13, 6, 14, 7, 15>(a,b) }
        #[inline] fn lo_i32(a : v128, b : v128) -> v128 { i32x4_shuffle::<0, 4, 1, 5>(a,b) }
        #[inline] fn hi_i32(a : v128, b : v128) -> v128 { i32x4_shuffle::<2, 6, 3, 7>(a,b) }
        #[inline] fn lo_i64(a : v128, b : v128) -> v128 { i64x2_shuffle::<0, 2>(a,b) }
        #[inline] fn hi_i64(a : v128, b : v128) -> v128 { i64x2_shuffle::<1, 3>(a,b) }

        let a1 = lo_i16(data[0].simd, data[1].simd);
        let a2 = hi_i16(data[0].simd, data[1].simd);
        let a3 = lo_i16(data[2].simd, data[3].simd);
        let a4 = hi_i16(data[2].simd, data[3].simd);
        let a5 = lo_i16(data[4].simd, data[5].simd);
        let a6 = hi_i16(data[4].simd, data[5].simd);
        let a7 = lo_i16(data[6].simd, data[7].simd);
        let a8 = hi_i16(data[6].simd, data[7].simd);

        let b1 = lo_i32(a1, a3);
        let b2 = hi_i32(a1, a3);
        let b3 = lo_i32(a2, a4);
        let b4 = hi_i32(a2, a4);
        let b5 = lo_i32(a5, a7);
        let b6 = hi_i32(a5, a7);
        let b7 = lo_i32(a6, a8);
        let b8 = hi_i32(a6, a8);

        [
          i16x8 { simd: lo_i64(b1, b5) },
          i16x8 { simd: hi_i64(b1, b5) },
          i16x8 { simd: lo_i64(b2, b6) },
          i16x8 { simd: hi_i64(b2, b6) },
          i16x8 { simd: lo_i64(b3, b7) },
          i16x8 { simd: hi_i64(b3, b7) },
          i16x8 { simd: lo_i64(b4, b8) },
          i16x8 { simd: hi_i64(b4, b8) } ,
        ]

      } else {
        #[inline(always)]
        fn transpose_column(data: &[i16x8; 8], index: usize) -> i16x8 {
          i16x8::new([
            data[0].as_array_ref()[index],
            data[1].as_array_ref()[index],
            data[2].as_array_ref()[index],
            data[3].as_array_ref()[index],
            data[4].as_array_ref()[index],
            data[5].as_array_ref()[index],
            data[6].as_array_ref()[index],
            data[7].as_array_ref()[index],
          ])
        }

        [
          transpose_column(&data, 0),
          transpose_column(&data, 1),
          transpose_column(&data, 2),
          transpose_column(&data, 3),
          transpose_column(&data, 4),
          transpose_column(&data, 5),
          transpose_column(&data, 6),
          transpose_column(&data, 7),
        ]
      }
    }
  }

  #[inline]
  #[must_use]
  /// Multiply and scale, equivalent to `((self * rhs) + 0x4000) >> 15` on each
  /// lane, effectively multiplying by a 16 bit fixed point number between `-1`
  /// and `1`. This corresponds to the following instructions:
  /// - `vqrdmulhq_n_s16` instruction on neon
  /// - `i16x8_q15mulr_sat` on simd128
  /// - `_mm_mulhrs_epi16` on ssse3
  /// - emulated via `mul_i16_*` on sse2
  pub fn mul_scale_round_n(self, rhs: i16) -> Self {
    pick! {
      if #[cfg(target_feature="ssse3")] {
        Self { sse:  mul_i16_scale_round_m128i(self.sse, set_splat_i16_m128i(rhs)) }
      } else if #[cfg(target_feature="sse2")] {
        // unfortunately mul_i16_scale_round_m128i only got added in sse3
        let r = set_splat_i16_m128i(rhs);
        let hi = mul_i16_keep_high_m128i(self.sse, r);
        let lo = mul_i16_keep_low_m128i(self.sse, r);
        let mut v1 = unpack_low_i16_m128i(lo, hi);
        let mut v2 = unpack_high_i16_m128i(lo, hi);
        let a = set_splat_i32_m128i(0x4000);
        v1 = shr_imm_i32_m128i::<15>(add_i32_m128i(v1, a));
        v2 = shr_imm_i32_m128i::<15>(add_i32_m128i(v2, a));
        let s = pack_i32_to_i16_m128i(v1, v2);
        Self { sse: s }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i16x8_q15mulr_sat(self.simd, i16x8_splat(rhs)) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vqrdmulhq_n_s16(self.neon, rhs) } }
      } else {
        // compiler does a surprisingly good job of vectorizing this
        Self { arr: [
          ((i32::from(self.arr[0]) * i32::from(rhs) + 0x4000) >> 15) as i16,
          ((i32::from(self.arr[1]) * i32::from(rhs) + 0x4000) >> 15) as i16,
          ((i32::from(self.arr[2]) * i32::from(rhs) + 0x4000) >> 15) as i16,
          ((i32::from(self.arr[3]) * i32::from(rhs) + 0x4000) >> 15) as i16,
          ((i32::from(self.arr[4]) * i32::from(rhs) + 0x4000) >> 15) as i16,
          ((i32::from(self.arr[5]) * i32::from(rhs) + 0x4000) >> 15) as i16,
          ((i32::from(self.arr[6]) * i32::from(rhs) + 0x4000) >> 15) as i16,
          ((i32::from(self.arr[7]) * i32::from(rhs) + 0x4000) >> 15) as i16,
        ]}
      }
    }
  }

  #[inline]
  pub fn to_array(self) -> [i16; 8] {
    cast(self)
  }

  #[inline]
  pub fn as_array_ref(&self) -> &[i16; 8] {
    cast_ref(self)
  }

  #[inline]
  pub fn as_array_mut(&mut self) -> &mut [i16; 8] {
    cast_mut(self)
  }
}
