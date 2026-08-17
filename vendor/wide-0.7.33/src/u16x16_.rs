use super::*;

pick! {
  if #[cfg(target_feature="avx2")] {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(32))]
    pub struct u16x16 { pub(crate) avx2: m256i }
  } else {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(32))]
    pub struct u16x16 { pub(crate) a : u16x8, pub(crate) b : u16x8 }
  }
}

int_uint_consts!(u16, 16, u16x16, 256);

unsafe impl Zeroable for u16x16 {}
unsafe impl Pod for u16x16 {}

impl Add for u16x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: add_i16_m256i(self.avx2, rhs.avx2) }
      } else {
        Self {
          a : self.a.add(rhs.a),
          b : self.b.add(rhs.b),
        }
      }
    }
  }
}

impl Sub for u16x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: sub_i16_m256i(self.avx2, rhs.avx2) }
      } else {
        Self {
          a : self.a.sub(rhs.a),
          b : self.b.sub(rhs.b),
        }
      }
    }
  }
}

impl Add<u16> for u16x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: u16) -> Self::Output {
    self.add(Self::splat(rhs))
  }
}

impl Sub<u16> for u16x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: u16) -> Self::Output {
    self.sub(Self::splat(rhs))
  }
}

impl Add<u16x16> for u16 {
  type Output = u16x16;
  #[inline]
  #[must_use]
  fn add(self, rhs: u16x16) -> Self::Output {
    u16x16::splat(self).add(rhs)
  }
}

impl Sub<u16x16> for u16 {
  type Output = u16x16;
  #[inline]
  #[must_use]
  fn sub(self, rhs: u16x16) -> Self::Output {
    u16x16::splat(self).sub(rhs)
  }
}

impl BitAnd for u16x16 {
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

impl BitOr for u16x16 {
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

impl BitXor for u16x16 {
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

impl Not for u16x16 {
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

macro_rules! impl_shl_t_for_u16x16 {
  ($($shift_type:ty),+ $(,)?) => {
    $(impl Shl<$shift_type> for u16x16 {
      type Output = Self;
      /// Shifts all lanes by the value given.
      #[inline]
      #[must_use]
      fn shl(self, rhs: $shift_type) -> Self::Output {
        pick! {
          if #[cfg(target_feature="avx2")] {
            let shift = cast([rhs as u64, 0]);
            Self { avx2: shl_all_u16_m256i(self.avx2, shift) }
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
impl_shl_t_for_u16x16!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

macro_rules! impl_shr_t_for_u16x16 {
  ($($shift_type:ty),+ $(,)?) => {
    $(impl Shr<$shift_type> for u16x16 {
      type Output = Self;
      /// Shifts all lanes by the value given.
      #[inline]
      #[must_use]
      fn shr(self, rhs: $shift_type) -> Self::Output {
        pick! {
          if #[cfg(target_feature="avx2")] {
            let shift = cast([rhs as u64, 0]);
            Self { avx2: shr_all_u16_m256i(self.avx2, shift) }
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
impl_shr_t_for_u16x16!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

impl CmpEq for u16x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_eq(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: cmp_eq_mask_i16_m256i(self.avx2, rhs.avx2) }
      } else {
        Self {
          a : self.a.cmp_eq(rhs.a),
          b : self.b.cmp_eq(rhs.b),
        }
      }
    }
  }
}

impl Mul for u16x16 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        // non-widening multiplication is the same for unsigned and signed
        Self { avx2: mul_i16_keep_low_m256i(self.avx2, rhs.avx2) }
      } else {
        Self {
          a : self.a.mul(rhs.a),
          b : self.b.mul(rhs.b),
        }
      }
    }
  }
}

impl From<u8x16> for u16x16 {
  /// widens and sign extends to u16x16
  #[inline]
  #[must_use]
  fn from(v: u8x16) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        u16x16 { avx2:convert_to_i16_m256i_from_u8_m128i(v.sse) }
      } else if #[cfg(target_feature="sse2")] {
        u16x16 {
          a: u16x8 { sse: shr_imm_u16_m128i::<8>( unpack_low_i8_m128i(v.sse, v.sse)) },
          b: u16x8 { sse: shr_imm_u16_m128i::<8>( unpack_high_i8_m128i(v.sse, v.sse)) },
        }
      } else {

        u16x16::new([
          v.as_array_ref()[0] as u16,
          v.as_array_ref()[1] as u16,
          v.as_array_ref()[2] as u16,
          v.as_array_ref()[3] as u16,
          v.as_array_ref()[4] as u16,
          v.as_array_ref()[5] as u16,
          v.as_array_ref()[6] as u16,
          v.as_array_ref()[7] as u16,
          v.as_array_ref()[8] as u16,
          v.as_array_ref()[9] as u16,
          v.as_array_ref()[10] as u16,
          v.as_array_ref()[11] as u16,
          v.as_array_ref()[12] as u16,
          v.as_array_ref()[13] as u16,
          v.as_array_ref()[14] as u16,
          v.as_array_ref()[15] as u16,
          ])
      }
    }
  }
}

impl u16x16 {
  #[inline]
  #[must_use]
  pub const fn new(array: [u16; 16]) -> Self {
    unsafe { core::mem::transmute(array) }
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
        Self { avx2: max_u16_m256i(self.avx2, rhs.avx2) }
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
        Self { avx2: min_u16_m256i(self.avx2, rhs.avx2) }
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
  pub fn saturating_add(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: add_saturating_u16_m256i(self.avx2, rhs.avx2) }
      } else {
        Self {
          a : self.a.saturating_add(rhs.a),
          b : self.b.saturating_add(rhs.b),
        }
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn saturating_sub(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: sub_saturating_u16_m256i(self.avx2, rhs.avx2) }
      } else {
        Self {
          a : self.a.saturating_sub(rhs.a),
          b : self.b.saturating_sub(rhs.b),
        }
      }
    }
  }

  #[inline]
  pub fn to_array(self) -> [u16; 16] {
    cast(self)
  }

  #[inline]
  pub fn as_array_ref(&self) -> &[u16; 16] {
    cast_ref(self)
  }

  #[inline]
  pub fn as_array_mut(&mut self) -> &mut [u16; 16] {
    cast_mut(self)
  }
}
