use super::*;

pick! {
  if #[cfg(target_feature="sse2")] {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(16))]
    pub struct i32x4 { pub(crate) sse: m128i }
  } else if #[cfg(target_feature="simd128")] {
    use core::arch::wasm32::*;

    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct i32x4 { pub(crate) simd: v128 }

    impl Default for i32x4 {
      fn default() -> Self {
        Self::splat(0)
      }
    }

    impl PartialEq for i32x4 {
      fn eq(&self, other: &Self) -> bool {
        u32x4_all_true(i32x4_eq(self.simd, other.simd))
      }
    }

    impl Eq for i32x4 { }
  } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
    use core::arch::aarch64::*;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct i32x4 { pub(crate) neon : int32x4_t }

    impl Default for i32x4 {
      #[inline]
      #[must_use]
      fn default() -> Self {
        Self::splat(0)
      }
    }

    impl PartialEq for i32x4 {
      #[inline]
      #[must_use]
      fn eq(&self, other: &Self) -> bool {
        unsafe { vminvq_u32(vceqq_s32(self.neon, other.neon))==u32::MAX }
      }
    }

    impl Eq for i32x4 { }
  } else {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(16))]
    pub struct i32x4 { pub(crate) arr: [i32;4] }
  }
}

int_uint_consts!(i32, 4, i32x4, 128);

unsafe impl Zeroable for i32x4 {}
unsafe impl Pod for i32x4 {}

impl Add for i32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: add_i32_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i32x4_add(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vaddq_s32(self.neon, rhs.neon) } }
      } else {
        Self { arr: [
          self.arr[0].wrapping_add(rhs.arr[0]),
          self.arr[1].wrapping_add(rhs.arr[1]),
          self.arr[2].wrapping_add(rhs.arr[2]),
          self.arr[3].wrapping_add(rhs.arr[3]),
        ]}
      }
    }
  }
}

impl Sub for i32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: sub_i32_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i32x4_sub(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vsubq_s32(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].wrapping_sub(rhs.arr[0]),
          self.arr[1].wrapping_sub(rhs.arr[1]),
          self.arr[2].wrapping_sub(rhs.arr[2]),
          self.arr[3].wrapping_sub(rhs.arr[3]),
        ]}
      }
    }
  }
}

impl Mul for i32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse4.1")] {
        Self { sse: mul_32_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i32x4_mul(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vmulq_s32(self.neon, rhs.neon) }}
      } else {
        let arr1: [i32; 4] = cast(self);
        let arr2: [i32; 4] = cast(rhs);
        cast([
          arr1[0].wrapping_mul(arr2[0]),
          arr1[1].wrapping_mul(arr2[1]),
          arr1[2].wrapping_mul(arr2[2]),
          arr1[3].wrapping_mul(arr2[3]),
        ])
      }
    }
  }
}

impl Add<i32> for i32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: i32) -> Self::Output {
    self.add(Self::splat(rhs))
  }
}

impl Sub<i32> for i32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: i32) -> Self::Output {
    self.sub(Self::splat(rhs))
  }
}

impl Mul<i32> for i32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: i32) -> Self::Output {
    self.mul(Self::splat(rhs))
  }
}

impl Add<i32x4> for i32 {
  type Output = i32x4;
  #[inline]
  #[must_use]
  fn add(self, rhs: i32x4) -> Self::Output {
    i32x4::splat(self).add(rhs)
  }
}

impl Sub<i32x4> for i32 {
  type Output = i32x4;
  #[inline]
  #[must_use]
  fn sub(self, rhs: i32x4) -> Self::Output {
    i32x4::splat(self).sub(rhs)
  }
}

impl Mul<i32x4> for i32 {
  type Output = i32x4;
  #[inline]
  #[must_use]
  fn mul(self, rhs: i32x4) -> Self::Output {
    i32x4::splat(self).mul(rhs)
  }
}

impl BitAnd for i32x4 {
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
        unsafe {Self { neon: vandq_s32(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].bitand(rhs.arr[0]),
          self.arr[1].bitand(rhs.arr[1]),
          self.arr[2].bitand(rhs.arr[2]),
          self.arr[3].bitand(rhs.arr[3]),
        ]}
      }
    }
  }
}

