// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[macro_use]
pub mod forward_shared;

pub use self::forward::forward_transform;
pub use self::inverse::inverse_transform_add;

use crate::context::MI_SIZE_LOG2;
use crate::partition::{BlockSize, BlockSize::*};
use crate::util::*;

use TxSize::*;

pub mod forward;
pub mod inverse;

pub static RAV1E_TX_TYPES: &[TxType] = &[
  TxType::DCT_DCT,
  TxType::ADST_DCT,
  TxType::DCT_ADST,
  TxType::ADST_ADST,
  // TODO: Add a speed setting for FLIPADST
  // TxType::FLIPADST_DCT,
  // TxType::DCT_FLIPADST,
  // TxType::FLIPADST_FLIPADST,
  // TxType::ADST_FLIPADST,
  // TxType::FLIPADST_ADST,
  TxType::IDTX,
  TxType::V_DCT,
  TxType::H_DCT,
  //TxType::V_FLIPADST,
  //TxType::H_FLIPADST,
];

pub mod consts {
  pub static SQRT2_BITS: usize = 12;
  pub static SQRT2: i32 = 5793; // 2^12 * sqrt(2)
  pub static INV_SQRT2: i32 = 2896; // 2^12 / sqrt(2)
}

pub const TX_TYPES: usize = 16;
pub const TX_TYPES_PLUS_LL: usize = 17;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum TxType {
  DCT_DCT = 0,   // DCT  in both horizontal and vertical
  ADST_DCT = 1,  // ADST in vertical, DCT in horizontal
  DCT_ADST = 2,  // DCT  in vertical, ADST in horizontal
  ADST_ADST = 3, // ADST in both directions
  FLIPADST_DCT = 4,
  DCT_FLIPADST = 5,
  FLIPADST_FLIPADST = 6,
  ADST_FLIPADST = 7,
  FLIPADST_ADST = 8,
  IDTX = 9,
  V_DCT = 10,
  H_DCT = 11,
  V_ADST = 12,
  H_ADST = 13,
  V_FLIPADST = 14,
  H_FLIPADST = 15,
  WHT_WHT = 16,
}

impl TxType {
  /// Compute transform type for inter chroma.
  ///
  /// <https://aomediacodec.github.io/av1-spec/#compute-transform-type-function>
  #[inline]
  pub fn uv_inter(self, uv_tx_size: TxSize) -> Self {
    use TxType::*;
    if uv_tx_size.sqr_up() == TX_32X32 {
      match self {
        IDTX => IDTX,
        _ => DCT_DCT,
      }
    } else if uv_tx_size.sqr() == TX_16X16 {
      match self {
        V_ADST | H_ADST | V_FLIPADST | H_FLIPADST => DCT_DCT,
        _ => self,
      }
    } else {
      self
    }
  }
}

/// Transform Size
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum TxSize {
  TX_4X4,
  TX_8X8,
  TX_16X16,
  TX_32X32,
  TX_64X64,

  TX_4X8,
  TX_8X4,
  TX_8X16,
  TX_16X8,
  TX_16X32,
  TX_32X16,
  TX_32X64,
  TX_64X32,

  TX_4X16,
  TX_16X4,
  TX_8X32,
  TX_32X8,
  TX_16X64,
  TX_64X16,
}

impl TxSize {
  /// Number of square transform sizes
  pub const TX_SIZES: usize = 5;

  /// Number of transform sizes (including non-square sizes)
  pub const TX_SIZES_ALL: usize = 14 + 5;

  #[inline]
  pub const fn width(self) -> usize {
    1 << self.width_log2()
  }

  #[inline]
  pub const fn width_log2(self) -> usize {
    match self {
      TX_4X4 | TX_4X8 | TX_4X16 => 2,
      TX_8X8 | TX_8X4 | TX_8X16 | TX_8X32 => 3,
      TX_16X16 | TX_16X8 | TX_16X32 | TX_16X4 | TX_16X64 => 4,
      TX_32X32 | TX_32X16 | TX_32X64 | TX_32X8 => 5,
      TX_64X64 | TX_64X32 | TX_64X16 => 6,
    }
  }

  #[inline]
  pub const fn width_index(self) -> usize {
    self.width_log2() - TX_4X4.width_log2()
  }

  #[inline]
  pub const fn height(self) -> usize {
    1 << self.height_log2()
  }

