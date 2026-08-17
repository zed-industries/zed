// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#![allow(non_upper_case_globals)]

mod tables;

cfg_if::cfg_if! {
  if #[cfg(nasm_x86_64)] {
    pub use crate::asm::x86::quantize::*;
  } else {
    pub use self::rust::*;
  }
}

pub use tables::*;

use crate::scan_order::av1_scan_orders;
use crate::transform::{TxSize, TxType};
use crate::util::*;
use std::mem;
use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};

pub fn get_log_tx_scale(tx_size: TxSize) -> usize {
  let num_pixels = tx_size.area();

  Into::<usize>::into(num_pixels > 256)
    + Into::<usize>::into(num_pixels > 1024)
}

pub fn dc_q(qindex: u8, delta_q: i8, bit_depth: usize) -> NonZeroU16 {
  let dc_q: [&[NonZeroU16; 256]; 3] =
    [&dc_qlookup_Q3, &dc_qlookup_10_Q3, &dc_qlookup_12_Q3];
  let bd = ((bit_depth ^ 8) >> 1).min(2);
  dc_q[bd][((qindex as isize + delta_q as isize).max(0) as usize).min(255)]
}

pub fn ac_q(qindex: u8, delta_q: i8, bit_depth: usize) -> NonZeroU16 {
  let ac_q: [&[NonZeroU16; 256]; 3] =
    [&ac_qlookup_Q3, &ac_qlookup_10_Q3, &ac_qlookup_12_Q3];
  let bd = ((bit_depth ^ 8) >> 1).min(2);
  ac_q[bd][((qindex as isize + delta_q as isize).max(0) as usize).min(255)]
}

// TODO: Handle lossless properly.
fn select_qi(quantizer: i64, qlookup: &[NonZeroU16; QINDEX_RANGE]) -> u8 {
  if quantizer < qlookup[MINQ].get() as i64 {
    MINQ as u8
  } else if quantizer >= qlookup[MAXQ].get() as i64 {
    MAXQ as u8
  } else {
    match qlookup
      .binary_search(&NonZeroU16::new(quantizer as u16).expect("Not zero"))
    {
      Ok(qi) => qi as u8,
      Err(qi) => {
        debug_assert!(qi > MINQ);
        debug_assert!(qi <= MAXQ);
        // Pick the closest quantizer in the log domain.
        let qthresh =
          (qlookup[qi - 1].get() as i32) * (qlookup[qi].get() as i32);
        let q2_i32 = (quantizer as i32) * (quantizer as i32);
        if q2_i32 < qthresh {
          (qi - 1) as u8
        } else {
          qi as u8
        }
      }
    }
  }
}

pub fn select_dc_qi(quantizer: i64, bit_depth: usize) -> u8 {
  let qlookup = match bit_depth {
    8 => &dc_qlookup_Q3,
    10 => &dc_qlookup_10_Q3,
    12 => &dc_qlookup_12_Q3,
    _ => unimplemented!(),
  };
  select_qi(quantizer, qlookup)
}

pub fn select_ac_qi(quantizer: i64, bit_depth: usize) -> u8 {
  let qlookup = match bit_depth {
    8 => &ac_qlookup_Q3,
    10 => &ac_qlookup_10_Q3,
    12 => &ac_qlookup_12_Q3,
    _ => unimplemented!(),
  };
  select_qi(quantizer, qlookup)
}

#[derive(Debug, Clone, Copy)]
pub struct QuantizationContext {
  log_tx_scale: usize,
  dc_quant: NonZeroU16,
  dc_offset: u32,
  dc_mul_add: (u32, u32, u32),

  ac_quant: NonZeroU16,
  ac_offset_eob: u32,
  ac_offset0: u32,
  ac_offset1: u32,
  ac_mul_add: (u32, u32, u32),
}

impl Default for QuantizationContext {
  fn default() -> Self {
    QuantizationContext {
      dc_quant: NonZeroU16::new(1).expect("Not zero"),
      ac_quant: NonZeroU16::new(1).expect("Not zero"),
      log_tx_scale: Default::default(),
      dc_offset: Default::default(),
      dc_mul_add: Default::default(),
      ac_offset_eob: Default::default(),
      ac_offset0: Default::default(),
      ac_offset1: Default::default(),
      ac_mul_add: Default::default(),
    }
  }
}

