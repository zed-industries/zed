use super::*;

pick! {
  if #[cfg(target_feature="avx")] {
    #[derive(Default, Clone, Copy, PartialEq)]
    #[repr(C, align(32))]
    pub struct f32x8 { avx: m256 }
  } else {
    #[derive(Default, Clone, Copy, PartialEq)]
    #[repr(C, align(32))]
    pub struct f32x8 { a : f32x4, b : f32x4 }
  }
}

macro_rules! const_f32_as_f32x8 {
  ($i:ident, $f:expr) => {
    #[allow(non_upper_case_globals)]
    pub const $i: f32x8 = f32x8::new([$f; 8]);
  };
}

impl f32x8 {
  const_f32_as_f32x8!(ONE, 1.0);
  const_f32_as_f32x8!(HALF, 0.5);
  const_f32_as_f32x8!(ZERO, 0.0);
  const_f32_as_f32x8!(E, core::f32::consts::E);
  const_f32_as_f32x8!(FRAC_1_PI, core::f32::consts::FRAC_1_PI);
  const_f32_as_f32x8!(FRAC_2_PI, core::f32::consts::FRAC_2_PI);
  const_f32_as_f32x8!(FRAC_2_SQRT_PI, core::f32::consts::FRAC_2_SQRT_PI);
  const_f32_as_f32x8!(FRAC_1_SQRT_2, core::f32::consts::FRAC_1_SQRT_2);
  const_f32_as_f32x8!(FRAC_PI_2, core::f32::consts::FRAC_PI_2);
  const_f32_as_f32x8!(FRAC_PI_3, core::f32::consts::FRAC_PI_3);
  const_f32_as_f32x8!(FRAC_PI_4, core::f32::consts::FRAC_PI_4);
  const_f32_as_f32x8!(FRAC_PI_6, core::f32::consts::FRAC_PI_6);
  const_f32_as_f32x8!(FRAC_PI_8, core::f32::consts::FRAC_PI_8);
  const_f32_as_f32x8!(LN_2, core::f32::consts::LN_2);
  const_f32_as_f32x8!(LN_10, core::f32::consts::LN_10);
  const_f32_as_f32x8!(LOG2_E, core::f32::consts::LOG2_E);
  const_f32_as_f32x8!(LOG10_E, core::f32::consts::LOG10_E);
  const_f32_as_f32x8!(LOG10_2, core::f32::consts::LOG10_2);
  const_f32_as_f32x8!(LOG2_10, core::f32::consts::LOG2_10);
  const_f32_as_f32x8!(PI, core::f32::consts::PI);
  const_f32_as_f32x8!(SQRT_2, core::f32::consts::SQRT_2);
  const_f32_as_f32x8!(TAU, core::f32::consts::TAU);
}

unsafe impl Zeroable for f32x8 {}
unsafe impl Pod for f32x8 {}

impl Add for f32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: add_m256(self.avx, rhs.avx) }
      } else {
        Self {
          a : self.a.add(rhs.a),
          b : self.b.add(rhs.b),
        }
      }
    }
  }
}

impl Sub for f32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: sub_m256(self.avx, rhs.avx) }
      } else {
        Self {
          a : self.a.sub(rhs.a),
          b : self.b.sub(rhs.b),
        }
      }
    }
  }
}

impl Mul for f32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: mul_m256(self.avx, rhs.avx) }
      } else {
        Self {
          a : self.a.mul(rhs.a),
          b : self.b.mul(rhs.b),
        }
      }
    }
  }
}

impl Div for f32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn div(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: div_m256(self.avx, rhs.avx) }
      } else {
        Self {
          a : self.a.div(rhs.a),
          b : self.b.div(rhs.b),
        }
      }
    }
  }
}

impl Add<f32> for f32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: f32) -> Self::Output {
    self.add(Self::splat(rhs))
  }
}

impl Sub<f32> for f32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: f32) -> Self::Output {
    self.sub(Self::splat(rhs))
  }
}

impl Mul<f32> for f32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: f32) -> Self::Output {
    self.mul(Self::splat(rhs))
  }
}

impl Div<f32> for f32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn div(self, rhs: f32) -> Self::Output {
    self.div(Self::splat(rhs))
  }
}

impl Add<f32x8> for f32 {
  type Output = f32x8;
  #[inline]
  #[must_use]
  fn add(self, rhs: f32x8) -> Self::Output {
    f32x8::splat(self).add(rhs)
  }
}

impl Sub<f32x8> for f32 {
  type Output = f32x8;
  #[inline]
  #[must_use]
  fn sub(self, rhs: f32x8) -> Self::Output {
    f32x8::splat(self).sub(rhs)
  }
}

impl Mul<f32x8> for f32 {
  type Output = f32x8;
  #[inline]
  #[must_use]
  fn mul(self, rhs: f32x8) -> Self::Output {
    f32x8::splat(self).mul(rhs)
  }
}

impl Div<f32x8> for f32 {
  type Output = f32x8;
  #[inline]
  #[must_use]
  fn div(self, rhs: f32x8) -> Self::Output {
    f32x8::splat(self).div(rhs)
  }
}

impl BitAnd for f32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitand(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: bitand_m256(self.avx, rhs.avx) }
      } else {
        Self {
          a : self.a.bitand(rhs.a),
          b : self.b.bitand(rhs.b),
        }
      }
    }
  }
}

impl BitOr for f32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitor(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: bitor_m256(self.avx, rhs.avx) }
      } else {
        Self {
          a : self.a.bitor(rhs.a),
          b : self.b.bitor(rhs.b),
        }
      }
    }
  }
}

impl BitXor for f32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn bitxor(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: bitxor_m256(self.avx, rhs.avx) }
      } else {
        Self {
          a : self.a.bitxor(rhs.a),
          b : self.b.bitxor(rhs.b),
        }
      }
    }
  }
}

