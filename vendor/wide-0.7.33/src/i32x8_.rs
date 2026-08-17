use super::*;

pick! {
  if #[cfg(target_feature="avx2")] {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(32))]
    pub struct i32x8 { pub(crate) avx2: m256i }
  } else {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[repr(C, align(32))]
    pub struct i32x8 { pub(crate) a : i32x4, pub(crate) b : i32x4}
  }
}

int_uint_consts!(i32, 8, i32x8, 256);

unsafe impl Zeroable for i32x8 {}
unsafe impl Pod for i32x8 {}

impl Add for i32x8 {
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

impl Sub for i32x8 {
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

impl Mul for i32x8 {
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

impl Add<i32> for i32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn add(self, rhs: i32) -> Self::Output {
    self.add(Self::splat(rhs))
  }
}

impl Sub<i32> for i32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn sub(self, rhs: i32) -> Self::Output {
    self.sub(Self::splat(rhs))
  }
}

impl Mul<i32> for i32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn mul(self, rhs: i32) -> Self::Output {
    self.mul(Self::splat(rhs))
  }
}

impl Add<i32x8> for i32 {
  type Output = i32x8;
  #[inline]
  #[must_use]
  fn add(self, rhs: i32x8) -> Self::Output {
    i32x8::splat(self) + rhs
  }
}

impl Sub<i32x8> for i32 {
  type Output = i32x8;
  #[inline]
  #[must_use]
  fn sub(self, rhs: i32x8) -> Self::Output {
    i32x8::splat(self) - rhs
  }
}

impl Mul<i32x8> for i32 {
  type Output = i32x8;
  #[inline]
  #[must_use]
  fn mul(self, rhs: i32x8) -> Self::Output {
    i32x8::splat(self) * rhs
  }
}

impl BitAnd for i32x8 {
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

impl BitOr for i32x8 {
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
    }    }
  }
}