  #[inline]
  pub const fn height_log2(self) -> usize {
    match self {
      TX_4X4 | TX_8X4 | TX_16X4 => 2,
      TX_8X8 | TX_4X8 | TX_16X8 | TX_32X8 => 3,
      TX_16X16 | TX_8X16 | TX_32X16 | TX_4X16 | TX_64X16 => 4,
      TX_32X32 | TX_16X32 | TX_64X32 | TX_8X32 => 5,
      TX_64X64 | TX_32X64 | TX_16X64 => 6,
    }
  }

  #[inline]
  pub const fn height_index(self) -> usize {
    self.height_log2() - TX_4X4.height_log2()
  }

  #[inline]
  pub const fn width_mi(self) -> usize {
    self.width() >> MI_SIZE_LOG2
  }

  #[inline]
  pub const fn area(self) -> usize {
    1 << self.area_log2()
  }

  #[inline]
  pub const fn area_log2(self) -> usize {
    self.width_log2() + self.height_log2()
  }

  #[inline]
  pub const fn height_mi(self) -> usize {
    self.height() >> MI_SIZE_LOG2
  }

  #[inline]
  pub const fn block_size(self) -> BlockSize {
    match self {
      TX_4X4 => BLOCK_4X4,
      TX_8X8 => BLOCK_8X8,
      TX_16X16 => BLOCK_16X16,
      TX_32X32 => BLOCK_32X32,
      TX_64X64 => BLOCK_64X64,
      TX_4X8 => BLOCK_4X8,
      TX_8X4 => BLOCK_8X4,
      TX_8X16 => BLOCK_8X16,
      TX_16X8 => BLOCK_16X8,
      TX_16X32 => BLOCK_16X32,
      TX_32X16 => BLOCK_32X16,
      TX_32X64 => BLOCK_32X64,
      TX_64X32 => BLOCK_64X32,
      TX_4X16 => BLOCK_4X16,
      TX_16X4 => BLOCK_16X4,
      TX_8X32 => BLOCK_8X32,
      TX_32X8 => BLOCK_32X8,
      TX_16X64 => BLOCK_16X64,
      TX_64X16 => BLOCK_64X16,
    }
  }

  #[inline]
  pub const fn sqr(self) -> TxSize {
    match self {
      TX_4X4 | TX_4X8 | TX_8X4 | TX_4X16 | TX_16X4 => TX_4X4,
      TX_8X8 | TX_8X16 | TX_16X8 | TX_8X32 | TX_32X8 => TX_8X8,
      TX_16X16 | TX_16X32 | TX_32X16 | TX_16X64 | TX_64X16 => TX_16X16,
      TX_32X32 | TX_32X64 | TX_64X32 => TX_32X32,
      TX_64X64 => TX_64X64,
    }
  }

  #[inline]
  pub const fn sqr_up(self) -> TxSize {
    match self {
      TX_4X4 => TX_4X4,
      TX_8X8 | TX_4X8 | TX_8X4 => TX_8X8,
      TX_16X16 | TX_8X16 | TX_16X8 | TX_4X16 | TX_16X4 => TX_16X16,
      TX_32X32 | TX_16X32 | TX_32X16 | TX_8X32 | TX_32X8 => TX_32X32,
      TX_64X64 | TX_32X64 | TX_64X32 | TX_16X64 | TX_64X16 => TX_64X64,
    }
  }

  #[inline]
  pub fn by_dims(w: usize, h: usize) -> TxSize {
    match (w, h) {
      (4, 4) => TX_4X4,
      (8, 8) => TX_8X8,
      (16, 16) => TX_16X16,
      (32, 32) => TX_32X32,
      (64, 64) => TX_64X64,
      (4, 8) => TX_4X8,
      (8, 4) => TX_8X4,
      (8, 16) => TX_8X16,
      (16, 8) => TX_16X8,
      (16, 32) => TX_16X32,
      (32, 16) => TX_32X16,
      (32, 64) => TX_32X64,
      (64, 32) => TX_64X32,
      (4, 16) => TX_4X16,
      (16, 4) => TX_16X4,
      (8, 32) => TX_8X32,
      (32, 8) => TX_32X8,
      (16, 64) => TX_16X64,
      (64, 16) => TX_64X16,
      _ => unreachable!(),
    }
  }

  #[inline]
  pub const fn is_rect(self) -> bool {
    self.width_log2() != self.height_log2()
  }