impl CmpEq for f32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_eq(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: cmp_op_mask_m256::<{cmp_op!(EqualOrdered)}>(self.avx, rhs.avx) }
      } else {
        Self {
          a : self.a.cmp_eq(rhs.a),
          b : self.b.cmp_eq(rhs.b),
        }
      }
    }
  }
}

impl CmpGe for f32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_ge(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: cmp_op_mask_m256::<{cmp_op!(GreaterEqualOrdered)}>(self.avx, rhs.avx) }
      } else {
        Self {
          a : self.a.cmp_ge(rhs.a),
          b : self.b.cmp_ge(rhs.b),
        }
      }
    }
  }
}

impl CmpGt for f32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_gt(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: cmp_op_mask_m256::<{cmp_op!(GreaterThanOrdered)}>(self.avx, rhs.avx) }
      } else {
        Self {
          a : self.a.cmp_gt(rhs.a),
          b : self.b.cmp_gt(rhs.b),
        }
      }
    }
  }
}

impl CmpNe for f32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_ne(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: cmp_op_mask_m256::<{cmp_op!(NotEqualOrdered)}>(self.avx, rhs.avx) }
      } else {
        Self {
          a : self.a.cmp_ne(rhs.a),
          b : self.b.cmp_ne(rhs.b),
        }
      }
    }
  }
}

impl CmpLe for f32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_le(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: cmp_op_mask_m256::<{cmp_op!(LessEqualOrdered)}>(self.avx, rhs.avx) }
      } else {
        Self {
          a : self.a.cmp_le(rhs.a),
          b : self.b.cmp_le(rhs.b),
        }
      }
    }
  }
}

impl CmpLt for f32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_lt(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: cmp_op_mask_m256::<{cmp_op!(LessThanOrdered)}>(self.avx, rhs.avx) }
      } else {
        Self {
          a : self.a.cmp_lt(rhs.a),
          b : self.b.cmp_lt(rhs.b),
        }
      }
    }
  }
}