impl BitXor for i32x8 {
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

macro_rules! impl_shl_t_for_i32x8 {
  ($($shift_type:ty),+ $(,)?) => {
    $(impl Shl<$shift_type> for i32x8 {
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
impl_shl_t_for_i32x8!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

macro_rules! impl_shr_t_for_i32x8 {
  ($($shift_type:ty),+ $(,)?) => {
    $(impl Shr<$shift_type> for i32x8 {
      type Output = Self;
      /// Shifts all lanes by the value given.
      #[inline]
      #[must_use]
      fn shr(self, rhs: $shift_type) -> Self::Output {
        pick! {
          if #[cfg(target_feature="avx2")] {
            let shift = cast([rhs as u64, 0]);
            Self { avx2: shr_all_i32_m256i(self.avx2, shift) }
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

impl_shr_t_for_i32x8!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

/// Shifts lanes by the corresponding lane.
///
/// Bitwise shift-right; yields `self >> mask(rhs)`, where mask removes any
/// high-order bits of `rhs` that would cause the shift to exceed the bitwidth
/// of the type. (same as `wrapping_shr`)
impl Shr<i32x8> for i32x8 {
  type Output = Self;

  #[inline]
  #[must_use]
  fn shr(self, rhs: i32x8) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        // ensure same behavior as scalar
        let shift_by = bitand_m256i(rhs.avx2, set_splat_i32_m256i(31));
        Self { avx2: shr_each_i32_m256i(self.avx2, shift_by ) }
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
impl Shl<i32x8> for i32x8 {
  type Output = Self;

  #[inline]
  #[must_use]
  fn shl(self, rhs: i32x8) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        // ensure same behavior as scalar wrapping_shl by masking the shift count
        let shift_by = bitand_m256i(rhs.avx2, set_splat_i32_m256i(31));
        // shl is the same for unsigned and signed
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

impl CmpEq for i32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_eq(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: cmp_eq_mask_i32_m256i(self.avx2, rhs.avx2) }
      } else {
        Self {
          a : self.a.cmp_eq(rhs.a),
          b : self.b.cmp_eq(rhs.b),
        }
      }
    }
  }
}

impl CmpGt for i32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_gt(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: cmp_gt_mask_i32_m256i(self.avx2, rhs.avx2) }
      } else {
        Self {
          a : self.a.cmp_gt(rhs.a),
          b : self.b.cmp_gt(rhs.b),
        }
      }
    }
  }
}

impl CmpLt for i32x8 {
  type Output = Self;
  #[inline]
  #[must_use]
  fn cmp_lt(self, rhs: Self) -> Self::Output {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: !cmp_gt_mask_i32_m256i(self.avx2, rhs.avx2)  ^ cmp_eq_mask_i32_m256i(self.avx2,rhs.avx2) }
      } else {
        Self {
          a : self.a.cmp_lt(rhs.a),
          b : self.b.cmp_lt(rhs.b),
        }
      }
    }
  }
}

impl From<i16x8> for i32x8 {
  #[inline]
  #[must_use]
  fn from(value: i16x8) -> Self {
    i32x8::from_i16x8(value)
  }
}

impl i32x8 {
  #[inline]
  #[must_use]
  pub const fn new(array: [i32; 8]) -> Self {
    unsafe { core::mem::transmute(array) }
  }

  /// widens and sign extends to `i32x8`
  #[inline]
  #[must_use]
  pub fn from_i16x8(v: i16x8) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        i32x8 { avx2:convert_to_i32_m256i_from_i16_m128i(v.sse) }
      } else if #[cfg(target_feature="sse2")] {
        i32x8 {
          a: i32x4 { sse: shr_imm_i32_m128i::<16>( unpack_low_i16_m128i(v.sse, v.sse)) },
          b: i32x4 { sse: shr_imm_i32_m128i::<16>( unpack_high_i16_m128i(v.sse, v.sse)) },
        }
      } else {
        i32x8::new([
          i32::from(v.as_array_ref()[0]),
          i32::from(v.as_array_ref()[1]),
          i32::from(v.as_array_ref()[2]),
          i32::from(v.as_array_ref()[3]),
          i32::from(v.as_array_ref()[4]),
          i32::from(v.as_array_ref()[5]),
          i32::from(v.as_array_ref()[6]),
          i32::from(v.as_array_ref()[7]),
        ])
      }
    }
  }

  /// widens and zero extends to `i32x8`
  #[inline]
  #[must_use]
  pub fn from_u16x8(v: u16x8) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        i32x8 { avx2:convert_to_i32_m256i_from_u16_m128i(v.sse) }
      } else if #[cfg(target_feature="sse2")] {
        i32x8 {
          a: i32x4 { sse: shr_imm_u32_m128i::<16>( unpack_low_i16_m128i(v.sse, v.sse)) },
          b: i32x4 { sse: shr_imm_u32_m128i::<16>( unpack_high_i16_m128i(v.sse, v.sse)) },
        }
      } else {
        i32x8::new([
          i32::from(v.as_array_ref()[0]),
          i32::from(v.as_array_ref()[1]),
          i32::from(v.as_array_ref()[2]),
          i32::from(v.as_array_ref()[3]),
          i32::from(v.as_array_ref()[4]),
          i32::from(v.as_array_ref()[5]),
          i32::from(v.as_array_ref()[6]),
          i32::from(v.as_array_ref()[7]),
        ])
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
          b : self.b.blend(t.b, f.b)
        }
      }
    }
  }

  /// horizontal add of all the elements of the vector
  #[inline]
  #[must_use]
  pub fn reduce_add(self) -> i32 {
    let arr: [i32x4; 2] = cast(self);
    (arr[0] + arr[1]).reduce_add()
  }

  /// horizontal max of all the elements of the vector
  #[inline]
  #[must_use]
  pub fn reduce_max(self) -> i32 {
    let arr: [i32x4; 2] = cast(self);
    arr[0].max(arr[1]).reduce_max()
  }

  /// horizontal min of all the elements of the vector
  #[inline]
  #[must_use]
  pub fn reduce_min(self) -> i32 {
    let arr: [i32x4; 2] = cast(self);
    arr[0].min(arr[1]).reduce_min()
  }

  #[inline]
  #[must_use]
  pub fn abs(self) -> Self {
    pick! {
      if #[cfg(target_feature="avx2")] {
        Self { avx2: abs_i32_m256i(self.avx2) }
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
  pub fn unsigned_abs(self) -> u32x8 {
    pick! {
      if #[cfg(target_feature="avx2")] {
        u32x8 { avx2: abs_i32_m256i(self.avx2) }
      } else {
        u32x8 {
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
        Self { avx2: max_i32_m256i(self.avx2, rhs.avx2) }
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
        Self { avx2: min_i32_m256i(self.avx2, rhs.avx2) }
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
  pub fn round_float(self) -> f32x8 {
    pick! {
      if #[cfg(target_feature="avx2")] {
        cast(convert_to_m256_from_i32_m256i(self.avx2))
      } else {
        cast([
          self.a.round_float(),
          self.b.round_float(),
        ])
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn move_mask(self) -> i32 {
    pick! {
      if #[cfg(target_feature="avx2")] {
        // use f32 move_mask since it is the same size as i32
        move_mask_m256(cast(self.avx2))
      } else {
        self.a.move_mask() | (self.b.move_mask() << 4)
      }
    }
  }

  #[inline]
  #[must_use]
  pub fn any(self) -> bool {
    pick! {
      if #[cfg(target_feature="avx2")] {
        move_mask_m256(cast(self.avx2)) != 0
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
        move_mask_m256(cast(self.avx2)) == 0b11111111
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

  /// Transpose matrix of 8x8 `i32` matrix. Currently only accelerated on AVX2.
  #[must_use]
  #[inline]
  pub fn transpose(data: [i32x8; 8]) -> [i32x8; 8] {
    pick! {
      if #[cfg(target_feature="avx2")] {
        let a0 = unpack_low_i32_m256i(data[0].avx2, data[1].avx2);
        let a1 = unpack_high_i32_m256i(data[0].avx2, data[1].avx2);
        let a2 = unpack_low_i32_m256i(data[2].avx2, data[3].avx2);
        let a3 = unpack_high_i32_m256i(data[2].avx2, data[3].avx2);
        let a4 = unpack_low_i32_m256i(data[4].avx2, data[5].avx2);
        let a5 = unpack_high_i32_m256i(data[4].avx2, data[5].avx2);
        let a6 = unpack_low_i32_m256i(data[6].avx2, data[7].avx2);
        let a7 = unpack_high_i32_m256i(data[6].avx2, data[7].avx2);

        pub const fn mm_shuffle(z: i32, y: i32, x: i32, w: i32) -> i32 {
          (z << 6) | (y << 4) | (x << 2) | w
        }

        const SHUFF_LO : i32 = mm_shuffle(1,0,1,0);
        const SHUFF_HI : i32 = mm_shuffle(3,2,3,2);

        // possible todo: intel performance manual suggests alternative with blend to avoid port 5 pressure
        // (since blend runs on a different port than shuffle)
        let b0 = cast::<m256,m256i>(shuffle_m256::<SHUFF_LO>(cast(a0),cast(a2)));
        let b1 = cast::<m256,m256i>(shuffle_m256::<SHUFF_HI>(cast(a0),cast(a2)));
        let b2 = cast::<m256,m256i>(shuffle_m256::<SHUFF_LO>(cast(a1),cast(a3)));
        let b3 = cast::<m256,m256i>(shuffle_m256::<SHUFF_HI>(cast(a1),cast(a3)));
        let b4 = cast::<m256,m256i>(shuffle_m256::<SHUFF_LO>(cast(a4),cast(a6)));
        let b5 = cast::<m256,m256i>(shuffle_m256::<SHUFF_HI>(cast(a4),cast(a6)));
        let b6 = cast::<m256,m256i>(shuffle_m256::<SHUFF_LO>(cast(a5),cast(a7)));
        let b7 = cast::<m256,m256i>(shuffle_m256::<SHUFF_HI>(cast(a5),cast(a7)));

        [
          i32x8 { avx2: permute2z_m256i::<0x20>(b0, b4) },
          i32x8 { avx2: permute2z_m256i::<0x20>(b1, b5) },
          i32x8 { avx2: permute2z_m256i::<0x20>(b2, b6) },
          i32x8 { avx2: permute2z_m256i::<0x20>(b3, b7) },
          i32x8 { avx2: permute2z_m256i::<0x31>(b0, b4) },
          i32x8 { avx2: permute2z_m256i::<0x31>(b1, b5) },
          i32x8 { avx2: permute2z_m256i::<0x31>(b2, b6) },
          i32x8 { avx2: permute2z_m256i::<0x31>(b3, b7) }
        ]
      } else {
        // possible todo: not sure that 128bit SIMD gives us a a lot of speedup here

        #[inline(always)]
        fn transpose_column(data: &[i32x8; 8], index: usize) -> i32x8 {
          i32x8::new([
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
  pub fn to_array(self) -> [i32; 8] {
    cast(self)
  }

  #[inline]
  pub fn as_array_ref(&self) -> &[i32; 8] {
    cast_ref(self)
  }

  #[inline]
  pub fn as_array_mut(&mut self) -> &mut [i32; 8] {
    cast_mut(self)
  }
}

impl Not for i32x8 {
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
