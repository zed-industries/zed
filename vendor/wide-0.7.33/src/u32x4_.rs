use super::*;

pick! {
  if #[cfg(target_feature="sse2")] {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(16))]
    pub struct u32x4 { pub(crate) sse: m128i }
  } else if #[cfg(target_feature="simd128")] {
    use core::arch::wasm32::*;

    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct u32x4 { pub(crate) simd: v128 }

    impl Default for u32x4 {
      fn default() -> Self {
        Self::splat(0)
      }
    }

    impl PartialEq for u32x4 {
      fn eq(&self, other: &Self) -> bool {
        u32x4_all_true(u32x4_eq(self.simd, other.simd))
      }
    }

    impl Eq for u32x4 { }
  } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
    use core::arch::aarch64::*;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct u32x4 { pub(crate) neon : uint32x4_t }

    impl Default for u32x4 {
      #[inline]
      #[must_use]
      fn default() -> Self {
        Self::splat(0)
      }
    }

    impl PartialEq for u32x4 {
      #[inline]
      #[must_use]
      fn eq(&self, other: &Self) -> bool {
        unsafe { vminvq_u32(vceqq_u32(self.neon, other.neon))==u32::MAX }
      }
    }

    impl Eq for u32x4 { }
} else {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(16))]
    pub struct u32x4 { arr: [u32;4] }
  }
}

int_uint_consts!(u32, 4, u32x4, 128);

unsafe impl Zeroable for u32x4 {}
unsafe impl Pod for u32x4 {}

impl Add for u32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: add_i32_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u32x4_add(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe { Self { neon: vaddq_u32(self.neon, rhs.neon) } }
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

impl Sub for u32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: sub_i32_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u32x4_sub(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vsubq_u32(self.neon, rhs.neon) }}
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

impl Mul for u32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="sse4.1")] {
        Self { sse: mul_32_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u32x4_mul(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vmulq_u32(self.neon, rhs.neon) }}
      } else {
        let arr1: [u32; 4] = cast(self);
        let arr2: [u32; 4] = cast(rhs);
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

impl Add<u32> for u32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: u32) -> Self::Output {
    self.add(Self::splat(rhs))
  }
}

impl Sub<u32> for u32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: u32) -> Self::Output {
    self.sub(Self::splat(rhs))
  }
}

impl Mul<u32> for u32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: u32) -> Self::Output {
    self.mul(Self::splat(rhs))
  }
}

impl Add<u32x4> for u32 {
  type Output = u32x4;
  #[inline]
  #[must_use]
  fn add(self, rhs: u32x4) -> Self::Output {
    u32x4::splat(self).add(rhs)
  }
}

impl Sub<u32x4> for u32 {
  type Output = u32x4;
  #[inline]
  #[must_use]
  fn sub(self, rhs: u32x4) -> Self::Output {
    u32x4::splat(self).sub(rhs)
  }
}

impl Mul<u32x4> for u32 {
  type Output = u32x4;
  #[inline]
  #[must_use]
  fn mul(self, rhs: u32x4) -> Self::Output {
    u32x4::splat(self).mul(rhs)
  }
}

