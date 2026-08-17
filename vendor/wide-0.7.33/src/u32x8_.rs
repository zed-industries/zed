use super::*;

pick! {
  if #[cfg(target_feature="avx2")] {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(32))]
    pub struct u32x8 { pub(crate) avx2: m256i }
  } else {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(32))]
    pub struct u32x8 { pub(crate) a : u32x4, pub(crate) b : u32x4 }
  }
}

int_uint_consts!(u32, 8, u32x8, 256);

unsafe impl Zeroable for u32x8 {}
unsafe impl Pod for u32x8 {}

impl Add for u32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: add_i32_m256i(self.avx2, rhs.avx2) }
      } else {
        Self {
          a : self.a.add(rhs.a),
          b : self.b.add(rhs.b),
        }
      }
    }
  }
}

impl Sub for u32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: sub_i32_m256i(self.avx2, rhs.avx2) }
      } else {
        Self {
          a : self.a.sub(rhs.a),
          b : self.b.sub(rhs.b),
        }
      }
    }
  }
}

impl Mul for u32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: mul_i32_keep_low_m256i(self.avx2, rhs.avx2) }
      } else {
        Self {
          a : self.a.mul(rhs.a),
          b : self.b.mul(rhs.b),
        }
      }
    }
  }
}

impl BitAnd for u32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitand(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: bitand_m256i(self.avx2, rhs.avx2) }
      } else {
        Self {
          a : self.a.bitand(rhs.a),
          b : self.b.bitand(rhs.b),
        }
      }
    }
  }
}

impl BitOr for u32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitor(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: bitor_m256i(self.avx2, rhs.avx2) }
      } else {
        Self {
          a : self.a.bitor(rhs.a),
          b : self.b.bitor(rhs.b),
        }
      }
    }
  }
}

impl BitXor for u32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitxor(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: bitxor_m256i(self.avx2, rhs.avx2) }
      } else {
        Self {
          a : self.a.bitxor(rhs.a),
          b : self.b.bitxor(rhs.b),
        }
      }
    }
  }
}

impl From<u16x8> for u32x8 {
  /// widens and zero extends to u32x8
  #[inline]
  #[must_use]
  fn from(v: u16x8) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2:convert_to_i32_m256i_from_u16_m128i(v.sse) }
      } else if #[cfg(target_feature="sse2")] {
        Self {
          a: u32x4 { sse: shr_imm_u32_m128i::<16>( unpack_low_i16_m128i(v.sse, v.sse)) },
          b: u32x4 { sse: shr_imm_u32_m128i::<16>( unpack_high_i16_m128i(v.sse, v.sse)) },
        }
      } else {
        u32x8::new([
          u32::from(v.as_array_ref()[0]),
          u32::from(v.as_array_ref()[1]),
          u32::from(v.as_array_ref()[2]),
          u32::from(v.as_array_ref()[3]),
          u32::from(v.as_array_ref()[4]),
          u32::from(v.as_array_ref()[5]),
          u32::from(v.as_array_ref()[6]),
          u32::from(v.as_array_ref()[7]),
        ])
      }
    }
  }
}

macro_rules! impl_shl_t_for_u32x8 {
  ($($shift_type:ty),+ $(,)?) => {
    $(impl Shl<$shift_type> for u32x8 {
      type Output = Self;
      /// Shifts all lanes by the value given.
      #[inline]
      #[must_use]
      fn shl(self, rhs: $shift_type) -> Self::Output {
        pick! {
          if #[cfg(target_feature="avx2")] {
            let shift = cast([rhs as u64, 0]);
            Self { avx2: shl_all_u32_m256i(self.avx2, shift) }
          } else {
            Self {
              a : self.a.shl(rhs),
              b : self.b.shl(rhs),
            }
          }
        }
      }
    })+
  };
}
impl_shl_t_for_u32x8!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

macro_rules! impl_shr_t_for_u32x8 {
  ($($shift_type:ty),+ $(,)?) => {
    $(impl Shr<$shift_type> for u32x8 {
      type Output = Self;
      /// Shifts all lanes by the value given.
      #[inline]
      #[must_use]
      fn shr(self, rhs: $shift_type) -> Self::Output {
        pick! {
          if #[cfg(target_feature="avx2")] {
            let shift = cast([rhs as u64, 0]);
            Self { avx2: shr_all_u32_m256i(self.avx2, shift) }
          } else {
            Self {
              a : self.a.shr(rhs),
              b : self.b.shr(rhs),
            }
          }
        }
      }
    })+
  };
}

impl_shr_t_for_u32x8!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

/// Shifts lanes by the corresponding lane.
///
/// Bitwise shift-right; yields `self >> mask(rhs)`, where mask removes any
/// high-order bits of `rhs` that would cause the shift to exceed the bitwidth
/// of the type. (same as `wrapping_shr`)
impl Shr<u32x8> for u32x8 {
  type Output = Self;

