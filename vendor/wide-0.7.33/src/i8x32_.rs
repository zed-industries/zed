use super::*;

pick! {
  if #[cfg(target_feature="avx2")] {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(32))]
    pub struct i8x32 { avx: m256i }
  } else {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(32))]
    pub struct i8x32 { a : i8x16, b : i8x16 }
  }
}

int_uint_consts!(i8, 32, i8x32, 256);

unsafe impl Zeroable for i8x32 {}
unsafe impl Pod for i8x32 {}

impl Add for i8x32 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx: add_i8_m256i(self.avx,rhs.avx) }
      } else {
        Self {
          a : self.a.add(rhs.a),
          b : self.b.add(rhs.b),
        }
      }
    }
  }
}

impl Sub for i8x32 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx: sub_i8_m256i(self.avx,rhs.avx) }
      } else {
        Self {
          a : self.a.sub(rhs.a),
          b : self.b.sub(rhs.b),
        }
      }
    }
  }
}

impl Add<i8> for i8x32 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: i8) -> Self::Output {
    self.add(Self::splat(rhs))
  }
}

impl Sub<i8> for i8x32 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: i8) -> Self::Output {
    self.sub(Self::splat(rhs))
  }
}

impl Add<i8x32> for i8 {
  type Output = i8x32;
  #[inline]
  #[must_use]
  fn add(self, rhs: i8x32) -> Self::Output {
    i8x32::splat(self).add(rhs)
  }
}

impl Sub<i8x32> for i8 {
  type Output = i8x32;
  #[inline]
  #[must_use]
  fn sub(self, rhs: i8x32) -> Self::Output {
    i8x32::splat(self).sub(rhs)
  }
}

impl BitAnd for i8x32 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitand(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
          Self { avx : bitand_m256i(self.avx,rhs.avx) }
      } else {
          Self {
            a : self.a.bitand(rhs.a),
            b : self.b.bitand(rhs.b),
          }
      }
    }
  }
}

impl BitOr for i8x32 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitor(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx : bitor_m256i(self.avx,rhs.avx) }
      } else {
        Self {
          a : self.a.bitor(rhs.a),
          b : self.b.bitor(rhs.b),
        }
      }
    }
  }
}

impl BitXor for i8x32 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitxor(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx : bitxor_m256i(self.avx,rhs.avx) }
      } else {
        Self {
          a : self.a.bitxor(rhs.a),
          b : self.b.bitxor(rhs.b),
        }
      }
    }
  }
}

impl CmpEq for i8x32 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_eq(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx : cmp_eq_mask_i8_m256i(self.avx,rhs.avx) }
      } else {
        Self {
          a : self.a.cmp_eq(rhs.a),
          b : self.b.cmp_eq(rhs.b),
        }
      }
    }
  }
}

impl CmpGt for i8x32 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_gt(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx : cmp_gt_mask_i8_m256i(self.avx,rhs.avx) }
      } else {
        Self {
          a : self.a.cmp_gt(rhs.a),
          b : self.b.cmp_gt(rhs.b),
        }
      }
    }
  }
}

impl CmpLt for i8x32 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_lt(self, rhs: Self) -> Self::Output {
    rhs.cmp_gt(self)
  }
}

impl Not for i8x32 {
  type Output = Self;
  #[inline]
  fn not(self) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx: self.avx.not()  }
      } else {
        Self {
          a : self.a.not(),
          b : self.b.not(),
        }
      }
    }
  }
}

impl i8x32 {
  #[inline]
  #[must_use]
  pub const fn new(array: [i8; 32]) -> Self {
    unsafe { core::mem::transmute(array) }
  }
  #[inline]
  #[must_use]
  pub fn blend(self, t: Self, f: Self) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx: blend_varying_i8_m256i(f.avx, t.avx, self.avx) }
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
  pub fn abs(self) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx: abs_i8_m256i(self.avx) }
      } else {
        Self {
          a : self.a.abs(),
          b : self.b.abs(),
        }
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn unsigned_abs(self) -> u8x32 {
    pick! {
      if #[cfg(target_feature="avx2")] {
        u8x32 { avx: abs_i8_m256i(self.avx) }
      } else {
        u8x32 {
          a : self.a.unsigned_abs(),
          b : self.b.unsigned_abs(),
        }
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn max(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx: max_i8_m256i(self.avx,rhs.avx) }
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
        Self { avx: min_i8_m256i(self.avx,rhs.avx) }
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
        Self { avx: add_saturating_i8_m256i(self.avx, rhs.avx) }
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
        Self { avx: sub_saturating_i8_m256i(self.avx, rhs.avx) }
      } else {
        Self {
          a : self.a.saturating_sub(rhs.a),
          b : self.b.saturating_sub(rhs.b),
        }
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn move_mask(self) -> i32 {
    pick! {
      if #[cfg(target_feature="avx2")] {
        move_mask_i8_m256i(self.avx)
      } else {
        self.a.move_mask() | (self.b.move_mask() << 16)
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn any(self) -> bool {
    pick! {
      if #[cfg(target_feature="avx2")] {
        move_mask_i8_m256i(self.avx) != 0
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
        move_mask_i8_m256i(self.avx) == -1
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

  /// Returns a new vector with lanes selected from the lanes of the first input
  /// vector a specified in the second input vector `rhs`.
  /// The indices i in range `[0, 15]` select the i-th element of `self`. For
  /// indices outside of the range the resulting lane is `0`.
  ///
  /// This note that is the equivalent of two parallel swizzle operations on the
  /// two halves of the vector, and the indexes each refer to the
  /// corresponding half.
  #[inline]
  pub fn swizzle_half(self, rhs: i8x32) -> i8x32 {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx: shuffle_av_i8z_half_m256i(self.avx, rhs.saturating_add(i8x32::splat(0x60)).avx) }
      } else {
          Self {
            a : self.a.swizzle(rhs.a),
            b : self.b.swizzle(rhs.b),
          }
      }
    }
  }

  /// Indices in the range `[0, 15]` will select the i-th element of `self`. If
  /// the high bit of any element of `rhs` is set (negative) then the
  /// corresponding output lane is guaranteed to be zero. Otherwise if the
  /// element of `rhs` is within the range `[32, 127]` then the output lane is
  /// either `0` or `self[rhs[i] % 16]` depending on the implementation.
  ///
  /// This is the equivalent to two parallel swizzle operations on the two
  /// halves of the vector, and the indexes each refer to their corresponding
  /// half.
  #[inline]
  pub fn swizzle_half_relaxed(self, rhs: i8x32) -> i8x32 {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx: shuffle_av_i8z_half_m256i(self.avx, rhs.avx) }
      } else {
        Self {
          a : self.a.swizzle_relaxed(rhs.a),
          b : self.b.swizzle_relaxed(rhs.b),
        }
      }
    }
  }

  #[inline]
  pub fn to_array(self) -> [i8; 32] {
    cast(self)
  }

  #[inline]
  pub fn as_array_ref(&self) -> &[i8; 32] {
    cast_ref(self)
  }

  #[inline]
  pub fn as_array_mut(&mut self) -> &mut [i8; 32] {
    cast_mut(self)
  }
}