impl BitOr for i32x4 {
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
        unsafe {Self { neon: vorrq_s32(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].bitor(rhs.arr[0]),
          self.arr[1].bitor(rhs.arr[1]),
          self.arr[2].bitor(rhs.arr[2]),
          self.arr[3].bitor(rhs.arr[3]),
        ]}
      }
    }
  }
}

impl BitXor for i32x4 {
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
        unsafe {Self { neon: veorq_s32(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          self.arr[0].bitxor(rhs.arr[0]),
          self.arr[1].bitxor(rhs.arr[1]),
          self.arr[2].bitxor(rhs.arr[2]),
          self.arr[3].bitxor(rhs.arr[3]),
        ]}
      }
    }
  }
}

macro_rules! impl_shl_t_for_i32x4 {
  ($($shift_type:ty),+ $(,)?) => {
    $(impl Shl<$shift_type> for i32x4 {
      type Output = Self;
      /// Shifts all lanes by the value given.
      #[inline]
      #[must_use]
      fn shl(self, rhs: $shift_type) -> Self::Output {
        pick! {
          if #[cfg(target_feature="sse2")] {
            let shift = cast([rhs as u64, 0]);
            Self { sse: shl_all_u32_m128i(self.sse, shift) }
          } else if #[cfg(target_feature="simd128")] {
            Self { simd: i32x4_shl(self.simd, rhs as u32) }
          } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
            unsafe {Self { neon: vshlq_s32(self.neon, vmovq_n_s32(rhs as i32)) }}
          } else {
            let u = rhs as u64;
            Self { arr: [
              self.arr[0] << u,
              self.arr[1] << u,
              self.arr[2] << u,
              self.arr[3] << u,
            ]}
          }
        }
      }
    })+
  };
}
impl_shl_t_for_i32x4!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

macro_rules! impl_shr_t_for_i32x4 {
  ($($shift_type:ty),+ $(,)?) => {
    $(impl Shr<$shift_type> for i32x4 {
      type Output = Self;
      /// Shifts all lanes by the value given.
      #[inline]
      #[must_use]
      fn shr(self, rhs: $shift_type) -> Self::Output {
        pick! {
          if #[cfg(target_feature="sse2")] {
            let shift = cast([rhs as u64, 0]);
            Self { sse: shr_all_i32_m128i(self.sse, shift) }
          } else if #[cfg(target_feature="simd128")] {
            Self { simd: i32x4_shr(self.simd, rhs as u32) }
          } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
            unsafe {Self { neon: vshlq_s32(self.neon, vmovq_n_s32( -(rhs as i32))) }}
          } else {
            let u = rhs as u64;
            Self { arr: [
              self.arr[0] >> u,
              self.arr[1] >> u,
              self.arr[2] >> u,
              self.arr[3] >> u,
            ]}
          }
        }
      }
    })+
  };
}
impl_shr_t_for_i32x4!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

/// Shifts lanes by the corresponding lane.
///
/// Bitwise shift-right; yields `self >> mask(rhs)`, where mask removes any
/// high-order bits of `rhs` that would cause the shift to exceed the bitwidth
/// of the type. (same as `wrapping_shr`)
impl Shr<i32x4> for i32x4 {
  type Output = Self;

  #[inline]
  #[must_use]
  fn shr(self, rhs: i32x4) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        // mask the shift count to 31 to have same behavior on all platforms
        let shift_by = bitand_m128i(rhs.sse, set_splat_i32_m128i(31));
        Self { sse: shr_each_i32_m128i(self.sse, shift_by) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {
          // mask the shift count to 31 to have same behavior on all platforms
          // no right shift, have to pass negative value to left shift on neon
          let shift_by = vnegq_s32(vandq_s32(rhs.neon, vmovq_n_s32(31)));
          Self { neon: vshlq_s32(self.neon, shift_by) }
        }
      } else {
        let arr: [i32; 4] = cast(self);
        let rhs: [i32; 4] = cast(rhs);
        cast([
          arr[0].wrapping_shr(rhs[0] as u32),
          arr[1].wrapping_shr(rhs[1] as u32),
          arr[2].wrapping_shr(rhs[2] as u32),
          arr[3].wrapping_shr(rhs[3] as u32),
        ])
      }
    }
  }
}