  #[inline]
  #[must_use]
  fn shr(self, rhs: u32x8) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        // ensure same behavior as scalar wrapping_shr
        let shift_by = bitand_m256i(rhs.avx2, set_splat_i32_m256i(31));
        Self { avx2: shr_each_u32_m256i(self.avx2, shift_by ) }
      } else {
        Self {
          a : self.a.shr(rhs.a),
          b : self.b.shr(rhs.b),
        }
      }
    }
  }
}

/// Shifts lanes by the corresponding lane.
///
/// Bitwise shift-left; yields `self << mask(rhs)`, where mask removes any
/// high-order bits of `rhs` that would cause the shift to exceed the bitwidth
/// of the type. (same as `wrapping_shl`)
impl Shl<u32x8> for u32x8 {
  type Output = Self;

  #[inline]
  #[must_use]
  fn shl(self, rhs: u32x8) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        // ensure same behavior as scalar wrapping_shl
        let shift_by = bitand_m256i(rhs.avx2, set_splat_i32_m256i(31));
        Self { avx2: shl_each_u32_m256i(self.avx2, shift_by) }
      } else {
        Self {
          a : self.a.shl(rhs.a),
          b : self.b.shl(rhs.b),
        }
      }
    }
  }
}

impl CmpEq for u32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_eq(self, rhs: Self) -> Self::Output {
    Self::cmp_eq(self, rhs)
  }
}

impl u32x8 {
  #[inline]
  #[must_use]
  pub const fn new(array: [u32; 8]) -> Self {
    unsafe { core::mem::transmute(array) }
  }
  #[inline]
  #[must_use]
  pub fn cmp_eq(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: cmp_eq_mask_i32_m256i(self.avx2, rhs.avx2 ) }
      } else {
        Self {
          a : self.a.cmp_eq(rhs.a),
          b : self.b.cmp_eq(rhs.b),
        }
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn cmp_gt(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        // no unsigned gt than so inverting the high bit will get the correct result
        let highbit = u32x8::splat(1 << 31);
        Self { avx2: cmp_gt_mask_i32_m256i((self ^ highbit).avx2, (rhs ^ highbit).avx2 ) }
      } else {
        Self {
          a : self.a.cmp_gt(rhs.a),
          b : self.b.cmp_gt(rhs.b),
        }
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
  pub fn mul_keep_high(self, rhs: u32x8) -> u32x8 {
    pick! {
      if #[cfg(target_feature="avx2")] {
        let a : [u32;8]= cast(self);
        let b : [u32;8]= cast(rhs);

        // let the compiler shuffle the values around, it does the right thing
        let r1 : [u32;8] = cast(mul_u64_low_bits_m256i(cast([a[0], 0, a[1], 0, a[2], 0, a[3], 0]), cast([b[0], 0, b[1], 0, b[2], 0, b[3], 0])));
        let r2 : [u32;8] = cast(mul_u64_low_bits_m256i(cast([a[4], 0, a[5], 0, a[6], 0, a[7], 0]), cast([b[4], 0, b[5], 0, b[6], 0, b[7], 0])));

        cast([r1[1], r1[3], r1[5], r1[7], r2[1], r2[3], r2[5], r2[7]])
      } else {
        Self {
          a : self.a.mul_keep_high(rhs.a),
          b : self.b.mul_keep_high(rhs.b),
        }
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn blend(self, t: Self, f: Self) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: blend_varying_i8_m256i(f.avx2, t.avx2, self.avx2) }
      } else {
        Self {
          a : self.a.blend(t.a, f.a),
          b : self.b.blend(t.b, f.b),
        }
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn max(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: max_u32_m256i(self.avx2, rhs.avx2 ) }
      } else {
        Self {
          a : self.a.max(rhs.a),
          b : self.b.max(rhs.b),
        }
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn min(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: min_u32_m256i(self.avx2, rhs.avx2 ) }
      } else {
        Self {
          a : self.a.min(rhs.a),
          b : self.b.min(rhs.b),
        }
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn any(self) -> bool {
    pick! {
      if #[cfg(target_feature="avx2")] {
        ((move_mask_i8_m256i(self.avx2) as u32) & 0b10001000100010001000100010001000) != 0
      } else {
        (self.a | self.b).any()
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn all(self) -> bool {
    pick! {
      if #[cfg(target_feature="avx2")] {
        ((move_mask_i8_m256i(self.avx2) as u32) & 0b10001000100010001000100010001000) == 0b10001000100010001000100010001000
      } else {
        (self.a & self.b).all()
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn none(self) -> bool {
    !self.any()
  }

  #[inline]
  pub fn to_array(self) -> [u32; 8] {
    cast(self)
  }

  #[inline]
  pub fn as_array_ref(&self) -> &[u32; 8] {
    cast_ref(self)
  }

  #[inline]
  pub fn as_array_mut(&mut self) -> &mut [u32; 8] {
    cast_mut(self)
  }
}

impl Not for u32x8 {
  type Output = Self;
  #[inline]
  fn not(self) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: self.avx2.not()  }
      } else {
        Self {
          a : self.a.not(),
          b : self.b.not(),
        }
      }
    }
  }
}