impl BitAnd for u32x4 {
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
        unsafe {Self { neon: vandq_u32(self.neon, rhs.neon) }}
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

impl BitOr for u32x4 {
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
        unsafe {Self { neon: vorrq_u32(self.neon, rhs.neon) }}
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

impl BitXor for u32x4 {
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
        unsafe {Self { neon: veorq_u32(self.neon, rhs.neon) }}
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

macro_rules! impl_shl_t_for_u32x4 {
  ($($shift_type:ty),+ $(,)?) => {
    $(impl Shl<$shift_type> for u32x4 {
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
            Self { simd: u32x4_shl(self.simd, rhs as u32) }
          } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
            unsafe {Self { neon: vshlq_u32(self.neon, vmovq_n_s32(rhs as i32)) }}
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
impl_shl_t_for_u32x4!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

macro_rules! impl_shr_t_for_u32x4 {
  ($($shift_type:ty),+ $(,)?) => {
    $(impl Shr<$shift_type> for u32x4 {
      type Output = Self;
      /// Shifts all lanes by the value given.
      #[inline]
      #[must_use]
      fn shr(self, rhs: $shift_type) -> Self::Output {
        pick! {
          if #[cfg(target_feature="sse2")] {
            let shift = cast([rhs as u64, 0]);
            Self { sse: shr_all_u32_m128i(self.sse, shift) }
          } else if #[cfg(target_feature="simd128")] {
            Self { simd: u32x4_shr(self.simd, rhs as u32) }
          } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
            unsafe {Self { neon: vshlq_u32(self.neon, vmovq_n_s32( -(rhs as i32))) }}
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
impl_shr_t_for_u32x4!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

/// Shifts lanes by the corresponding lane.
///
/// Bitwise shift-right; yields `self >> mask(rhs)`, where mask removes any
/// high-order bits of `rhs` that would cause the shift to exceed the bitwidth
/// of the type. (same as `wrapping_shr`)
impl Shr<u32x4> for u32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn shr(self, rhs: u32x4) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        // mask the shift count to 31 to have same behavior on all platforms
        let shift_by = bitand_m128i(rhs.sse, set_splat_i32_m128i(31));
        Self { sse: shr_each_u32_m128i(self.sse, shift_by) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {
          // mask the shift count to 31 to have same behavior on all platforms
          // no right shift, have to pass negative value to left shift on neon
          let shift_by = vnegq_s32(vreinterpretq_s32_u32(vandq_u32(rhs.neon, vmovq_n_u32(31))));
          Self { neon: vshlq_u32(self.neon, shift_by) }
        }
      } else {
        let arr: [u32; 4] = cast(self);
        let rhs: [u32; 4] = cast(rhs);
        cast([
          arr[0].wrapping_shr(rhs[0]),
          arr[1].wrapping_shr(rhs[1]),
          arr[2].wrapping_shr(rhs[2]),
          arr[3].wrapping_shr(rhs[3]),
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
impl Shl<u32x4> for u32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn shl(self, rhs: u32x4) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        // mask the shift count to 31 to have same behavior on all platforms
        let shift_by = bitand_m128i(rhs.sse, set_splat_i32_m128i(31));
        Self { sse: shl_each_u32_m128i(self.sse, shift_by) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {
          // mask the shift count to 31 to have same behavior on all platforms
          let shift_by = vreinterpretq_s32_u32(vandq_u32(rhs.neon, vmovq_n_u32(31)));
          Self { neon: vshlq_u32(self.neon, shift_by) }
        }
      } else {
        let arr: [u32; 4] = cast(self);
        let rhs: [u32; 4] = cast(rhs);
        cast([
          arr[0].wrapping_shl(rhs[0]),
          arr[1].wrapping_shl(rhs[1]),
          arr[2].wrapping_shl(rhs[2]),
          arr[3].wrapping_shl(rhs[3]),
        ])
      }
    }
  }
}

impl CmpEq for u32x4 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_eq(self, rhs: Self) -> Self::Output {
    Self::cmp_eq(self, rhs)
  }
}

impl u32x4 {
  #[inline]
  #[must_use]
  pub const fn new(array: [u32; 4]) -> Self {
    unsafe { core::mem::transmute(array) }
  }
  #[inline]
  #[must_use]
  pub fn cmp_eq(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        Self { sse: cmp_eq_mask_i32_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u32x4_eq(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vceqq_u32(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          if self.arr[0] == rhs.arr[0] { u32::MAX } else { 0 },
          if self.arr[1] == rhs.arr[1] { u32::MAX } else { 0 },
          if self.arr[2] == rhs.arr[2] { u32::MAX } else { 0 },
          if self.arr[3] == rhs.arr[3] { u32::MAX } else { 0 },
        ]}
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn cmp_gt(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse2")] {
        // no unsigned less than so inverting the high bit will get the correct result
        let h = u32x4::splat(1 << 31);
        Self { sse: cmp_gt_mask_i32_m128i((self ^ h).sse, (rhs ^ h).sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u32x4_gt(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
        unsafe {Self { neon: vcgtq_u32(self.neon, rhs.neon) }}
      } else {
        Self { arr: [
          if self.arr[0] > rhs.arr[0] { u32::MAX } else { 0 },
          if self.arr[1] > rhs.arr[1] { u32::MAX } else { 0 },
          if self.arr[2] > rhs.arr[2] { u32::MAX } else { 0 },
          if self.arr[3] > rhs.arr[3] { u32::MAX } else { 0 },
        ]}
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn cmp_lt(self, rhs: Self) -> Self {
    // lt is just gt the other way around
    rhs.cmp_gt(self)
  }

  /// Multiplies 32x32 bit to 64 bit and then only keeps the high 32 bits of the
  /// result. Useful for implementing divide constant value (see t_usefulness
  /// example)
  #[inline]
  #[must_use]
  pub fn mul_keep_high(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        let a = convert_to_i64_m256i_from_u32_m128i(self.sse);
        let b = convert_to_i64_m256i_from_u32_m128i(rhs.sse);
        let r = mul_u64_low_bits_m256i(a, b);

        // the compiler does a good job shuffling the lanes around
        let b : [u32;8] = cast(r);
        cast([b[1],b[3],b[5],b[7]])
      } else if #[cfg(target_feature="sse2")] {
        let evenp = mul_widen_u32_odd_m128i(self.sse, rhs.sse);

        let oddp = mul_widen_u32_odd_m128i(
          shr_imm_u64_m128i::<32>(self.sse),
          shr_imm_u64_m128i::<32>(rhs.sse));

        // the compiler does a good job shuffling the lanes around
        let a : [u32;4]= cast(evenp);
        let b : [u32;4]= cast(oddp);
        cast([a[1],b[1],a[3],b[3]])

      } else if #[cfg(target_feature="simd128")] {
        let low =  u64x2_extmul_low_u32x4(self.simd, rhs.simd);
        let high = u64x2_extmul_high_u32x4(self.simd, rhs.simd);

        Self { simd: u32x4_shuffle::<1, 3, 5, 7>(low, high) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
        unsafe {
          let l = vmull_u32(vget_low_u32(self.neon), vget_low_u32(rhs.neon));
          let h = vmull_u32(vget_high_u32(self.neon), vget_high_u32(rhs.neon));
          u32x4 { neon: vcombine_u32(vshrn_n_u64(l,32), vshrn_n_u64(h,32)) }
        }
      } else {
        let a: [u32; 4] = cast(self);
        let b: [u32; 4] = cast(rhs);
        cast([
          ((u64::from(a[0]) * u64::from(b[0])) >> 32) as u32,
          ((u64::from(a[1]) * u64::from(b[1])) >> 32) as u32,
          ((u64::from(a[2]) * u64::from(b[2])) >> 32) as u32,
          ((u64::from(a[3]) * u64::from(b[3])) >> 32) as u32,
        ])
      }
    }
  }

  /// Multiplies corresponding 32 bit lanes and returns the 64 bit result
  /// on the corresponding lanes.
  ///
  /// Effectively does two multiplies on 128 bit platforms, but is easier
  /// to use than wrapping mul_widen_u32_odd_m128i individually.
  #[inline]
  #[must_use]
  pub fn mul_widen(self, rhs: Self) -> u64x4 {
    pick! {
      if #[cfg(target_feature="avx2")] {
        // ok to sign extend since we are throwing away the high half of the result anyway
        let a = convert_to_i64_m256i_from_i32_m128i(self.sse);
        let b = convert_to_i64_m256i_from_i32_m128i(rhs.sse);
        cast(mul_u64_low_bits_m256i(a, b))
      } else if #[cfg(target_feature="sse2")] {
        let evenp = mul_widen_u32_odd_m128i(self.sse, rhs.sse);

        let oddp = mul_widen_u32_odd_m128i(
          shr_imm_u64_m128i::<32>(self.sse),
          shr_imm_u64_m128i::<32>(rhs.sse));

        u64x4 {
          a: u64x2 { sse: unpack_low_i64_m128i(evenp, oddp)},
          b: u64x2 { sse: unpack_high_i64_m128i(evenp, oddp)}
        }
      } else if #[cfg(target_feature="simd128")] {
        u64x4 {
          a: u64x2 { simd: u64x2_extmul_low_u32x4(self.simd, rhs.simd) },
          b: u64x2 { simd: u64x2_extmul_high_u32x4(self.simd, rhs.simd) },
        }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))] {
      unsafe {
        u64x4 { a: u64x2 { neon: vmull_u32(vget_low_u32(self.neon), vget_low_u32(rhs.neon)) },
                b: u64x2 { neon: vmull_u32(vget_high_u32(self.neon), vget_high_u32(rhs.neon)) } }
        }
      } else {
        let a: [u32; 4] = cast(self);
        let b: [u32; 4] = cast(rhs);
        cast([
          u64::from(a[0]) * u64::from(b[0]),
          u64::from(a[1]) * u64::from(b[1]),
          u64::from(a[2]) * u64::from(b[2]),
          u64::from(a[3]) * u64::from(b[3]),
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
        unsafe {Self { neon: vbslq_u32(self.neon, t.neon, f.neon) }}
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
        Self { sse: max_u32_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u32x4_max(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vmaxq_u32(self.neon, rhs.neon) }}
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vmaxq_u16(self.neon, rhs.neon) }}
      } else {
        let arr: [u32; 4] = cast(self);
        let rhs: [u32; 4] = cast(rhs);
        cast([
          arr[0].max(rhs[0]),
          arr[1].max(rhs[1]),
          arr[2].max(rhs[2]),
          arr[3].max(rhs[3]),
        ])
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn min(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="sse4.1")] {
        Self { sse: min_u32_m128i(self.sse, rhs.sse) }
      } else if #[cfg(target_feature="simd128")] {
        Self { simd: u32x4_min(self.simd, rhs.simd) }
      } else if #[cfg(all(target_feature="neon",target_arch="aarch64"))]{
        unsafe {Self { neon: vminq_u32(self.neon, rhs.neon) }}
      } else {
        let arr: [u32; 4] = cast(self);
        let rhs: [u32; 4] = cast(rhs);
        cast([
          arr[0].min(rhs[0]),
          arr[1].min(rhs[1]),
          arr[2].min(rhs[2]),
          arr[3].min(rhs[3]),
        ])
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn any(self) -> bool {
    pick! {
      if #[cfg(target_feature="sse2")] {
        (move_mask_i8_m128i(self.sse) & 0b1000100010001000) != 0
      } else if #[cfg(target_feature="simd128")] {
        u32x4_bitmask(self.simd) != 0
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
        (move_mask_i8_m128i(self.sse) & 0b1000100010001000) == 0b1000100010001000
      } else if #[cfg(target_feature="simd128")] {
        u32x4_bitmask(self.simd) == 0b1111
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

  /// Transpose matrix of 4x4 `u32` matrix. Currently only accelerated on SSE.
  #[must_use]
  #[inline]
  pub fn transpose(data: [u32x4; 4]) -> [u32x4; 4] {
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
        fn transpose_column(data: &[u32x4; 4], index: usize) -> u32x4 {
          u32x4::new([
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
  pub fn to_array(self) -> [u32; 4] {
    cast(self)
  }

  #[inline]
  pub fn as_array_ref(&self) -> &[u32; 4] {
    cast_ref(self)
  }

  #[inline]
  pub fn as_array_mut(&mut self) -> &mut [u32; 4] {
    cast_mut(self)
  }
}