/// Shifts lanes by the corresponding lane.
///
/// Bitwise shift-left; yields `self << mask(rhs)`, where mask removes any
/// high-order bits of `rhs` that would cause the shift to exceed the bitwidth
/// of the type. (same as `wrapping_shl`)
impl Shl<i32x4> for i32x4 {
  type Output = Self;

  #[inline]
  #[must_use]
  fn shl(self, rhs: i32x4) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        // mask the shift count to 31 to have same behavior on all platforms
        let shift_by = bitand_m128i(rhs.sse, set_splat_i32_m128i(31));
        Self { sse: shl_each_u32_m128i(self.sse, shift_by) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {
          // mask the shift count to 31 to have same behavior on all platforms
          let shift_by = vandq_s32(rhs.neon, vmovq_n_s32(31));
          Self { neon: vshlq_s32(self.neon, shift_by) }
        }
      } else {
        let arr: [i32; 4] = cast(self);
        let rhs: [i32; 4] = cast(rhs);
        cast([
          arr[0].wrapping_shl(rhs[0] as u32),
          arr[1].wrapping_shl(rhs[1] as u32),
          arr[2].wrapping_shl(rhs[2] as u32),
          arr[3].wrapping_shl(rhs[3] as u32),
        ])
      }
    }
  }
}

impl CmpEq for i32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_eq(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_eq_mask_i32_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i32x4_eq(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_s32_u32(vceqq_s32(self.neon, rhs.neon)) }}
      } else {
        Self { arr: [
          if self.arr[0] == rhs.arr[0] { -1 } else { 0 },
          if self.arr[1] == rhs.arr[1] { -1 } else { 0 },
          if self.arr[2] == rhs.arr[2] { -1 } else { 0 },
          if self.arr[3] == rhs.arr[3] { -1 } else { 0 },
        ]}
      }
    }
  }
}

impl CmpGt for i32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_gt(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_gt_mask_i32_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i32x4_gt(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_s32_u32(vcgtq_s32(self.neon, rhs.neon)) }}
      } else {
        Self { arr: [
          if self.arr[0] > rhs.arr[0] { -1 } else { 0 },
          if self.arr[1] > rhs.arr[1] { -1 } else { 0 },
          if self.arr[2] > rhs.arr[2] { -1 } else { 0 },
          if self.arr[3] > rhs.arr[3] { -1 } else { 0 },
        ]}
      }
    }
  }
}

impl CmpLt for i32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_lt(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_lt_mask_i32_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i32x4_lt(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vreinterpretq_s32_u32(vcltq_s32(self.neon, rhs.neon)) }}
      } else {
        Self { arr: [
          if self.arr[0] < rhs.arr[0] { -1 } else { 0 },
          if self.arr[1] < rhs.arr[1] { -1 } else { 0 },
          if self.arr[2] < rhs.arr[2] { -1 } else { 0 },
          if self.arr[3] < rhs.arr[3] { -1 } else { 0 },
        ]}
      }
    }
  }
}