impl f32x8 {
  #[inline]
  #[must_use]
  pub const fn new(array: [f32; 8]) -> Self {
    unsafe { core::mem::transmute(array) }
  }
  #[inline]
  #[must_use]
  pub fn blend(self, t: Self, f: Self) -> Self {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: blend_varying_m256(f.avx, t.avx, self.avx) }
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
      if #[cfg(target_feature="avx")] {
        let non_sign_bits = f32x8::from(f32::from_bits(i32::MAX as u32));
        self & non_sign_bits
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
  pub fn floor(self) -> Self {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: floor_m256(self.avx) }
      } else {
        Self {
          a : self.a.floor(),
          b : self.b.floor(),
        }
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn ceil(self) -> Self {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: ceil_m256(self.avx) }
      } else {
        Self {
          a : self.a.ceil(),
          b : self.b.ceil(),
        }
      }
    }
  }

  /// Calculates the lanewise maximum of both vectors. This is a faster
  /// implementation than `max`, but it doesn't specify any behavior if NaNs are
  /// involved.
  #[inline]
  #[must_use]
  pub fn fast_max(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: max_m256(self.avx, rhs.avx) }
      } else {
        Self {
          a : self.a.fast_max(rhs.a),
          b : self.b.fast_max(rhs.b),
        }
      }
    }
  }

  /// Calculates the lanewise maximum of both vectors. This doesn't match
  /// IEEE-754 and instead is defined as `self < rhs ? rhs : self`.
  #[inline]
  #[must_use]
  pub fn max(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="avx")] {
        // max_m256 seems to do rhs < self ? self : rhs. So if there's any NaN
        // involved, it chooses rhs, so we need to specifically check rhs for
        // NaN.
        rhs.is_nan().blend(self, Self { avx: max_m256(self.avx, rhs.avx) })
      } else {
        Self {
          a : self.a.max(rhs.a),
          b : self.b.max(rhs.b),
        }
      }

    }
  }

  /// Calculates the lanewise minimum of both vectors. This is a faster
  /// implementation than `min`, but it doesn't specify any behavior if NaNs are
  /// involved.
  #[inline]
  #[must_use]
  pub fn fast_min(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: min_m256(self.avx, rhs.avx) }
      } else {
        Self {
          a : self.a.fast_min(rhs.a),
          b : self.b.fast_min(rhs.b),
        }
      }
    }
  }

  /// Calculates the lanewise minimum of both vectors. If either lane is NaN,
  /// the other lane gets chosen. Use `fast_min` for a faster implementation
  /// that doesn't handle NaNs.
  #[inline]
  #[must_use]
  pub fn min(self, rhs: Self) -> Self {
    pick! {
      if #[cfg(target_feature="avx")] {
        // min_m256 seems to do rhs > self ? self : rhs. So if there's any NaN
        // involved, it chooses rhs, so we need to specifically check rhs for
        // NaN.
        rhs.is_nan().blend(self, Self { avx: min_m256(self.avx, rhs.avx) })
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
  pub fn is_nan(self) -> Self {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: cmp_op_mask_m256::<{cmp_op!(Unordered)}>(self.avx, self.avx) }
      } else {
        Self {
          a : self.a.is_nan(),
          b : self.b.is_nan(),
        }
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn is_finite(self) -> Self {
    let shifted_exp_mask = u32x8::from(0xFF000000);
    let u: u32x8 = cast(self);
    let shift_u = u << 1_u64;
    let out = !(shift_u & shifted_exp_mask).cmp_eq(shifted_exp_mask);
    cast(out)
  }
  #[inline]
  #[must_use]
  pub fn is_inf(self) -> Self {
    let shifted_inf = u32x8::from(0xFF000000);
    let u: u32x8 = cast(self);
    let shift_u = u << 1_u64;
    let out = (shift_u).cmp_eq(shifted_inf);
    cast(out)
  }

  #[inline]
  #[must_use]
  pub fn round(self) -> Self {
    pick! {
      // NOTE: Is there an SSE2 version of this? f32x4 version probably translates but I've not had time to figure it out
      if #[cfg(target_feature="avx")] {
        Self { avx: round_m256::<{round_op!(Nearest)}>(self.avx) }
      } else {
        Self {
          a : self.a.round(),
          b : self.b.round(),
        }
      }
    }
  }

  /// Rounds each lane into an integer. This is a faster implementation than
  /// `round_int`, but it doesn't handle out of range values or NaNs. For those
  /// values you get implementation defined behavior.
  #[inline]
  #[must_use]
  pub fn fast_round_int(self) -> i32x8 {
    pick! {
      if #[cfg(target_feature="avx")] {
        cast(convert_to_i32_m256i_from_m256(self.avx))
      } else {
        cast([
          self.a.fast_round_int(),
          self.b.fast_round_int()])
      }
    }
  }

  /// Rounds each lane into an integer. This saturates out of range values and
  /// turns NaNs into 0. Use `fast_round_int` for a faster implementation that
  /// doesn't handle out of range values or NaNs.
  #[inline]
  #[must_use]
  pub fn round_int(self) -> i32x8 {
    pick! {
      if #[cfg(target_feature="avx")] {
        // Based on: https://github.com/v8/v8/blob/210987a552a2bf2a854b0baa9588a5959ff3979d/src/codegen/shared-ia32-x64/macro-assembler-shared-ia32-x64.h#L489-L504
        let non_nan_mask = self.cmp_eq(self);
        let non_nan = self & non_nan_mask;
        let flip_to_max: i32x8 = cast(self.cmp_ge(Self::splat(2147483648.0)));
        let cast: i32x8 = cast(convert_to_i32_m256i_from_m256(non_nan.avx));
        flip_to_max ^ cast
      } else {
        cast([
          self.a.round_int(),
          self.b.round_int(),
        ])
      }
    }
  }

  /// Truncates each lane into an integer. This is a faster implementation than
  /// `trunc_int`, but it doesn't handle out of range values or NaNs. For those
  /// values you get implementation defined behavior.
  #[inline]
  #[must_use]
  pub fn fast_trunc_int(self) -> i32x8 {
    pick! {
      if #[cfg(all(target_feature="avx"))] {
        cast(convert_truncate_to_i32_m256i_from_m256(self.avx))
      } else {
        cast([
          self.a.fast_trunc_int(),
          self.b.fast_trunc_int(),
        ])
      }
    }
  }

  /// Truncates each lane into an integer. This saturates out of range values
  /// and turns NaNs into 0. Use `fast_trunc_int` for a faster implementation
  /// that doesn't handle out of range values or NaNs.
  #[inline]
  #[must_use]
  pub fn trunc_int(self) -> i32x8 {
    pick! {
        if #[cfg(target_feature="avx")] {
        // Based on: https://github.com/v8/v8/blob/210987a552a2bf2a854b0baa9588a5959ff3979d/src/codegen/shared-ia32-x64/macro-assembler-shared-ia32-x64.h#L489-L504
        let non_nan_mask = self.cmp_eq(self);
        let non_nan = self & non_nan_mask;
        let flip_to_max: i32x8 = cast(self.cmp_ge(Self::splat(2147483648.0)));
        let cast: i32x8 = cast(convert_truncate_to_i32_m256i_from_m256(non_nan.avx));
        flip_to_max ^ cast
      } else {
        cast([
          self.a.trunc_int(),
          self.b.trunc_int(),
        ])
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn mul_add(self, m: Self, a: Self) -> Self {
    pick! {
      if #[cfg(all(target_feature="avx",target_feature="fma"))] {
        Self { avx: fused_mul_add_m256(self.avx, m.avx, a.avx) }
      } else if #[cfg(target_feature="avx")] {
        // still want to use 256 bit ops
        (self * m) + a
      } else {
        Self {
          a : self.a.mul_add(m.a, a.a),
          b : self.b.mul_add(m.b, a.b),
        }
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn mul_sub(self, m: Self, a: Self) -> Self {
    pick! {
      if #[cfg(all(target_feature="avx",target_feature="fma"))] {
        Self { avx: fused_mul_sub_m256(self.avx, m.avx, a.avx) }
      } else if #[cfg(target_feature="avx")] {
        // still want to use 256 bit ops
        (self * m) - a
      } else {
        Self {
          a : self.a.mul_sub(m.a, a.a),
          b : self.b.mul_sub(m.b, a.b),
        }
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn mul_neg_add(self, m: Self, a: Self) -> Self {
    pick! {
      if #[cfg(all(target_feature="avx",target_feature="fma"))] {
        Self { avx: fused_mul_neg_add_m256(self.avx, m.avx, a.avx) }
      } else if #[cfg(target_feature="avx")] {
        // still want to use 256 bit ops
        a - (self * m)
      } else {
        Self {
          a : self.a.mul_neg_add(m.a, a.a),
          b : self.b.mul_neg_add(m.b, a.b),
        }
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn mul_neg_sub(self, m: Self, a: Self) -> Self {
    pick! {
      if #[cfg(all(target_feature="avx",target_feature="fma"))] {
        Self { avx: fused_mul_neg_sub_m256(self.avx, m.avx, a.avx) }
      } else if #[cfg(target_feature="avx")] {
        // still want to use 256 bit ops
        -(self * m) - a
      } else {
        Self {
          a : self.a.mul_neg_sub(m.a, a.a),
          b : self.b.mul_neg_sub(m.b, a.b),
        }
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn flip_signs(self, signs: Self) -> Self {
    self ^ (signs & Self::from(-0.0))
  }

  #[inline]
  #[must_use]
  pub fn copysign(self, sign: Self) -> Self {
    let magnitude_mask = Self::from(f32::from_bits(u32::MAX >> 1));
    (self & magnitude_mask) | (sign & Self::from(-0.0))
  }

  #[inline]
  pub fn asin_acos(self) -> (Self, Self) {
    // Based on the Agner Fog "vector class library":
    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h
    const_f32_as_f32x8!(P4asinf, 4.2163199048E-2);
    const_f32_as_f32x8!(P3asinf, 2.4181311049E-2);
    const_f32_as_f32x8!(P2asinf, 4.5470025998E-2);
    const_f32_as_f32x8!(P1asinf, 7.4953002686E-2);
    const_f32_as_f32x8!(P0asinf, 1.6666752422E-1);

    let xa = self.abs();
    let big = xa.cmp_ge(f32x8::splat(0.5));

    let x1 = f32x8::splat(0.5) * (f32x8::ONE - xa);
    let x2 = xa * xa;
    let x3 = big.blend(x1, x2);

    let xb = x1.sqrt();

    let x4 = big.blend(xb, xa);

    let z = polynomial_4!(x3, P0asinf, P1asinf, P2asinf, P3asinf, P4asinf);
    let z = z.mul_add(x3 * x4, x4);

    let z1 = z + z;

    // acos
    let z3 = self.cmp_lt(f32x8::ZERO).blend(f32x8::PI - z1, z1);
    let z4 = f32x8::FRAC_PI_2 - z.flip_signs(self);
    let acos = big.blend(z3, z4);

    // asin
    let z3 = f32x8::FRAC_PI_2 - z1;
    let asin = big.blend(z3, z);
    let asin = asin.flip_signs(self);

    (asin, acos)
  }

  #[inline]
  #[must_use]
  pub fn asin(self) -> Self {
    // Based on the Agner Fog "vector class library":
    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h
    const_f32_as_f32x8!(P4asinf, 4.2163199048E-2);
    const_f32_as_f32x8!(P3asinf, 2.4181311049E-2);
    const_f32_as_f32x8!(P2asinf, 4.5470025998E-2);
    const_f32_as_f32x8!(P1asinf, 7.4953002686E-2);
    const_f32_as_f32x8!(P0asinf, 1.6666752422E-1);

    let xa = self.abs();
    let big = xa.cmp_ge(f32x8::splat(0.5));

    let x1 = f32x8::splat(0.5) * (f32x8::ONE - xa);
    let x2 = xa * xa;
    let x3 = big.blend(x1, x2);

    let xb = x1.sqrt();

    let x4 = big.blend(xb, xa);

    let z = polynomial_4!(x3, P0asinf, P1asinf, P2asinf, P3asinf, P4asinf);
    let z = z.mul_add(x3 * x4, x4);

    let z1 = z + z;

    // asin
    let z3 = f32x8::FRAC_PI_2 - z1;
    let asin = big.blend(z3, z);
    let asin = asin.flip_signs(self);

    asin
  }

  #[inline]
  #[must_use]
  pub fn acos(self) -> Self {
    // Based on the Agner Fog "vector class library":
    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h
    const_f32_as_f32x8!(P4asinf, 4.2163199048E-2);
    const_f32_as_f32x8!(P3asinf, 2.4181311049E-2);
    const_f32_as_f32x8!(P2asinf, 4.5470025998E-2);
    const_f32_as_f32x8!(P1asinf, 7.4953002686E-2);
    const_f32_as_f32x8!(P0asinf, 1.6666752422E-1);

    let xa = self.abs();
    let big = xa.cmp_ge(f32x8::splat(0.5));

    let x1 = f32x8::splat(0.5) * (f32x8::ONE - xa);
    let x2 = xa * xa;
    let x3 = big.blend(x1, x2);

    let xb = x1.sqrt();

    let x4 = big.blend(xb, xa);

    let z = polynomial_4!(x3, P0asinf, P1asinf, P2asinf, P3asinf, P4asinf);
    let z = z.mul_add(x3 * x4, x4);

    let z1 = z + z;

    // acos
    let z3 = self.cmp_lt(f32x8::ZERO).blend(f32x8::PI - z1, z1);
    let z4 = f32x8::FRAC_PI_2 - z.flip_signs(self);
    let acos = big.blend(z3, z4);

    acos
  }

  #[inline]
  pub fn atan(self) -> Self {
    // Based on the Agner Fog "vector class library":
    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h
    const_f32_as_f32x8!(P3atanf, 8.05374449538E-2);
    const_f32_as_f32x8!(P2atanf, -1.38776856032E-1);
    const_f32_as_f32x8!(P1atanf, 1.99777106478E-1);
    const_f32_as_f32x8!(P0atanf, -3.33329491539E-1);

    let t = self.abs();

    // small:  z = t / 1.0;
    // medium: z = (t-1.0) / (t+1.0);
    // big:    z = -1.0 / t;
    let notsmal = t.cmp_ge(Self::SQRT_2 - Self::ONE);
    let notbig = t.cmp_le(Self::SQRT_2 + Self::ONE);

    let mut s = notbig.blend(Self::FRAC_PI_4, Self::FRAC_PI_2);
    s = notsmal & s;

    let mut a = notbig & t;
    a = notsmal.blend(a - Self::ONE, a);
    let mut b = notbig & Self::ONE;
    b = notsmal.blend(b + t, b);
    let z = a / b;

    let zz = z * z;

    // Taylor expansion
    let mut re = polynomial_3!(zz, P0atanf, P1atanf, P2atanf, P3atanf);
    re = re.mul_add(zz * z, z) + s;

    // get sign bit
    re = (self.sign_bit()).blend(-re, re);

    re
  }

  #[inline]
  pub fn atan2(self, x: Self) -> Self {
    // Based on the Agner Fog "vector class library":
    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h
    const_f32_as_f32x8!(P3atanf, 8.05374449538E-2);
    const_f32_as_f32x8!(P2atanf, -1.38776856032E-1);
    const_f32_as_f32x8!(P1atanf, 1.99777106478E-1);
    const_f32_as_f32x8!(P0atanf, -3.33329491539E-1);

    let y = self;

    // move in first octant
    let x1 = x.abs();
    let y1 = y.abs();
    let swapxy = y1.cmp_gt(x1);
    // swap x and y if y1 > x1
    let mut x2 = swapxy.blend(y1, x1);
    let mut y2 = swapxy.blend(x1, y1);

    // check for special case: x and y are both +/- INF
    let both_infinite = x.is_inf() & y.is_inf();
    if both_infinite.any() {
      let minus_one = -Self::ONE;
      x2 = both_infinite.blend(x2 & minus_one, x2);
      y2 = both_infinite.blend(y2 & minus_one, y2);
    }

    // x = y = 0 will produce NAN. No problem, fixed below
    let t = y2 / x2;

    // small:  z = t / 1.0;
    // medium: z = (t-1.0) / (t+1.0);
    let notsmal = t.cmp_ge(Self::SQRT_2 - Self::ONE);

    let a = notsmal.blend(t - Self::ONE, t);
    let b = notsmal.blend(t + Self::ONE, Self::ONE);
    let s = notsmal & Self::FRAC_PI_4;
    let z = a / b;

    let zz = z * z;

    // Taylor expansion
    let mut re = polynomial_3!(zz, P0atanf, P1atanf, P2atanf, P3atanf);
    re = re.mul_add(zz * z, z) + s;

    // move back in place
    re = swapxy.blend(Self::FRAC_PI_2 - re, re);
    re = ((x | y).cmp_eq(Self::ZERO)).blend(Self::ZERO, re);
    re = (x.sign_bit()).blend(Self::PI - re, re);

    // get sign bit
    re = (y.sign_bit()).blend(-re, re);

    re
  }

  #[inline]
  #[must_use]
  pub fn sin_cos(self) -> (Self, Self) {
    // Based on the Agner Fog "vector class library":
    // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h

    const_f32_as_f32x8!(DP1F, 0.78515625_f32 * 2.0);
    const_f32_as_f32x8!(DP2F, 2.4187564849853515625E-4_f32 * 2.0);
    const_f32_as_f32x8!(DP3F, 3.77489497744594108E-8_f32 * 2.0);

    const_f32_as_f32x8!(P0sinf, -1.6666654611E-1);
    const_f32_as_f32x8!(P1sinf, 8.3321608736E-3);
    const_f32_as_f32x8!(P2sinf, -1.9515295891E-4);

    const_f32_as_f32x8!(P0cosf, 4.166664568298827E-2);
    const_f32_as_f32x8!(P1cosf, -1.388731625493765E-3);
    const_f32_as_f32x8!(P2cosf, 2.443315711809948E-5);

    const_f32_as_f32x8!(TWO_OVER_PI, 2.0 / core::f32::consts::PI);

    let xa = self.abs();

    // Find quadrant
    let y = (xa * TWO_OVER_PI).round();
    let q: i32x8 = y.round_int();

    let x = y.mul_neg_add(DP3F, y.mul_neg_add(DP2F, y.mul_neg_add(DP1F, xa)));

    let x2 = x * x;
    let mut s = polynomial_2!(x2, P0sinf, P1sinf, P2sinf) * (x * x2) + x;
    let mut c = polynomial_2!(x2, P0cosf, P1cosf, P2cosf) * (x2 * x2)
      + f32x8::from(0.5).mul_neg_add(x2, f32x8::from(1.0));

    let swap = !(q & i32x8::from(1)).cmp_eq(i32x8::from(0));

    let mut overflow: f32x8 = cast(q.cmp_gt(i32x8::from(0x2000000)));
    overflow &= xa.is_finite();
    s = overflow.blend(f32x8::from(0.0), s);
    c = overflow.blend(f32x8::from(1.0), c);

    // calc sin
    let mut sin1 = cast::<_, f32x8>(swap).blend(c, s);
    let sign_sin: i32x8 = (q << 30) ^ cast::<_, i32x8>(self);
    sin1 = sin1.flip_signs(cast(sign_sin));

    // calc cos
    let mut cos1 = cast::<_, f32x8>(swap).blend(s, c);
    let sign_cos: i32x8 = ((q + i32x8::from(1)) & i32x8::from(2)) << 30;
    cos1 ^= cast::<_, f32x8>(sign_cos);

    (sin1, cos1)
  }
  #[inline]
  #[must_use]
  pub fn sin(self) -> Self {
    let (s, _) = self.sin_cos();
    s
  }
  #[inline]
  #[must_use]
  pub fn cos(self) -> Self {
    let (_, c) = self.sin_cos();
    c
  }
  #[inline]
  #[must_use]
  pub fn tan(self) -> Self {
    let (s, c) = self.sin_cos();
    s / c
  }
  #[inline]
  #[must_use]
  pub fn to_degrees(self) -> Self {
    const_f32_as_f32x8!(RAD_TO_DEG_RATIO, 180.0_f32 / core::f32::consts::PI);
    self * RAD_TO_DEG_RATIO
  }
  #[inline]
  #[must_use]
  pub fn to_radians(self) -> Self {
    const_f32_as_f32x8!(DEG_TO_RAD_RATIO, core::f32::consts::PI / 180.0_f32);
    self * DEG_TO_RAD_RATIO
  }
  #[inline]
  #[must_use]
  pub fn recip(self) -> Self {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: reciprocal_m256(self.avx) }
      } else {
        Self {
          a : self.a.recip(),
          b : self.b.recip(),
        }
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn recip_sqrt(self) -> Self {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: reciprocal_sqrt_m256(self.avx) }
      } else {
        Self {
          a : self.a.recip_sqrt(),
          b : self.b.recip_sqrt(),
        }
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn sqrt(self) -> Self {
    pick! {
      if #[cfg(target_feature="avx")] {
        Self { avx: sqrt_m256(self.avx) }
      } else {
        Self {
          a : self.a.sqrt(),
          b : self.b.sqrt(),
        }
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn move_mask(self) -> i32 {
    pick! {
      if #[cfg(target_feature="avx")] {
        move_mask_m256(self.avx)
      } else {
        (self.b.move_mask() << 4) | self.a.move_mask()
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn any(self) -> bool {
    pick! {
      if #[cfg(target_feature="avx")] {
        move_mask_m256(self.avx) != 0
      } else {
        self.a.any() || self.b.any()
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn all(self) -> bool {
    pick! {
      if #[cfg(target_feature="avx")] {
        move_mask_m256(self.avx) == 0b11111111
      } else {
        self.a.all() && self.b.all()
      }
    }
  }
  #[inline]
  #[must_use]
  pub fn none(self) -> bool {
    !self.any()
  }

  #[inline]
  fn vm_pow2n(self) -> Self {
    const_f32_as_f32x8!(pow2_23, 8388608.0);
    const_f32_as_f32x8!(bias, 127.0);
    let a = self + (bias + pow2_23);
    let c = cast::<_, i32x8>(a) << 23;
    cast::<_, f32x8>(c)
  }

  /// Calculate the exponent of a packed `f32x8`
  #[inline]
  #[must_use]
  pub fn exp(self) -> Self {
    const_f32_as_f32x8!(P0, 1.0 / 2.0);
    const_f32_as_f32x8!(P1, 1.0 / 6.0);
    const_f32_as_f32x8!(P2, 1. / 24.);
    const_f32_as_f32x8!(P3, 1. / 120.);
    const_f32_as_f32x8!(P4, 1. / 720.);
    const_f32_as_f32x8!(P5, 1. / 5040.);
    const_f32_as_f32x8!(LN2D_HI, 0.693359375);
    const_f32_as_f32x8!(LN2D_LO, -2.12194440e-4);
    let max_x = f32x8::from(87.3);
    let r = (self * Self::LOG2_E).round();
    let x = r.mul_neg_add(LN2D_HI, self);
    let x = r.mul_neg_add(LN2D_LO, x);
    let z = polynomial_5!(x, P0, P1, P2, P3, P4, P5);
    let x2 = x * x;
    let z = z.mul_add(x2, x);
    let n2 = Self::vm_pow2n(r);
    let z = (z + Self::ONE) * n2;
    // check for overflow
    let in_range = self.abs().cmp_lt(max_x);
    let in_range = in_range & self.is_finite();
    in_range.blend(z, Self::ZERO)
  }

  #[inline]
  fn exponent(self) -> f32x8 {
    const_f32_as_f32x8!(pow2_23, 8388608.0);
    const_f32_as_f32x8!(bias, 127.0);
    let a = cast::<_, u32x8>(self);
    let b = a >> 23;
    let c = b | cast::<_, u32x8>(pow2_23);
    let d = cast::<_, f32x8>(c);
    let e = d - (pow2_23 + bias);
    e
  }

  #[inline]
  fn fraction_2(self) -> Self {
    let t1 = cast::<_, u32x8>(self);
    let t2 = cast::<_, u32x8>(
      (t1 & u32x8::from(0x007FFFFF)) | u32x8::from(0x3F000000),
    );
    cast::<_, f32x8>(t2)
  }
  #[inline]
  fn is_zero_or_subnormal(self) -> Self {
    let t = cast::<_, i32x8>(self);
    let t = t & i32x8::splat(0x7F800000);
    i32x8::round_float(t.cmp_eq(i32x8::splat(0)))
  }
  #[inline]
  fn infinity() -> Self {
    cast::<_, f32x8>(i32x8::splat(0x7F800000))
  }
  #[inline]
  fn nan_log() -> Self {
    cast::<_, f32x8>(i32x8::splat(0x7FC00000 | 0x101 & 0x003FFFFF))
  }
  #[inline]
  fn nan_pow() -> Self {
    cast::<_, f32x8>(i32x8::splat(0x7FC00000 | 0x101 & 0x003FFFFF))
  }
  #[inline]
  pub fn sign_bit(self) -> Self {
    let t1 = cast::<_, i32x8>(self);
    let t2 = t1 >> 31;
    !cast::<_, f32x8>(t2).cmp_eq(f32x8::ZERO)
  }

  /// horizontal add of all the elements of the vector
  #[inline]
  #[must_use]
  pub fn reduce_add(self) -> f32 {
    pick! {
      // From https://stackoverflow.com/questions/13219146/how-to-sum-m256-horizontally
      if #[cfg(target_feature="avx")]{
        let hi_quad = extract_m128_from_m256::<1>(self.avx);
        let lo_quad = cast_to_m128_from_m256(self.avx);
        let sum_quad = add_m128(lo_quad,hi_quad);
        let lo_dual = sum_quad;
        let hi_dual = move_high_low_m128(sum_quad,sum_quad);
        let sum_dual = add_m128(lo_dual,hi_dual);
        let lo = sum_dual;
        let hi = shuffle_abi_f32_all_m128::<0b_01>(sum_dual, sum_dual);
        let sum = add_m128_s(lo, hi);
        get_f32_from_m128_s(sum)
      } else {
        self.a.reduce_add() + self.b.reduce_add()
      }
    }
  }

  /// Natural log (ln(x))
  #[inline]
  #[must_use]
  pub fn ln(self) -> Self {
    const_f32_as_f32x8!(HALF, 0.5);
    const_f32_as_f32x8!(P0, 3.3333331174E-1);
    const_f32_as_f32x8!(P1, -2.4999993993E-1);
    const_f32_as_f32x8!(P2, 2.0000714765E-1);
    const_f32_as_f32x8!(P3, -1.6668057665E-1);
    const_f32_as_f32x8!(P4, 1.4249322787E-1);
    const_f32_as_f32x8!(P5, -1.2420140846E-1);
    const_f32_as_f32x8!(P6, 1.1676998740E-1);
    const_f32_as_f32x8!(P7, -1.1514610310E-1);
    const_f32_as_f32x8!(P8, 7.0376836292E-2);
    const_f32_as_f32x8!(LN2F_HI, 0.693359375);
    const_f32_as_f32x8!(LN2F_LO, -2.12194440e-4);
    const_f32_as_f32x8!(VM_SMALLEST_NORMAL, 1.17549435E-38);

    let x1 = self;
    let x = Self::fraction_2(x1);
    let e = Self::exponent(x1);
    let mask = x.cmp_gt(Self::SQRT_2 * HALF);
    let x = (!mask).blend(x + x, x);
    let fe = mask.blend(e + Self::ONE, e);
    let x = x - Self::ONE;
    let res = polynomial_8!(x, P0, P1, P2, P3, P4, P5, P6, P7, P8);
    let x2 = x * x;
    let res = x2 * x * res;
    let res = fe.mul_add(LN2F_LO, res);
    let res = res + x2.mul_neg_add(HALF, x);
    let res = fe.mul_add(LN2F_HI, res);
    let overflow = !self.is_finite();
    let underflow = x1.cmp_lt(VM_SMALLEST_NORMAL);
    let mask = overflow | underflow;
    if !mask.any() {
      res
    } else {
      let is_zero = self.is_zero_or_subnormal();
      let res = underflow.blend(Self::nan_log(), res);
      let res = is_zero.blend(Self::infinity(), res);
      let res = overflow.blend(self, res);
      res
    }
  }

  #[inline]
  #[must_use]
  pub fn log2(self) -> Self {
    Self::ln(self) * Self::LOG2_E
  }
  #[inline]
  #[must_use]
  pub fn log10(self) -> Self {
    Self::ln(self) * Self::LOG10_E
  }

  #[inline]
  #[must_use]
  pub fn pow_f32x8(self, y: Self) -> Self {
    const_f32_as_f32x8!(ln2f_hi, 0.693359375);
    const_f32_as_f32x8!(ln2f_lo, -2.12194440e-4);
    const_f32_as_f32x8!(P0logf, 3.3333331174E-1);
    const_f32_as_f32x8!(P1logf, -2.4999993993E-1);
    const_f32_as_f32x8!(P2logf, 2.0000714765E-1);
    const_f32_as_f32x8!(P3logf, -1.6668057665E-1);
    const_f32_as_f32x8!(P4logf, 1.4249322787E-1);
    const_f32_as_f32x8!(P5logf, -1.2420140846E-1);
    const_f32_as_f32x8!(P6logf, 1.1676998740E-1);
    const_f32_as_f32x8!(P7logf, -1.1514610310E-1);
    const_f32_as_f32x8!(P8logf, 7.0376836292E-2);

    const_f32_as_f32x8!(p2expf, 1.0 / 2.0); // coefficients for Taylor expansion of exp
    const_f32_as_f32x8!(p3expf, 1.0 / 6.0);
    const_f32_as_f32x8!(p4expf, 1.0 / 24.0);
    const_f32_as_f32x8!(p5expf, 1.0 / 120.0);
    const_f32_as_f32x8!(p6expf, 1.0 / 720.0);
    const_f32_as_f32x8!(p7expf, 1.0 / 5040.0);

    let x1 = self.abs();
    let x = x1.fraction_2();
    let mask = x.cmp_gt(f32x8::SQRT_2 * f32x8::HALF);
    let x = (!mask).blend(x + x, x);

    let x = x - f32x8::ONE;
    let x2 = x * x;
    let lg1 = polynomial_8!(
      x, P0logf, P1logf, P2logf, P3logf, P4logf, P5logf, P6logf, P7logf, P8logf
    );
    let lg1 = lg1 * x2 * x;

    let ef = x1.exponent();
    let ef = mask.blend(ef + f32x8::ONE, ef);
    let e1 = (ef * y).round();
    let yr = ef.mul_sub(y, e1);

    let lg = f32x8::HALF.mul_neg_add(x2, x) + lg1;
    let x2_err = (f32x8::HALF * x).mul_sub(x, f32x8::HALF * x2);
    let lg_err = f32x8::HALF.mul_add(x2, lg - x) - lg1;

    let e2 = (lg * y * f32x8::LOG2_E).round();
    let v = lg.mul_sub(y, e2 * ln2f_hi);
    let v = e2.mul_neg_add(ln2f_lo, v);
    let v = v - (lg_err + x2_err).mul_sub(y, yr * f32x8::LN_2);

    let x = v;
    let e3 = (x * f32x8::LOG2_E).round();
    let x = e3.mul_neg_add(f32x8::LN_2, x);
    let x2 = x * x;
    let z = x2.mul_add(
      polynomial_5!(x, p2expf, p3expf, p4expf, p5expf, p6expf, p7expf),
      x + f32x8::ONE,
    );

    let ee = e1 + e2 + e3;
    let ei = cast::<_, i32x8>(ee.round_int());
    let ej = cast::<_, i32x8>(ei + (cast::<_, i32x8>(z) >> 23));

    let overflow = cast::<_, f32x8>(ej.cmp_gt(i32x8::splat(0x0FF)))
      | (ee.cmp_gt(f32x8::splat(300.0)));
    let underflow = cast::<_, f32x8>(ej.cmp_lt(i32x8::splat(0x000)))
      | (ee.cmp_lt(f32x8::splat(-300.0)));

    // Add exponent by integer addition
    let z = cast::<_, f32x8>(cast::<_, i32x8>(z) + (ei << 23));
    // Check for overflow/underflow
    let z = underflow.blend(f32x8::ZERO, z);
    let z = overflow.blend(Self::infinity(), z);

    // Check for self == 0
    let x_zero = self.is_zero_or_subnormal();
    let z = x_zero.blend(
      y.cmp_lt(f32x8::ZERO).blend(
        Self::infinity(),
        y.cmp_eq(f32x8::ZERO).blend(f32x8::ONE, f32x8::ZERO),
      ),
      z,
    );

    let x_sign = self.sign_bit();
    let z = if x_sign.any() {
      // Y into an integer
      let yi = y.cmp_eq(y.round());

      // Is y odd?
      let y_odd = cast::<_, i32x8>(y.round_int() << 31).round_float();

      let z1 =
        yi.blend(z | y_odd, self.cmp_eq(Self::ZERO).blend(z, Self::nan_pow()));

      x_sign.blend(z1, z)
    } else {
      z
    };

    let x_finite = self.is_finite();
    let y_finite = y.is_finite();
    let e_finite = ee.is_finite();
    if (x_finite & y_finite & (e_finite | x_zero)).all() {
      return z;
    }

    (self.is_nan() | y.is_nan()).blend(self + y, z)
  }
  #[inline]
  pub fn powf(self, y: f32) -> Self {
    Self::pow_f32x8(self, f32x8::splat(y))
  }

  /// Transpose matrix of 8x8 `f32` matrix. Currently only accelerated on AVX.
  #[must_use]
  #[inline]
  pub fn transpose(data: [f32x8; 8]) -> [f32x8; 8] {
    pick! {
      if #[cfg(target_feature="avx")] {
        let a0 = unpack_lo_m256(data[0].avx, data[1].avx);
        let a1 = unpack_hi_m256(data[0].avx, data[1].avx);
        let a2 = unpack_lo_m256(data[2].avx, data[3].avx);
        let a3 = unpack_hi_m256(data[2].avx, data[3].avx);
        let a4 = unpack_lo_m256(data[4].avx, data[5].avx);
        let a5 = unpack_hi_m256(data[4].avx, data[5].avx);
        let a6 = unpack_lo_m256(data[6].avx, data[7].avx);
        let a7 = unpack_hi_m256(data[6].avx, data[7].avx);

        pub const fn mm_shuffle(z: i32, y: i32, x: i32, w: i32) -> i32 {
          (z << 6) | (y << 4) | (x << 2) | w
        }

        const SHUFF_LO : i32 = mm_shuffle(1,0,1,0);
        const SHUFF_HI : i32 = mm_shuffle(3,2,3,2);

        // possible todo: intel performance manual suggests alternative with blend to avoid port 5 pressure
        // (since blend runs on a different port than shuffle)
        let b0 = shuffle_m256::<SHUFF_LO>(a0,a2);
        let b1 = shuffle_m256::<SHUFF_HI>(a0,a2);
        let b2 = shuffle_m256::<SHUFF_LO>(a1,a3);
        let b3 = shuffle_m256::<SHUFF_HI>(a1,a3);
        let b4 = shuffle_m256::<SHUFF_LO>(a4,a6);
        let b5 = shuffle_m256::<SHUFF_HI>(a4,a6);
        let b6 = shuffle_m256::<SHUFF_LO>(a5,a7);
        let b7 = shuffle_m256::<SHUFF_HI>(a5,a7);

        [
          f32x8 { avx: permute2z_m256::<0x20>(b0, b4) },
          f32x8 { avx: permute2z_m256::<0x20>(b1, b5) },
          f32x8 { avx: permute2z_m256::<0x20>(b2, b6) },
          f32x8 { avx: permute2z_m256::<0x20>(b3, b7) },
          f32x8 { avx: permute2z_m256::<0x31>(b0, b4) },
          f32x8 { avx: permute2z_m256::<0x31>(b1, b5) },
          f32x8 { avx: permute2z_m256::<0x31>(b2, b6) },
          f32x8 { avx: permute2z_m256::<0x31>(b3, b7) }
        ]
      } else {
        // possible todo: not sure that 128bit SIMD gives us a a lot of speedup here

        #[inline(always)]
        fn transpose_column(data: &[f32x8; 8], index: usize) -> f32x8 {
          f32x8::new([
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
  pub fn to_array(self) -> [f32; 8] {
    cast(self)
  }

  #[inline]
  pub fn as_array_ref(&self) -> &[f32; 8] {
    cast_ref(self)
  }

  #[inline]
  pub fn as_array_mut(&mut self) -> &mut [f32; 8] {
    cast_mut(self)
  }

  #[inline]
  pub fn from_i32x8(v: i32x8) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx: convert_to_m256_from_i32_m256i(v.avx2) }
      } else {
        Self::new([
            v.as_array_ref()[0] as f32,
            v.as_array_ref()[1] as f32,
            v.as_array_ref()[2] as f32,
            v.as_array_ref()[3] as f32,
            v.as_array_ref()[4] as f32,
            v.as_array_ref()[5] as f32,
            v.as_array_ref()[6] as f32,
            v.as_array_ref()[7] as f32,
          ])
      }
    }
  }
}

impl Not for f32x8 {
  type Output = Self;
  #[inline]
  fn not(self) -> Self {
    pick! {
      if #[cfg(target_feature="avx")] {
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