  /// Returns log2(width / height), e.g. `TX_16x4` -> log2(16 / 4) = 2
  #[inline]
  pub const fn rect_ratio_log2(self) -> i8 {
    self.width_log2() as i8 - self.height_log2() as i8
  }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum TxSet {
  // DCT only
  TX_SET_DCTONLY,
  // DCT + Identity only
  TX_SET_INTER_3, // TX_SET_DCT_IDTX
  // Discrete Trig transforms w/o flip (4) + Identity (1)
  TX_SET_INTRA_2, // TX_SET_DTT4_IDTX
  // Discrete Trig transforms w/o flip (4) + Identity (1) + 1D Hor/vert DCT (2)
  TX_SET_INTRA_1, // TX_SET_DTT4_IDTX_1DDCT
  // Discrete Trig transforms w/ flip (9) + Identity (1) + 1D Hor/Ver DCT (2)
  TX_SET_INTER_2, // TX_SET_DTT9_IDTX_1DDCT
  // Discrete Trig transforms w/ flip (9) + Identity (1) + 1D Hor/Ver (6)
  TX_SET_INTER_1, // TX_SET_ALL16
}

// performs half a butterfly
#[inline]
const fn half_btf(w0: i32, in0: i32, w1: i32, in1: i32, bit: usize) -> i32 {
  // Ensure defined behaviour for when w0*in0 + w1*in1 is negative and
  //   overflows, but w0*in0 + w1*in1 + rounding isn't.
  let result = (w0 * in0).wrapping_add(w1 * in1);
  // Implement a version of round_shift with wrapping
  if bit == 0 {
    result
  } else {
    result.wrapping_add(1 << (bit - 1)) >> bit
  }
}

// clamps value to a signed integer type of bit bits
#[inline]
fn clamp_value(value: i32, bit: usize) -> i32 {
  let max_value: i32 = ((1i64 << (bit - 1)) - 1) as i32;
  let min_value: i32 = (-(1i64 << (bit - 1))) as i32;
  clamp(value, min_value, max_value)
}

pub fn av1_round_shift_array(arr: &mut [i32], size: usize, bit: i8) {
  if bit == 0 {
    return;
  }
  if bit > 0 {
    let bit = bit as usize;
    arr.iter_mut().take(size).for_each(|i| {
      *i = round_shift(*i, bit);
    })
  } else {
    arr.iter_mut().take(size).for_each(|i| {
      *i <<= -bit;
    })
  }
}

#[derive(Debug, Clone, Copy)]
enum TxType1D {
  DCT,
  ADST,
  FLIPADST,
  IDTX,
  WHT,
}

const fn get_1d_tx_types(tx_type: TxType) -> (TxType1D, TxType1D) {
  match tx_type {
    TxType::DCT_DCT => (TxType1D::DCT, TxType1D::DCT),
    TxType::ADST_DCT => (TxType1D::ADST, TxType1D::DCT),
    TxType::DCT_ADST => (TxType1D::DCT, TxType1D::ADST),
    TxType::ADST_ADST => (TxType1D::ADST, TxType1D::ADST),
    TxType::FLIPADST_DCT => (TxType1D::FLIPADST, TxType1D::DCT),
    TxType::DCT_FLIPADST => (TxType1D::DCT, TxType1D::FLIPADST),
    TxType::FLIPADST_FLIPADST => (TxType1D::FLIPADST, TxType1D::FLIPADST),
    TxType::ADST_FLIPADST => (TxType1D::ADST, TxType1D::FLIPADST),
    TxType::FLIPADST_ADST => (TxType1D::FLIPADST, TxType1D::ADST),
    TxType::IDTX => (TxType1D::IDTX, TxType1D::IDTX),
    TxType::V_DCT => (TxType1D::DCT, TxType1D::IDTX),
    TxType::H_DCT => (TxType1D::IDTX, TxType1D::DCT),
    TxType::V_ADST => (TxType1D::ADST, TxType1D::IDTX),
    TxType::H_ADST => (TxType1D::IDTX, TxType1D::ADST),
    TxType::V_FLIPADST => (TxType1D::FLIPADST, TxType1D::IDTX),
    TxType::H_FLIPADST => (TxType1D::IDTX, TxType1D::FLIPADST),
    TxType::WHT_WHT => (TxType1D::WHT, TxType1D::WHT),
  }
}

const VTX_TAB: [TxType1D; TX_TYPES_PLUS_LL] = [
  TxType1D::DCT,
  TxType1D::ADST,
  TxType1D::DCT,
  TxType1D::ADST,
  TxType1D::FLIPADST,
  TxType1D::DCT,
  TxType1D::FLIPADST,
  TxType1D::ADST,
  TxType1D::FLIPADST,
  TxType1D::IDTX,
  TxType1D::DCT,
  TxType1D::IDTX,
  TxType1D::ADST,
  TxType1D::IDTX,
  TxType1D::FLIPADST,
  TxType1D::IDTX,
  TxType1D::WHT,
];

const HTX_TAB: [TxType1D; TX_TYPES_PLUS_LL] = [
  TxType1D::DCT,
  TxType1D::DCT,
  TxType1D::ADST,
  TxType1D::ADST,
  TxType1D::DCT,
  TxType1D::FLIPADST,
  TxType1D::FLIPADST,
  TxType1D::FLIPADST,
  TxType1D::ADST,
  TxType1D::IDTX,
  TxType1D::IDTX,
  TxType1D::DCT,
  TxType1D::IDTX,
  TxType1D::ADST,
  TxType1D::IDTX,
  TxType1D::FLIPADST,
  TxType1D::WHT,
];

#[inline]
pub const fn valid_av1_transform(tx_size: TxSize, tx_type: TxType) -> bool {
  let size_sq = tx_size.sqr_up();
  use TxSize::*;
  use TxType::*;
  match (size_sq, tx_type) {
    (TX_64X64, DCT_DCT) => true,
    (TX_64X64, _) => false,
    (TX_32X32, DCT_DCT) => true,
    (TX_32X32, IDTX) => true,
    (TX_32X32, _) => false,
    (_, _) => true,
  }
}

#[cfg(any(test, feature = "bench"))]
pub fn get_valid_txfm_types(tx_size: TxSize) -> &'static [TxType] {
  let size_sq = tx_size.sqr_up();
  use TxType::*;
  if size_sq == TxSize::TX_64X64 {
    &[DCT_DCT]
  } else if size_sq == TxSize::TX_32X32 {
    &[DCT_DCT, IDTX]
  } else if size_sq == TxSize::TX_4X4 {
    &[
      DCT_DCT,
      ADST_DCT,
      DCT_ADST,
      ADST_ADST,
      FLIPADST_DCT,
      DCT_FLIPADST,
      FLIPADST_FLIPADST,
      ADST_FLIPADST,
      FLIPADST_ADST,
      IDTX,
      V_DCT,
      H_DCT,
      V_ADST,
      H_ADST,
      V_FLIPADST,
      H_FLIPADST,
      WHT_WHT,
    ]
  } else {
    &[
      DCT_DCT,
      ADST_DCT,
      DCT_ADST,
      ADST_ADST,
      FLIPADST_DCT,
      DCT_FLIPADST,
      FLIPADST_FLIPADST,
      ADST_FLIPADST,
      FLIPADST_ADST,
      IDTX,
      V_DCT,
      H_DCT,
      V_ADST,
      H_ADST,
      V_FLIPADST,
      H_FLIPADST,
    ]
  }
}