impl i32x4 {
  #[inline]
  #[must_use]
  pub const fn new(array: [i32; 4]) -> Self {
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
        unsafe {Self { neon: vbslq_s32(vreinterpretq_u32_s32(self.neon), t.neon, f.neon) }}
      } else {
        generic_bit_blend(self, t, f)
      }
    }
  }

  /// Multiplies corresponding 32 bit lanes and returns the 64 bit result
  /// on the corresponding lanes.
  ///
  /// Effectively does two multiplies on 128 bit platforms, but is easier
  /// to use than wrapping mul_widen_i32_odd_m128i individually.
  #[inline]
  #[must_use]
  pub fn mul_widen(self, rhs: Self) -> i64x4 {
    pick! {
      if #[cfg(target_feature="avx2")] {
        let a = convert_to_i64_m256i_from_i32_m128i(self.sse);
        let b = convert_to_i64_m256i_from_i32_m128i(rhs.sse);
        cast(mul_i64_low_bits_m256i(a, b))
      } else if #[cfg(target_feature="sse4.1")] {
          let evenp = mul_widen_i32_odd_m128i(self.sse, rhs.sse);

          let oddp = mul_widen_i32_odd_m128i(
            shr_imm_u64_m128i::<32>(self.sse),
            shr_imm_u64_m128i::<32>(rhs.sse));

          i64x4 {
            a: i64x2 { sse: unpack_low_i64_m128i(evenp, oddp)},
            b: i64x2 { sse: unpack_high_i64_m128i(evenp, oddp)}
          }
      } else if #[cfg(target_feature="simd128")] {
          i64x4 {
            a: i64x2 { simd: i64x2_extmul_low_i32x4(self.simd, rhs.simd) },
            b: i64x2 { simd: i64x2_extmul_high_i32x4(self.simd, rhs.simd) },
          }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
        unsafe {
          i64x4 { a: i64x2 { neon: vmull_s32(vget_low_s32(self.neon), vget_low_s32(rhs.neon)) },
                  b: i64x2 { neon: vmull_s32(vget_high_s32(self.neon), vget_high_s32(rhs.neon)) } }
        }
      } else {
        let a: [i32; 4] = cast(self);
        let b: [i32; 4] = cast(rhs);
        cast([
          i64::from(a[0]) * i64::from(b[0]),
          i64::from(a[1]) * i64::from(b[1]),
          i64::from(a[2]) * i64::from(b[2]),
          i64::from(a[3]) * i64::from(b[3]),
        ])
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn abs(self) -> Self {
    pick! {
      if #[cfg(target_feature="ssse3")] {
        Self { sse: abs_i32_m128i(self.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i32x4_abs(self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vabsq_s32(self.neon) }}
      } else {
        let arr: [i32; 4] = cast(self);
        cast([
          arr[0].wrapping_abs(),
          arr[1].wrapping_abs(),
          arr[2].wrapping_abs(),
          arr[3].wrapping_abs(),
        ])
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn unsigned_abs(self) -> u32x4 {
    pick! {
      if #[cfg(target_feature="ssse3")] {
        u32x4 { sse: abs_i32_m128i(self.sse) }
      } else if #[cfg(target_feature="simd128")] {
        u32x4 { simd: i32x4_abs(self.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {u32x4 { neon: vreinterpretq_u32_s32(vabsq_s32(self.neon)) }}
      } else {
        let arr: [i32; 4] = cast(self);
        cast([
          arr[0].unsigned_abs(),
          arr[1].unsigned_abs(),
          arr[2].unsigned_abs(),
          arr[3].unsigned_abs(),
        ])
      }
    }
  }

  /// horizontal add of all the elements of the vector
  #[inline]
  #[must_use]
  pub fn reduce_add(self) -> i32 {
    pick! {
      if #[cfg(target_feature="sse2")] {
        let hi64  = unpack_high_i64_m128i(self.sse, self.sse);
        let sum64 = add_i32_m128i(hi64, self.sse);
        let hi32  = shuffle_ai_f32_all_m128i::<0b10_11_00_01>(sum64);    // Swap the low two elements
        let sum32 = add_i32_m128i(sum64, hi32);
        get_i32_from_m128i_s(sum32)
      } else {
        let arr: [i32; 4] = cast(self);
        arr[0].wrapping_add(arr[1]).wrapping_add(
        arr[2].wrapping_add(arr[3]))
      }
    }
  }

  /// horizontal max of all the elements of the vector
  #[inline]
  #[must_use]
  pub fn reduce_max(self) -> i32 {
    let arr: [i32; 4] = cast(self);
    arr[0].max(arr[1]).max(arr[2].max(arr[3]))
  }

  /// horizontal min of all the elements of the vector
  #[inline]
  #[must_use]
  pub fn reduce_min(self) -> i32 {
    let arr: [i32; 4] = cast(self);
    arr[0].min(arr[1]).min(arr[2].min(arr[3]))
  }

  #[inline]
  #[must_use]
  pub fn max(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse4.1")] {
        Self { sse: max_i32_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i32x4_max(self.simd, rhs.simd) }
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
        Self { sse: min_i32_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: i32x4_min(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vminq_s32(self.neon, rhs.neon) }}
      } else {
        self.cmp_lt(rhs).blend(self, rhs)
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn round_float(self) -> f32x4 {
    pick! {
      if #[cfg(target_feature="sse2")] {
        cast(convert_to_m128_from_i32_m128i(self.sse))
      } else if #[cfg(target_feature="simd128")] {
        cast(Self { simd: f32x4_convert_i32x4(self.simd) })
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        cast(unsafe {Self { neon: vreinterpretq_s32_f32(vcvtq_f32_s32(self.neon)) }})
      } else {
        let arr: [i32; 4] = cast(self);
        cast([
          arr[0] as f32,
          arr[1] as f32,
          arr[2] as f32,
          arr[3] as f32,
        ])
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn move_mask(self) -> i32 {
    pick! {
      if #[cfg(target_feature="sse2")] {
        // use f32 move_mask since it is the same size as i32
        move_mask_m128(cast(self.sse))
      } else if #[cfg(target_feature="simd128")] {
        u32x4_bitmask(self.simd) as i32
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe
        {
          // set all to 1 if top bit is set, else 0
          let masked = vcltq_s32(self.neon, vdupq_n_s32(0));

          // select the right bit out of each lane
          let selectbit : uint32x4_t = core::mem::transmute([1u32, 2, 4, 8]);
          let r = vandq_u32(masked, selectbit);

          // horizontally add the 32-bit lanes
          vaddvq_u32(r) as i32
         }
      } else {
        ((self.arr[0] < 0) as i32) << 0 |
        ((self.arr[1] < 0) as i32) << 1 |
        ((self.arr[2] < 0) as i32) << 2 |
        ((self.arr[3] < 0) as i32) << 3
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn any(self) -> bool {
    pick! {
      if #[cfg(target_feature="sse2")] {
        // use f32 move_mask since it is the same size as i32
        move_mask_m128(cast(self.sse)) != 0
      } else if #[cfg(target_feature="simd128")] {
        u32x4_bitmask(self.simd) != 0
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
        // some lanes are negative
        unsafe {
          vminvq_s32(self.neon) < 0
        }
      } else {
        let v : [u64;2] = cast(self);
        ((v[0] | v[1]) & 0x8000000080000000) != 0
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn all(self) -> bool {
    pick! {
      if #[cfg(target_feature="sse2")] {
        // use f32 move_mask since it is the same size as i32
        move_mask_m128(cast(self.sse)) == 0b1111
      } else if #[cfg(target_feature="simd128")] {
        u32x4_bitmask(self.simd) == 0b1111
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        // all lanes are negative
        unsafe {
          vmaxvq_s32(self.neon) < 0
        }
      } else {
        let v : [u64;2] = cast(self);
        (v[0] & v[1] & 0x8000000080000000) == 0x8000000080000000
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn none(self) -> bool {
    !self.any()
  }

  /// Transpose matrix of 4x4 `i32` matrix. Currently only accelerated on SSE.
  #[must_use]
  #[inline]
  pub fn transpose(data: [i32x4; 4]) -> [i32x4; 4] {
    pick! {
      if #[cfg(target_feature="sse")] {
        let mut e0 = data[0];
        let mut e1 = data[1];
        let mut e2 = data[2];
        let mut e3 = data[3];

        transpose_four_m128(
          cast_mut(&mut e0.sse),
          cast_mut(&mut e1.sse),
          cast_mut(&mut e2.sse),
          cast_mut(&mut e3.sse),
        );

        [e0, e1, e2, e3]
      } else {
        #[inline(always)]
        fn transpose_column(data: &[i32x4; 4], index: usize) -> i32x4 {
          i32x4::new([
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
  pub fn to_array(self) -> [i32; 4] {
    cast(self)
  }

  #[inline]
  pub fn as_array_ref(&self) -> &[i32; 4] {
    cast_ref(self)
  }

  #[inline]
  pub fn as_array_mut(&mut self) -> &mut [i32; 4] {
    cast_mut(self)
  }
}