fn divu_gen(d: NonZeroU32) -> (u32, u32, u32) {
  let nbits = (mem::size_of_val(&d) as u64) * 8;
  let m = nbits - d.leading_zeros() as u64 - 1;
  if d.is_power_of_two() {
    (0xFFFF_FFFF, 0xFFFF_FFFF, m as u32)
  } else {
    let d = NonZeroU64::from(d);
    let t = (1u64 << (m + nbits)) / d;

    let d = d.get();
    let r = (t * d + d) & ((1 << nbits) - 1);
    if r <= 1u64 << m {
      (t as u32 + 1, 0u32, m as u32)
    } else {
      (t as u32, t as u32, m as u32)
    }
  }
}

#[inline]
const fn divu_pair(x: u32, d: (u32, u32, u32)) -> u32 {
  let x = x as u64;
  let (a, b, shift) = d;
  let shift = shift as u64;
  let a = a as u64;
  let b = b as u64;

  (((a * x + b) >> 32) >> shift) as u32
}

#[inline]
const fn copysign(value: u32, signed: i32) -> i32 {
  if signed < 0 {
    -(value as i32)
  } else {
    value as i32
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::transform::TxSize::*;

  #[test]
  fn test_divu_pair() {
    for d in 1..1024 {
      for x in 0..1000 {
        let ab = divu_gen(NonZeroU32::new(d).unwrap());
        assert_eq!(x / d, divu_pair(x, ab));
      }
    }
  }
  #[test]
  fn gen_divu_table() {
    let b: Vec<(u32, u32, u32)> =
      dc_qlookup_Q3.iter().map(|&v| divu_gen(v.into())).collect();

    println!("{:?}", b);
  }
  #[test]
  fn test_tx_log_scale() {
    let tx_sizes = [
      (TX_4X4, 0),
      (TX_8X8, 0),
      (TX_16X16, 0),
      (TX_32X32, 1),
      (TX_64X64, 2),
      (TX_4X8, 0),
      (TX_8X4, 0),
      (TX_8X16, 0),
      (TX_16X8, 0),
      (TX_16X32, 1),
      (TX_32X16, 1),
      (TX_32X64, 2),
      (TX_64X32, 2),
      (TX_4X16, 0),
      (TX_16X4, 0),
      (TX_8X32, 0),
      (TX_32X8, 0),
      (TX_16X64, 1),
      (TX_64X16, 1),
    ];
    for &tx_size in tx_sizes.iter() {
      assert!(tx_size.1 == get_log_tx_scale(tx_size.0));
    }
  }
}

impl QuantizationContext {
  pub fn update(
    &mut self, qindex: u8, tx_size: TxSize, is_intra: bool, bit_depth: usize,
    dc_delta_q: i8, ac_delta_q: i8,
  ) {
    self.log_tx_scale = get_log_tx_scale(tx_size);

    self.dc_quant = dc_q(qindex, dc_delta_q, bit_depth);
    self.dc_mul_add = divu_gen(self.dc_quant.into());

    self.ac_quant = ac_q(qindex, ac_delta_q, bit_depth);
    self.ac_mul_add = divu_gen(self.ac_quant.into());

    // All of these biases were derived by measuring the cost of coding
    // a zero vs coding a one on any given coefficient position, or, in
    // the case of the EOB bias, the cost of coding the block with
    // the chosen EOB (rounding to one) vs rounding to zero and continuing
    // to choose a new EOB. This was done over several clips, with the
    // average of the bit costs taken over all blocks in the set, and a new
    // bias derived via the method outlined in Jean-Marc Valin's
    // Journal of Dubious Theoretical Results[1], aka:
    //
    // lambda = ln(2) / 6.0
    // threshold = 0.5 + (lambda * avg_rate_diff) / 2.0
    // bias = 1 - threshold
    //
    // lambda is a constant since our offsets are already adjusted for the
    // quantizer.
    //
    // Biases were then updated, and cost collection was re-run, until
    // the calculated biases started to converge after 2-4 iterations.
    //
    // In theory, the rounding biases for inter should be somewhat smaller
    // than the biases for intra, but this turns out to only be the case
    // for EOB optimization, or at least, is covered by EOB optimization.
    // The RD-optimal rounding biases for the actual coefficients seem
    // to be quite close (+/- 1/256), for both inter and intra,
    // post-deadzoning.
    //
    // [1] https://jmvalin.ca/notes/theoretical_results.pdf
    self.dc_offset =
      self.dc_quant.get() as u32 * (if is_intra { 109 } else { 108 }) / 256;
    self.ac_offset0 =
      self.ac_quant.get() as u32 * (if is_intra { 98 } else { 97 }) / 256;
    self.ac_offset1 =
      self.ac_quant.get() as u32 * (if is_intra { 109 } else { 108 }) / 256;
    self.ac_offset_eob =
      self.ac_quant.get() as u32 * (if is_intra { 88 } else { 44 }) / 256;
  }

  #[inline]
  pub fn quantize<T: Coefficient>(
    &self, coeffs: &[T], qcoeffs: &mut [T], tx_size: TxSize, tx_type: TxType,
  ) -> u16 {
    let scan = av1_scan_orders[tx_size as usize][tx_type as usize].scan;
    let iscan = av1_scan_orders[tx_size as usize][tx_type as usize].iscan;

    qcoeffs[0] = {
      let coeff: i32 = i32::cast_from(coeffs[0]) << self.log_tx_scale;
      let abs_coeff = coeff.unsigned_abs();
      T::cast_from(copysign(
        divu_pair(abs_coeff + self.dc_offset, self.dc_mul_add),
        coeff,
      ))
    };

    // Find the last non-zero coefficient using our smaller biases and
    // zero everything else.
    // This threshold is such that `abs(coeff) < deadzone` implies:
    // (abs(coeff << log_tx_scale) + ac_offset_eob) / ac_quant == 0
    let deadzone = T::cast_from(
      (self.ac_quant.get() as usize - self.ac_offset_eob as usize)
        .align_power_of_two_and_shift(self.log_tx_scale),
    );
    let eob = {
      let eob_minus_one = iscan
        .iter()
        .zip(coeffs)
        .map(|(&i, &c)| if c.abs() >= deadzone { i } else { 0 })
        .max()
        .unwrap_or(0);
      // We skip the DC coefficient since it has its own quantizer index.
      if eob_minus_one > 0 {
        eob_minus_one + 1
      } else {
        u16::from(qcoeffs[0] != T::cast_from(0))
      }
    };

    // Here we use different rounding biases depending on whether we've
    // had recent coefficients that are larger than one, or less than
    // one. The reason for this is that a block usually has a chunk of
    // large coefficients and a tail of zeroes and ones, and the tradeoffs
    // for coding these two are different. In the tail of zeroes and ones,
    // you'll likely end up spending most bits just saying where that
    // coefficient is in the block, whereas in the chunk of larger
    // coefficients, most bits will be spent on coding its magnitude.
    // To that end, we want to bias more toward rounding to zero for
    // that tail of zeroes and ones than we do for the larger coefficients.
    let mut level_mode = 1;
    let ac_quant = self.ac_quant.get() as u32;
    for &pos in scan.iter().take(usize::from(eob)).skip(1) {
      let coeff = i32::cast_from(coeffs[pos as usize]) << self.log_tx_scale;
      let abs_coeff = coeff.unsigned_abs();

      let level0 = divu_pair(abs_coeff, self.ac_mul_add);
      let offset = if level0 > 1 - level_mode {
        self.ac_offset1
      } else {
        self.ac_offset0
      };

      let abs_qcoeff: u32 =
        level0 + (abs_coeff + offset >= (level0 + 1) * ac_quant) as u32;
      if level_mode != 0 && abs_qcoeff == 0 {
        level_mode = 0;
      } else if abs_qcoeff > 1 {
        level_mode = 1;
      }

      qcoeffs[pos as usize] = T::cast_from(copysign(abs_qcoeff, coeff));
    }

    // Rather than zeroing the tail in scan order, assume that qcoeffs is
    // pre-filled with zeros.

    // Check the eob is correct
    debug_assert_eq!(
      usize::from(eob),
      scan
        .iter()
        .rposition(|&i| qcoeffs[i as usize] != T::cast_from(0))
        .map(|n| n + 1)
        .unwrap_or(0)
    );

    eob
  }
}

pub mod rust {
  use super::*;
  use crate::cpu_features::CpuFeatureLevel;
  use std::mem::MaybeUninit;

  pub fn dequantize<T: Coefficient>(
    qindex: u8, coeffs: &[T], _eob: u16, rcoeffs: &mut [MaybeUninit<T>],
    tx_size: TxSize, bit_depth: usize, dc_delta_q: i8, ac_delta_q: i8,
    _cpu: CpuFeatureLevel,
  ) {
    let log_tx_scale = get_log_tx_scale(tx_size) as i32;
    let offset = (1 << log_tx_scale) - 1;

    let dc_quant = dc_q(qindex, dc_delta_q, bit_depth).get() as i32;
    let ac_quant = ac_q(qindex, ac_delta_q, bit_depth).get() as i32;

    for (i, (r, c)) in rcoeffs
      .iter_mut()
      .zip(coeffs.iter().map(|&c| i32::cast_from(c)))
      .enumerate()
    {
      let quant = if i == 0 { dc_quant } else { ac_quant };
      r.write(T::cast_from(
        (c * quant + ((c >> 31) & offset)) >> log_tx_scale,
      ));
    }
  }
}