#[cfg(test)]
mod test {
  use super::TxType::*;
  use super::*;
  use crate::context::av1_get_coded_tx_size;
  use crate::cpu_features::CpuFeatureLevel;
  use crate::frame::*;
  use rand::random;
  use std::mem::MaybeUninit;

  fn test_roundtrip<T: Pixel>(
    tx_size: TxSize, tx_type: TxType, tolerance: i16,
  ) {
    let cpu = CpuFeatureLevel::default();

    let coeff_area: usize = av1_get_coded_tx_size(tx_size).area();
    let mut src_storage = [T::cast_from(0); 64 * 64];
    let src = &mut src_storage[..tx_size.area()];
    let mut dst = Plane::from_slice(
      &[T::zero(); 64 * 64][..tx_size.area()],
      tx_size.width(),
    );
    let mut res_storage = [0i16; 64 * 64];
    let res = &mut res_storage[..tx_size.area()];
    let mut freq_storage = [MaybeUninit::uninit(); 64 * 64];
    let freq = &mut freq_storage[..tx_size.area()];
    for ((r, s), d) in
      res.iter_mut().zip(src.iter_mut()).zip(dst.data.iter_mut())
    {
      *s = T::cast_from(random::<u8>());
      *d = T::cast_from(random::<u8>());
      *r = i16::cast_from(*s) - i16::cast_from(*d);
    }
    forward_transform(res, freq, tx_size.width(), tx_size, tx_type, 8, cpu);
    // SAFETY: forward_transform initialized freq
    let freq = unsafe { slice_assume_init_mut(freq) };
    inverse_transform_add(
      freq,
      &mut dst.as_region_mut(),
      coeff_area.try_into().unwrap(),
      tx_size,
      tx_type,
      8,
      cpu,
    );

    for (s, d) in src.iter().zip(dst.data.iter()) {
      assert!(i16::abs(i16::cast_from(*s) - i16::cast_from(*d)) <= tolerance);
    }
  }

  #[test]
  fn log_tx_ratios() {
    let combinations = [
      (TxSize::TX_4X4, 0),
      (TxSize::TX_8X8, 0),
      (TxSize::TX_16X16, 0),
      (TxSize::TX_32X32, 0),
      (TxSize::TX_64X64, 0),
      (TxSize::TX_4X8, -1),
      (TxSize::TX_8X4, 1),
      (TxSize::TX_8X16, -1),
      (TxSize::TX_16X8, 1),
      (TxSize::TX_16X32, -1),
      (TxSize::TX_32X16, 1),
      (TxSize::TX_32X64, -1),
      (TxSize::TX_64X32, 1),
      (TxSize::TX_4X16, -2),
      (TxSize::TX_16X4, 2),
      (TxSize::TX_8X32, -2),
      (TxSize::TX_32X8, 2),
      (TxSize::TX_16X64, -2),
      (TxSize::TX_64X16, 2),
    ];

    for &(tx_size, expected) in combinations.iter() {
      println!(
        "Testing combination {:?}, {:?}",
        tx_size.width(),
        tx_size.height()
      );
      assert!(tx_size.rect_ratio_log2() == expected);
    }
  }

  fn roundtrips<T: Pixel>() {
    let combinations = [
      (TX_4X4, WHT_WHT, 0),
      (TX_4X4, DCT_DCT, 0),
      (TX_4X4, ADST_DCT, 0),
      (TX_4X4, DCT_ADST, 0),
      (TX_4X4, ADST_ADST, 0),
      (TX_4X4, FLIPADST_DCT, 0),
      (TX_4X4, DCT_FLIPADST, 0),
      (TX_4X4, IDTX, 0),
      (TX_4X4, V_DCT, 0),
      (TX_4X4, H_DCT, 0),
      (TX_4X4, V_ADST, 0),
      (TX_4X4, H_ADST, 0),
      (TX_8X8, DCT_DCT, 1),
      (TX_8X8, ADST_DCT, 1),
      (TX_8X8, DCT_ADST, 1),
      (TX_8X8, ADST_ADST, 1),
      (TX_8X8, FLIPADST_DCT, 1),
      (TX_8X8, DCT_FLIPADST, 1),
      (TX_8X8, IDTX, 0),
      (TX_8X8, V_DCT, 0),
      (TX_8X8, H_DCT, 0),
      (TX_8X8, V_ADST, 0),
      (TX_8X8, H_ADST, 1),
      (TX_16X16, DCT_DCT, 1),
      (TX_16X16, ADST_DCT, 1),
      (TX_16X16, DCT_ADST, 1),
      (TX_16X16, ADST_ADST, 1),
      (TX_16X16, FLIPADST_DCT, 1),
      (TX_16X16, DCT_FLIPADST, 1),
      (TX_16X16, IDTX, 0),
      (TX_16X16, V_DCT, 1),
      (TX_16X16, H_DCT, 1),
      // 32x transforms only use DCT_DCT and IDTX
      (TX_32X32, DCT_DCT, 2),
      (TX_32X32, IDTX, 0),
      // 64x transforms only use DCT_DCT and IDTX
      //(TX_64X64, DCT_DCT, 0),
      (TX_4X8, DCT_DCT, 1),
      (TX_8X4, DCT_DCT, 1),
      (TX_4X16, DCT_DCT, 1),
      (TX_16X4, DCT_DCT, 1),
      (TX_8X16, DCT_DCT, 1),
      (TX_16X8, DCT_DCT, 1),
      (TX_8X32, DCT_DCT, 2),
      (TX_32X8, DCT_DCT, 2),
      (TX_16X32, DCT_DCT, 2),
      (TX_32X16, DCT_DCT, 2),
    ];
    for &(tx_size, tx_type, tolerance) in combinations.iter() {
      println!("Testing combination {:?}, {:?}", tx_size, tx_type);
      test_roundtrip::<T>(tx_size, tx_type, tolerance);
    }
  }

  #[test]
  fn roundtrips_u8() {
    roundtrips::<u8>();
  }

  #[test]
  fn roundtrips_u16() {
    roundtrips::<u16>();
  }
}
