// Copyright (c) 2018-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

cfg_if::cfg_if! {
  if #[cfg(nasm_x86_64)] {
    pub use crate::asm::x86::transform::inverse::*;
  } else if #[cfg(asm_neon)] {
    pub use crate::asm::aarch64::transform::inverse::*;
  } else {
    pub use self::rust::*;
  }
}

use crate::tiling::PlaneRegionMut;
use crate::util::*;

// TODO: move 1d txfm code to rust module.

use super::clamp_value;
use super::consts::*;
use super::get_1d_tx_types;
use super::half_btf;
use super::TxSize;
use super::TxType;

/// # Panics
///
/// - If `input` or `output` have fewer than 4 items.
pub fn av1_iwht4(input: &[i32], output: &mut [i32], _range: usize) {
  assert!(input.len() >= 4);
  assert!(output.len() >= 4);

  // <https://aomediacodec.github.io/av1-spec/#inverse-walsh-hadamard-transform-process>
  let x0 = input[0];
  let x1 = input[1];
  let x2 = input[2];
  let x3 = input[3];
  let s0 = x0 + x1;
  let s2 = x2 - x3;
  let s4 = (s0 - s2) >> 1;
  let s3 = s4 - x3;
  let s1 = s4 - x1;
  output[0] = s0 - s3;
  output[1] = s3;
  output[2] = s1;
  output[3] = s2 + s1;
}

static COSPI_INV: [i32; 64] = [
  4096, 4095, 4091, 4085, 4076, 4065, 4052, 4036, 4017, 3996, 3973, 3948,
  3920, 3889, 3857, 3822, 3784, 3745, 3703, 3659, 3612, 3564, 3513, 3461,
  3406, 3349, 3290, 3229, 3166, 3102, 3035, 2967, 2896, 2824, 2751, 2675,
  2598, 2520, 2440, 2359, 2276, 2191, 2106, 2019, 1931, 1842, 1751, 1660,
  1567, 1474, 1380, 1285, 1189, 1092, 995, 897, 799, 700, 601, 501, 401, 301,
  201, 101,
];

static SINPI_INV: [i32; 5] = [0, 1321, 2482, 3344, 3803];

const INV_COS_BIT: usize = 12;

/// # Panics
///
/// - If `input` or `output` have fewer than 4 items.
pub fn av1_idct4(input: &[i32], output: &mut [i32], range: usize) {
  assert!(input.len() >= 4);
  assert!(output.len() >= 4);

  // stage 1
  let stg1 = [input[0], input[2], input[1], input[3]];

  // stage 2
  let stg2 = [
    half_btf(COSPI_INV[32], stg1[0], COSPI_INV[32], stg1[1], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg1[0], -COSPI_INV[32], stg1[1], INV_COS_BIT),
    half_btf(COSPI_INV[48], stg1[2], -COSPI_INV[16], stg1[3], INV_COS_BIT),
    half_btf(COSPI_INV[16], stg1[2], COSPI_INV[48], stg1[3], INV_COS_BIT),
  ];

  // stage 3
  output[0] = clamp_value(stg2[0] + stg2[3], range);
  output[1] = clamp_value(stg2[1] + stg2[2], range);
  output[2] = clamp_value(stg2[1] - stg2[2], range);
  output[3] = clamp_value(stg2[0] - stg2[3], range);
}

pub fn av1_iflipadst4(input: &[i32], output: &mut [i32], range: usize) {
  av1_iadst4(input, output, range);
  output[..4].reverse();
}

/// # Panics
///
/// - If `input` or `output` have fewer than 4 items.
#[inline(always)]
pub fn av1_iadst4(input: &[i32], output: &mut [i32], _range: usize) {
  assert!(input.len() >= 4);
  assert!(output.len() >= 4);

  let bit = 12;

  let x0 = input[0];
  let x1 = input[1];
  let x2 = input[2];
  let x3 = input[3];

  // stage 1
  let s0 = SINPI_INV[1] * x0;
  let s1 = SINPI_INV[2] * x0;
  let s2 = SINPI_INV[3] * x1;
  let s3 = SINPI_INV[4] * x2;
  let s4 = SINPI_INV[1] * x2;
  let s5 = SINPI_INV[2] * x3;
  let s6 = SINPI_INV[4] * x3;

  // stage 2
  let s7 = (x0 - x2) + x3;

  // stage 3
  let s0 = s0 + s3;
  let s1 = s1 - s4;
  let s3 = s2;
  let s2 = SINPI_INV[3] * s7;

  // stage 4
  let s0 = s0 + s5;
  let s1 = s1 - s6;

  // stage 5
  let x0 = s0 + s3;
  let x1 = s1 + s3;
  let x2 = s2;
  let x3 = s0 + s1;

  // stage 6
  let x3 = x3 - s3;

  output[0] = round_shift(x0, bit);
  output[1] = round_shift(x1, bit);
  output[2] = round_shift(x2, bit);
  output[3] = round_shift(x3, bit);
}

pub fn av1_iidentity4(input: &[i32], output: &mut [i32], _range: usize) {
  output[..4]
    .iter_mut()
    .zip(input[..4].iter())
    .for_each(|(outp, inp)| *outp = round_shift(SQRT2 * *inp, 12));
}

/// # Panics
///
/// - If `input` or `output` have fewer than 8 items.
pub fn av1_idct8(input: &[i32], output: &mut [i32], range: usize) {
  assert!(input.len() >= 8);
  assert!(output.len() >= 8);

  // call idct4
  let temp_in = [input[0], input[2], input[4], input[6]];
  let mut temp_out: [i32; 4] = [0; 4];
  av1_idct4(&temp_in, &mut temp_out, range);

  // stage 0

  // stage 1
  let stg1 = [input[1], input[5], input[3], input[7]];

  // stage 2
  let stg2 = [
    half_btf(COSPI_INV[56], stg1[0], -COSPI_INV[8], stg1[3], INV_COS_BIT),
    half_btf(COSPI_INV[24], stg1[1], -COSPI_INV[40], stg1[2], INV_COS_BIT),
    half_btf(COSPI_INV[40], stg1[1], COSPI_INV[24], stg1[2], INV_COS_BIT),
    half_btf(COSPI_INV[8], stg1[0], COSPI_INV[56], stg1[3], INV_COS_BIT),
  ];

  // stage 3
  let stg3 = [
    clamp_value(stg2[0] + stg2[1], range),
    clamp_value(stg2[0] - stg2[1], range),
    clamp_value(-stg2[2] + stg2[3], range),
    clamp_value(stg2[2] + stg2[3], range),
  ];

  // stage 4
  let stg4 = [
    stg3[0],
    half_btf(-COSPI_INV[32], stg3[1], COSPI_INV[32], stg3[2], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg3[1], COSPI_INV[32], stg3[2], INV_COS_BIT),
    stg3[3],
  ];

  // stage 5
  output[0] = clamp_value(temp_out[0] + stg4[3], range);
  output[1] = clamp_value(temp_out[1] + stg4[2], range);
  output[2] = clamp_value(temp_out[2] + stg4[1], range);
  output[3] = clamp_value(temp_out[3] + stg4[0], range);
  output[4] = clamp_value(temp_out[3] - stg4[0], range);
  output[5] = clamp_value(temp_out[2] - stg4[1], range);
  output[6] = clamp_value(temp_out[1] - stg4[2], range);
  output[7] = clamp_value(temp_out[0] - stg4[3], range);
}

pub fn av1_iflipadst8(input: &[i32], output: &mut [i32], range: usize) {
  av1_iadst8(input, output, range);
  output[..8].reverse();
}

/// # Panics
///
/// - If `input` or `output` have fewer than 8 items.
#[inline(always)]
pub fn av1_iadst8(input: &[i32], output: &mut [i32], range: usize) {
  assert!(input.len() >= 8);
  assert!(output.len() >= 8);

  // stage 1
  let stg1 = [
    input[7], input[0], input[5], input[2], input[3], input[4], input[1],
    input[6],
  ];

  // stage 2
  let stg2 = [
    half_btf(COSPI_INV[4], stg1[0], COSPI_INV[60], stg1[1], INV_COS_BIT),
    half_btf(COSPI_INV[60], stg1[0], -COSPI_INV[4], stg1[1], INV_COS_BIT),
    half_btf(COSPI_INV[20], stg1[2], COSPI_INV[44], stg1[3], INV_COS_BIT),
    half_btf(COSPI_INV[44], stg1[2], -COSPI_INV[20], stg1[3], INV_COS_BIT),
    half_btf(COSPI_INV[36], stg1[4], COSPI_INV[28], stg1[5], INV_COS_BIT),
    half_btf(COSPI_INV[28], stg1[4], -COSPI_INV[36], stg1[5], INV_COS_BIT),
    half_btf(COSPI_INV[52], stg1[6], COSPI_INV[12], stg1[7], INV_COS_BIT),
    half_btf(COSPI_INV[12], stg1[6], -COSPI_INV[52], stg1[7], INV_COS_BIT),
  ];

  // stage 3
  let stg3 = [
    clamp_value(stg2[0] + stg2[4], range),
    clamp_value(stg2[1] + stg2[5], range),
    clamp_value(stg2[2] + stg2[6], range),
    clamp_value(stg2[3] + stg2[7], range),
    clamp_value(stg2[0] - stg2[4], range),
    clamp_value(stg2[1] - stg2[5], range),
    clamp_value(stg2[2] - stg2[6], range),
    clamp_value(stg2[3] - stg2[7], range),
  ];

  // stage 4
  let stg4 = [
    stg3[0],
    stg3[1],
    stg3[2],
    stg3[3],
    half_btf(COSPI_INV[16], stg3[4], COSPI_INV[48], stg3[5], INV_COS_BIT),
    half_btf(COSPI_INV[48], stg3[4], -COSPI_INV[16], stg3[5], INV_COS_BIT),
    half_btf(-COSPI_INV[48], stg3[6], COSPI_INV[16], stg3[7], INV_COS_BIT),
    half_btf(COSPI_INV[16], stg3[6], COSPI_INV[48], stg3[7], INV_COS_BIT),
  ];

  // stage 5
  let stg5 = [
    clamp_value(stg4[0] + stg4[2], range),
    clamp_value(stg4[1] + stg4[3], range),
    clamp_value(stg4[0] - stg4[2], range),
    clamp_value(stg4[1] - stg4[3], range),
    clamp_value(stg4[4] + stg4[6], range),
    clamp_value(stg4[5] + stg4[7], range),
    clamp_value(stg4[4] - stg4[6], range),
    clamp_value(stg4[5] - stg4[7], range),
  ];

  // stage 6
  let stg6 = [
    stg5[0],
    stg5[1],
    half_btf(COSPI_INV[32], stg5[2], COSPI_INV[32], stg5[3], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg5[2], -COSPI_INV[32], stg5[3], INV_COS_BIT),
    stg5[4],
    stg5[5],
    half_btf(COSPI_INV[32], stg5[6], COSPI_INV[32], stg5[7], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg5[6], -COSPI_INV[32], stg5[7], INV_COS_BIT),
  ];

  // stage 7
  output[0] = stg6[0];
  output[1] = -stg6[4];
  output[2] = stg6[6];
  output[3] = -stg6[2];
  output[4] = stg6[3];
  output[5] = -stg6[7];
  output[6] = stg6[5];
  output[7] = -stg6[1];
}

pub fn av1_iidentity8(input: &[i32], output: &mut [i32], _range: usize) {
  output[..8]
    .iter_mut()
    .zip(input[..8].iter())
    .for_each(|(outp, inp)| *outp = 2 * *inp);
}

fn av1_idct16(input: &[i32], output: &mut [i32], range: usize) {
  assert!(input.len() >= 16);
  assert!(output.len() >= 16);

  // call idct8
  let temp_in = [
    input[0], input[2], input[4], input[6], input[8], input[10], input[12],
    input[14],
  ];
  let mut temp_out: [i32; 8] = [0; 8];
  av1_idct8(&temp_in, &mut temp_out, range);

  // stage 1
  let stg1 = [
    input[1], input[9], input[5], input[13], input[3], input[11], input[7],
    input[15],
  ];

  // stage 2
  let stg2 = [
    half_btf(COSPI_INV[60], stg1[0], -COSPI_INV[4], stg1[7], INV_COS_BIT),
    half_btf(COSPI_INV[28], stg1[1], -COSPI_INV[36], stg1[6], INV_COS_BIT),
    half_btf(COSPI_INV[44], stg1[2], -COSPI_INV[20], stg1[5], INV_COS_BIT),
    half_btf(COSPI_INV[12], stg1[3], -COSPI_INV[52], stg1[4], INV_COS_BIT),
    half_btf(COSPI_INV[52], stg1[3], COSPI_INV[12], stg1[4], INV_COS_BIT),
    half_btf(COSPI_INV[20], stg1[2], COSPI_INV[44], stg1[5], INV_COS_BIT),
    half_btf(COSPI_INV[36], stg1[1], COSPI_INV[28], stg1[6], INV_COS_BIT),
    half_btf(COSPI_INV[4], stg1[0], COSPI_INV[60], stg1[7], INV_COS_BIT),
  ];

  // stage 3
  let stg3 = [
    clamp_value(stg2[0] + stg2[1], range),
    clamp_value(stg2[0] - stg2[1], range),
    clamp_value(-stg2[2] + stg2[3], range),
    clamp_value(stg2[2] + stg2[3], range),
    clamp_value(stg2[4] + stg2[5], range),
    clamp_value(stg2[4] - stg2[5], range),
    clamp_value(-stg2[6] + stg2[7], range),
    clamp_value(stg2[6] + stg2[7], range),
  ];

  // stage 4
  let stg4 = [
    stg3[0],
    half_btf(-COSPI_INV[16], stg3[1], COSPI_INV[48], stg3[6], INV_COS_BIT),
    half_btf(-COSPI_INV[48], stg3[2], -COSPI_INV[16], stg3[5], INV_COS_BIT),
    stg3[3],
    stg3[4],
    half_btf(-COSPI_INV[16], stg3[2], COSPI_INV[48], stg3[5], INV_COS_BIT),
    half_btf(COSPI_INV[48], stg3[1], COSPI_INV[16], stg3[6], INV_COS_BIT),
    stg3[7],
  ];

  // stage 5
  let stg5 = [
    clamp_value(stg4[0] + stg4[3], range),
    clamp_value(stg4[1] + stg4[2], range),
    clamp_value(stg4[1] - stg4[2], range),
    clamp_value(stg4[0] - stg4[3], range),
    clamp_value(-stg4[4] + stg4[7], range),
    clamp_value(-stg4[5] + stg4[6], range),
    clamp_value(stg4[5] + stg4[6], range),
    clamp_value(stg4[4] + stg4[7], range),
  ];

  // stage 6
  let stg6 = [
    stg5[0],
    stg5[1],
    half_btf(-COSPI_INV[32], stg5[2], COSPI_INV[32], stg5[5], INV_COS_BIT),
    half_btf(-COSPI_INV[32], stg5[3], COSPI_INV[32], stg5[4], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg5[3], COSPI_INV[32], stg5[4], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg5[2], COSPI_INV[32], stg5[5], INV_COS_BIT),
    stg5[6],
    stg5[7],
  ];

  // stage 7
  output[0] = clamp_value(temp_out[0] + stg6[7], range);
  output[1] = clamp_value(temp_out[1] + stg6[6], range);
  output[2] = clamp_value(temp_out[2] + stg6[5], range);
  output[3] = clamp_value(temp_out[3] + stg6[4], range);
  output[4] = clamp_value(temp_out[4] + stg6[3], range);
  output[5] = clamp_value(temp_out[5] + stg6[2], range);
  output[6] = clamp_value(temp_out[6] + stg6[1], range);
  output[7] = clamp_value(temp_out[7] + stg6[0], range);
  output[8] = clamp_value(temp_out[7] - stg6[0], range);
  output[9] = clamp_value(temp_out[6] - stg6[1], range);
  output[10] = clamp_value(temp_out[5] - stg6[2], range);
  output[11] = clamp_value(temp_out[4] - stg6[3], range);
  output[12] = clamp_value(temp_out[3] - stg6[4], range);
  output[13] = clamp_value(temp_out[2] - stg6[5], range);
  output[14] = clamp_value(temp_out[1] - stg6[6], range);
  output[15] = clamp_value(temp_out[0] - stg6[7], range);
}

pub fn av1_iflipadst16(input: &[i32], output: &mut [i32], range: usize) {
  av1_iadst16(input, output, range);
  output[..16].reverse();
}

#[inline(always)]
fn av1_iadst16(input: &[i32], output: &mut [i32], range: usize) {
  assert!(input.len() >= 16);
  assert!(output.len() >= 16);

  // stage 1
  let stg1 = [
    input[15], input[0], input[13], input[2], input[11], input[4], input[9],
    input[6], input[7], input[8], input[5], input[10], input[3], input[12],
    input[1], input[14],
  ];

  // stage 2
  let stg2 = [
    half_btf(COSPI_INV[2], stg1[0], COSPI_INV[62], stg1[1], INV_COS_BIT),
    half_btf(COSPI_INV[62], stg1[0], -COSPI_INV[2], stg1[1], INV_COS_BIT),
    half_btf(COSPI_INV[10], stg1[2], COSPI_INV[54], stg1[3], INV_COS_BIT),
    half_btf(COSPI_INV[54], stg1[2], -COSPI_INV[10], stg1[3], INV_COS_BIT),
    half_btf(COSPI_INV[18], stg1[4], COSPI_INV[46], stg1[5], INV_COS_BIT),
    half_btf(COSPI_INV[46], stg1[4], -COSPI_INV[18], stg1[5], INV_COS_BIT),
    half_btf(COSPI_INV[26], stg1[6], COSPI_INV[38], stg1[7], INV_COS_BIT),
    half_btf(COSPI_INV[38], stg1[6], -COSPI_INV[26], stg1[7], INV_COS_BIT),
    half_btf(COSPI_INV[34], stg1[8], COSPI_INV[30], stg1[9], INV_COS_BIT),
    half_btf(COSPI_INV[30], stg1[8], -COSPI_INV[34], stg1[9], INV_COS_BIT),
    half_btf(COSPI_INV[42], stg1[10], COSPI_INV[22], stg1[11], INV_COS_BIT),
    half_btf(COSPI_INV[22], stg1[10], -COSPI_INV[42], stg1[11], INV_COS_BIT),
    half_btf(COSPI_INV[50], stg1[12], COSPI_INV[14], stg1[13], INV_COS_BIT),
    half_btf(COSPI_INV[14], stg1[12], -COSPI_INV[50], stg1[13], INV_COS_BIT),
    half_btf(COSPI_INV[58], stg1[14], COSPI_INV[6], stg1[15], INV_COS_BIT),
    half_btf(COSPI_INV[6], stg1[14], -COSPI_INV[58], stg1[15], INV_COS_BIT),
  ];

  // stage 3
  let stg3 = [
    clamp_value(stg2[0] + stg2[8], range),
    clamp_value(stg2[1] + stg2[9], range),
    clamp_value(stg2[2] + stg2[10], range),
    clamp_value(stg2[3] + stg2[11], range),
    clamp_value(stg2[4] + stg2[12], range),
    clamp_value(stg2[5] + stg2[13], range),
    clamp_value(stg2[6] + stg2[14], range),
    clamp_value(stg2[7] + stg2[15], range),
    clamp_value(stg2[0] - stg2[8], range),
    clamp_value(stg2[1] - stg2[9], range),
    clamp_value(stg2[2] - stg2[10], range),
    clamp_value(stg2[3] - stg2[11], range),
    clamp_value(stg2[4] - stg2[12], range),
    clamp_value(stg2[5] - stg2[13], range),
    clamp_value(stg2[6] - stg2[14], range),
    clamp_value(stg2[7] - stg2[15], range),
  ];

  // stage 4
  let stg4 = [
    stg3[0],
    stg3[1],
    stg3[2],
    stg3[3],
    stg3[4],
    stg3[5],
    stg3[6],
    stg3[7],
    half_btf(COSPI_INV[8], stg3[8], COSPI_INV[56], stg3[9], INV_COS_BIT),
    half_btf(COSPI_INV[56], stg3[8], -COSPI_INV[8], stg3[9], INV_COS_BIT),
    half_btf(COSPI_INV[40], stg3[10], COSPI_INV[24], stg3[11], INV_COS_BIT),
    half_btf(COSPI_INV[24], stg3[10], -COSPI_INV[40], stg3[11], INV_COS_BIT),
    half_btf(-COSPI_INV[56], stg3[12], COSPI_INV[8], stg3[13], INV_COS_BIT),
    half_btf(COSPI_INV[8], stg3[12], COSPI_INV[56], stg3[13], INV_COS_BIT),
    half_btf(-COSPI_INV[24], stg3[14], COSPI_INV[40], stg3[15], INV_COS_BIT),
    half_btf(COSPI_INV[40], stg3[14], COSPI_INV[24], stg3[15], INV_COS_BIT),
  ];

  // stage 5
  let stg5 = [
    clamp_value(stg4[0] + stg4[4], range),
    clamp_value(stg4[1] + stg4[5], range),
    clamp_value(stg4[2] + stg4[6], range),
    clamp_value(stg4[3] + stg4[7], range),
    clamp_value(stg4[0] - stg4[4], range),
    clamp_value(stg4[1] - stg4[5], range),
    clamp_value(stg4[2] - stg4[6], range),
    clamp_value(stg4[3] - stg4[7], range),
    clamp_value(stg4[8] + stg4[12], range),
    clamp_value(stg4[9] + stg4[13], range),
    clamp_value(stg4[10] + stg4[14], range),
    clamp_value(stg4[11] + stg4[15], range),
    clamp_value(stg4[8] - stg4[12], range),
    clamp_value(stg4[9] - stg4[13], range),
    clamp_value(stg4[10] - stg4[14], range),
    clamp_value(stg4[11] - stg4[15], range),
  ];

  // stage 6
  let stg6 = [
    stg5[0],
    stg5[1],
    stg5[2],
    stg5[3],
    half_btf(COSPI_INV[16], stg5[4], COSPI_INV[48], stg5[5], INV_COS_BIT),
    half_btf(COSPI_INV[48], stg5[4], -COSPI_INV[16], stg5[5], INV_COS_BIT),
    half_btf(-COSPI_INV[48], stg5[6], COSPI_INV[16], stg5[7], INV_COS_BIT),
    half_btf(COSPI_INV[16], stg5[6], COSPI_INV[48], stg5[7], INV_COS_BIT),
    stg5[8],
    stg5[9],
    stg5[10],
    stg5[11],
    half_btf(COSPI_INV[16], stg5[12], COSPI_INV[48], stg5[13], INV_COS_BIT),
    half_btf(COSPI_INV[48], stg5[12], -COSPI_INV[16], stg5[13], INV_COS_BIT),
    half_btf(-COSPI_INV[48], stg5[14], COSPI_INV[16], stg5[15], INV_COS_BIT),
    half_btf(COSPI_INV[16], stg5[14], COSPI_INV[48], stg5[15], INV_COS_BIT),
  ];

  // stage 7
  let stg7 = [
    clamp_value(stg6[0] + stg6[2], range),
    clamp_value(stg6[1] + stg6[3], range),
    clamp_value(stg6[0] - stg6[2], range),
    clamp_value(stg6[1] - stg6[3], range),
    clamp_value(stg6[4] + stg6[6], range),
    clamp_value(stg6[5] + stg6[7], range),
    clamp_value(stg6[4] - stg6[6], range),
    clamp_value(stg6[5] - stg6[7], range),
    clamp_value(stg6[8] + stg6[10], range),
    clamp_value(stg6[9] + stg6[11], range),
    clamp_value(stg6[8] - stg6[10], range),
    clamp_value(stg6[9] - stg6[11], range),
    clamp_value(stg6[12] + stg6[14], range),
    clamp_value(stg6[13] + stg6[15], range),
    clamp_value(stg6[12] - stg6[14], range),
    clamp_value(stg6[13] - stg6[15], range),
  ];

  // stage 8
  let stg8 = [
    stg7[0],
    stg7[1],
    half_btf(COSPI_INV[32], stg7[2], COSPI_INV[32], stg7[3], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg7[2], -COSPI_INV[32], stg7[3], INV_COS_BIT),
    stg7[4],
    stg7[5],
    half_btf(COSPI_INV[32], stg7[6], COSPI_INV[32], stg7[7], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg7[6], -COSPI_INV[32], stg7[7], INV_COS_BIT),
    stg7[8],
    stg7[9],
    half_btf(COSPI_INV[32], stg7[10], COSPI_INV[32], stg7[11], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg7[10], -COSPI_INV[32], stg7[11], INV_COS_BIT),
    stg7[12],
    stg7[13],
    half_btf(COSPI_INV[32], stg7[14], COSPI_INV[32], stg7[15], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg7[14], -COSPI_INV[32], stg7[15], INV_COS_BIT),
  ];

  // stage 9
  output[0] = stg8[0];
  output[1] = -stg8[8];
  output[2] = stg8[12];
  output[3] = -stg8[4];
  output[4] = stg8[6];
  output[5] = -stg8[14];
  output[6] = stg8[10];
  output[7] = -stg8[2];
  output[8] = stg8[3];
  output[9] = -stg8[11];
  output[10] = stg8[15];
  output[11] = -stg8[7];
  output[12] = stg8[5];
  output[13] = -stg8[13];
  output[14] = stg8[9];
  output[15] = -stg8[1];
}

fn av1_iidentity16(input: &[i32], output: &mut [i32], _range: usize) {
  output[..16]
    .iter_mut()
    .zip(input[..16].iter())
    .for_each(|(outp, inp)| *outp = round_shift(SQRT2 * 2 * *inp, 12));
}

fn av1_idct32(input: &[i32], output: &mut [i32], range: usize) {
  assert!(input.len() >= 32);
  assert!(output.len() >= 32);

  // stage 1;
  let stg1 = [
    input[0], input[16], input[8], input[24], input[4], input[20], input[12],
    input[28], input[2], input[18], input[10], input[26], input[6], input[22],
    input[14], input[30], input[1], input[17], input[9], input[25], input[5],
    input[21], input[13], input[29], input[3], input[19], input[11],
    input[27], input[7], input[23], input[15], input[31],
  ];

  // stage 2
  let stg2 = [
    stg1[0],
    stg1[1],
    stg1[2],
    stg1[3],
    stg1[4],
    stg1[5],
    stg1[6],
    stg1[7],
    stg1[8],
    stg1[9],
    stg1[10],
    stg1[11],
    stg1[12],
    stg1[13],
    stg1[14],
    stg1[15],
    half_btf(COSPI_INV[62], stg1[16], -COSPI_INV[2], stg1[31], INV_COS_BIT),
    half_btf(COSPI_INV[30], stg1[17], -COSPI_INV[34], stg1[30], INV_COS_BIT),
    half_btf(COSPI_INV[46], stg1[18], -COSPI_INV[18], stg1[29], INV_COS_BIT),
    half_btf(COSPI_INV[14], stg1[19], -COSPI_INV[50], stg1[28], INV_COS_BIT),
    half_btf(COSPI_INV[54], stg1[20], -COSPI_INV[10], stg1[27], INV_COS_BIT),
    half_btf(COSPI_INV[22], stg1[21], -COSPI_INV[42], stg1[26], INV_COS_BIT),
    half_btf(COSPI_INV[38], stg1[22], -COSPI_INV[26], stg1[25], INV_COS_BIT),
    half_btf(COSPI_INV[6], stg1[23], -COSPI_INV[58], stg1[24], INV_COS_BIT),
    half_btf(COSPI_INV[58], stg1[23], COSPI_INV[6], stg1[24], INV_COS_BIT),
    half_btf(COSPI_INV[26], stg1[22], COSPI_INV[38], stg1[25], INV_COS_BIT),
    half_btf(COSPI_INV[42], stg1[21], COSPI_INV[22], stg1[26], INV_COS_BIT),
    half_btf(COSPI_INV[10], stg1[20], COSPI_INV[54], stg1[27], INV_COS_BIT),
    half_btf(COSPI_INV[50], stg1[19], COSPI_INV[14], stg1[28], INV_COS_BIT),
    half_btf(COSPI_INV[18], stg1[18], COSPI_INV[46], stg1[29], INV_COS_BIT),
    half_btf(COSPI_INV[34], stg1[17], COSPI_INV[30], stg1[30], INV_COS_BIT),
    half_btf(COSPI_INV[2], stg1[16], COSPI_INV[62], stg1[31], INV_COS_BIT),
  ];

  // stage 3
  let stg3 = [
    stg2[0],
    stg2[1],
    stg2[2],
    stg2[3],
    stg2[4],
    stg2[5],
    stg2[6],
    stg2[7],
    half_btf(COSPI_INV[60], stg2[8], -COSPI_INV[4], stg2[15], INV_COS_BIT),
    half_btf(COSPI_INV[28], stg2[9], -COSPI_INV[36], stg2[14], INV_COS_BIT),
    half_btf(COSPI_INV[44], stg2[10], -COSPI_INV[20], stg2[13], INV_COS_BIT),
    half_btf(COSPI_INV[12], stg2[11], -COSPI_INV[52], stg2[12], INV_COS_BIT),
    half_btf(COSPI_INV[52], stg2[11], COSPI_INV[12], stg2[12], INV_COS_BIT),
    half_btf(COSPI_INV[20], stg2[10], COSPI_INV[44], stg2[13], INV_COS_BIT),
    half_btf(COSPI_INV[36], stg2[9], COSPI_INV[28], stg2[14], INV_COS_BIT),
    half_btf(COSPI_INV[4], stg2[8], COSPI_INV[60], stg2[15], INV_COS_BIT),
    clamp_value(stg2[16] + stg2[17], range),
    clamp_value(stg2[16] - stg2[17], range),
    clamp_value(-stg2[18] + stg2[19], range),
    clamp_value(stg2[18] + stg2[19], range),
    clamp_value(stg2[20] + stg2[21], range),
    clamp_value(stg2[20] - stg2[21], range),
    clamp_value(-stg2[22] + stg2[23], range),
    clamp_value(stg2[22] + stg2[23], range),
    clamp_value(stg2[24] + stg2[25], range),
    clamp_value(stg2[24] - stg2[25], range),
    clamp_value(-stg2[26] + stg2[27], range),
    clamp_value(stg2[26] + stg2[27], range),
    clamp_value(stg2[28] + stg2[29], range),
    clamp_value(stg2[28] - stg2[29], range),
    clamp_value(-stg2[30] + stg2[31], range),
    clamp_value(stg2[30] + stg2[31], range),
  ];

  // stage 4
  let stg4 = [
    stg3[0],
    stg3[1],
    stg3[2],
    stg3[3],
    half_btf(COSPI_INV[56], stg3[4], -COSPI_INV[8], stg3[7], INV_COS_BIT),
    half_btf(COSPI_INV[24], stg3[5], -COSPI_INV[40], stg3[6], INV_COS_BIT),
    half_btf(COSPI_INV[40], stg3[5], COSPI_INV[24], stg3[6], INV_COS_BIT),
    half_btf(COSPI_INV[8], stg3[4], COSPI_INV[56], stg3[7], INV_COS_BIT),
    clamp_value(stg3[8] + stg3[9], range),
    clamp_value(stg3[8] - stg3[9], range),
    clamp_value(-stg3[10] + stg3[11], range),
    clamp_value(stg3[10] + stg3[11], range),
    clamp_value(stg3[12] + stg3[13], range),
    clamp_value(stg3[12] - stg3[13], range),
    clamp_value(-stg3[14] + stg3[15], range),
    clamp_value(stg3[14] + stg3[15], range),
    stg3[16],
    half_btf(-COSPI_INV[8], stg3[17], COSPI_INV[56], stg3[30], INV_COS_BIT),
    half_btf(-COSPI_INV[56], stg3[18], -COSPI_INV[8], stg3[29], INV_COS_BIT),
    stg3[19],
    stg3[20],
    half_btf(-COSPI_INV[40], stg3[21], COSPI_INV[24], stg3[26], INV_COS_BIT),
    half_btf(-COSPI_INV[24], stg3[22], -COSPI_INV[40], stg3[25], INV_COS_BIT),
    stg3[23],
    stg3[24],
    half_btf(-COSPI_INV[40], stg3[22], COSPI_INV[24], stg3[25], INV_COS_BIT),
    half_btf(COSPI_INV[24], stg3[21], COSPI_INV[40], stg3[26], INV_COS_BIT),
    stg3[27],
    stg3[28],
    half_btf(-COSPI_INV[8], stg3[18], COSPI_INV[56], stg3[29], INV_COS_BIT),
    half_btf(COSPI_INV[56], stg3[17], COSPI_INV[8], stg3[30], INV_COS_BIT),
    stg3[31],
  ];

  // stage 5
  let stg5 = [
    half_btf(COSPI_INV[32], stg4[0], COSPI_INV[32], stg4[1], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg4[0], -COSPI_INV[32], stg4[1], INV_COS_BIT),
    half_btf(COSPI_INV[48], stg4[2], -COSPI_INV[16], stg4[3], INV_COS_BIT),
    half_btf(COSPI_INV[16], stg4[2], COSPI_INV[48], stg4[3], INV_COS_BIT),
    clamp_value(stg4[4] + stg4[5], range),
    clamp_value(stg4[4] - stg4[5], range),
    clamp_value(-stg4[6] + stg4[7], range),
    clamp_value(stg4[6] + stg4[7], range),
    stg4[8],
    half_btf(-COSPI_INV[16], stg4[9], COSPI_INV[48], stg4[14], INV_COS_BIT),
    half_btf(-COSPI_INV[48], stg4[10], -COSPI_INV[16], stg4[13], INV_COS_BIT),
    stg4[11],
    stg4[12],
    half_btf(-COSPI_INV[16], stg4[10], COSPI_INV[48], stg4[13], INV_COS_BIT),
    half_btf(COSPI_INV[48], stg4[9], COSPI_INV[16], stg4[14], INV_COS_BIT),
    stg4[15],
    clamp_value(stg4[16] + stg4[19], range),
    clamp_value(stg4[17] + stg4[18], range),
    clamp_value(stg4[17] - stg4[18], range),
    clamp_value(stg4[16] - stg4[19], range),
    clamp_value(-stg4[20] + stg4[23], range),
    clamp_value(-stg4[21] + stg4[22], range),
    clamp_value(stg4[21] + stg4[22], range),
    clamp_value(stg4[20] + stg4[23], range),
    clamp_value(stg4[24] + stg4[27], range),
    clamp_value(stg4[25] + stg4[26], range),
    clamp_value(stg4[25] - stg4[26], range),
    clamp_value(stg4[24] - stg4[27], range),
    clamp_value(-stg4[28] + stg4[31], range),
    clamp_value(-stg4[29] + stg4[30], range),
    clamp_value(stg4[29] + stg4[30], range),
    clamp_value(stg4[28] + stg4[31], range),
  ];

  // stage 6
  let stg6 = [
    clamp_value(stg5[0] + stg5[3], range),
    clamp_value(stg5[1] + stg5[2], range),
    clamp_value(stg5[1] - stg5[2], range),
    clamp_value(stg5[0] - stg5[3], range),
    stg5[4],
    half_btf(-COSPI_INV[32], stg5[5], COSPI_INV[32], stg5[6], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg5[5], COSPI_INV[32], stg5[6], INV_COS_BIT),
    stg5[7],
    clamp_value(stg5[8] + stg5[11], range),
    clamp_value(stg5[9] + stg5[10], range),
    clamp_value(stg5[9] - stg5[10], range),
    clamp_value(stg5[8] - stg5[11], range),
    clamp_value(-stg5[12] + stg5[15], range),
    clamp_value(-stg5[13] + stg5[14], range),
    clamp_value(stg5[13] + stg5[14], range),
    clamp_value(stg5[12] + stg5[15], range),
    stg5[16],
    stg5[17],
    half_btf(-COSPI_INV[16], stg5[18], COSPI_INV[48], stg5[29], INV_COS_BIT),
    half_btf(-COSPI_INV[16], stg5[19], COSPI_INV[48], stg5[28], INV_COS_BIT),
    half_btf(-COSPI_INV[48], stg5[20], -COSPI_INV[16], stg5[27], INV_COS_BIT),
    half_btf(-COSPI_INV[48], stg5[21], -COSPI_INV[16], stg5[26], INV_COS_BIT),
    stg5[22],
    stg5[23],
    stg5[24],
    stg5[25],
    half_btf(-COSPI_INV[16], stg5[21], COSPI_INV[48], stg5[26], INV_COS_BIT),
    half_btf(-COSPI_INV[16], stg5[20], COSPI_INV[48], stg5[27], INV_COS_BIT),
    half_btf(COSPI_INV[48], stg5[19], COSPI_INV[16], stg5[28], INV_COS_BIT),
    half_btf(COSPI_INV[48], stg5[18], COSPI_INV[16], stg5[29], INV_COS_BIT),
    stg5[30],
    stg5[31],
  ];

  // stage 7
  let stg7 = [
    clamp_value(stg6[0] + stg6[7], range),
    clamp_value(stg6[1] + stg6[6], range),
    clamp_value(stg6[2] + stg6[5], range),
    clamp_value(stg6[3] + stg6[4], range),
    clamp_value(stg6[3] - stg6[4], range),
    clamp_value(stg6[2] - stg6[5], range),
    clamp_value(stg6[1] - stg6[6], range),
    clamp_value(stg6[0] - stg6[7], range),
    stg6[8],
    stg6[9],
    half_btf(-COSPI_INV[32], stg6[10], COSPI_INV[32], stg6[13], INV_COS_BIT),
    half_btf(-COSPI_INV[32], stg6[11], COSPI_INV[32], stg6[12], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg6[11], COSPI_INV[32], stg6[12], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg6[10], COSPI_INV[32], stg6[13], INV_COS_BIT),
    stg6[14],
    stg6[15],
    clamp_value(stg6[16] + stg6[23], range),
    clamp_value(stg6[17] + stg6[22], range),
    clamp_value(stg6[18] + stg6[21], range),
    clamp_value(stg6[19] + stg6[20], range),
    clamp_value(stg6[19] - stg6[20], range),
    clamp_value(stg6[18] - stg6[21], range),
    clamp_value(stg6[17] - stg6[22], range),
    clamp_value(stg6[16] - stg6[23], range),
    clamp_value(-stg6[24] + stg6[31], range),
    clamp_value(-stg6[25] + stg6[30], range),
    clamp_value(-stg6[26] + stg6[29], range),
    clamp_value(-stg6[27] + stg6[28], range),
    clamp_value(stg6[27] + stg6[28], range),
    clamp_value(stg6[26] + stg6[29], range),
    clamp_value(stg6[25] + stg6[30], range),
    clamp_value(stg6[24] + stg6[31], range),
  ];

  // stage 8
  let stg8 = [
    clamp_value(stg7[0] + stg7[15], range),
    clamp_value(stg7[1] + stg7[14], range),
    clamp_value(stg7[2] + stg7[13], range),
    clamp_value(stg7[3] + stg7[12], range),
    clamp_value(stg7[4] + stg7[11], range),
    clamp_value(stg7[5] + stg7[10], range),
    clamp_value(stg7[6] + stg7[9], range),
    clamp_value(stg7[7] + stg7[8], range),
    clamp_value(stg7[7] - stg7[8], range),
    clamp_value(stg7[6] - stg7[9], range),
    clamp_value(stg7[5] - stg7[10], range),
    clamp_value(stg7[4] - stg7[11], range),
    clamp_value(stg7[3] - stg7[12], range),
    clamp_value(stg7[2] - stg7[13], range),
    clamp_value(stg7[1] - stg7[14], range),
    clamp_value(stg7[0] - stg7[15], range),
    stg7[16],
    stg7[17],
    stg7[18],
    stg7[19],
    half_btf(-COSPI_INV[32], stg7[20], COSPI_INV[32], stg7[27], INV_COS_BIT),
    half_btf(-COSPI_INV[32], stg7[21], COSPI_INV[32], stg7[26], INV_COS_BIT),
    half_btf(-COSPI_INV[32], stg7[22], COSPI_INV[32], stg7[25], INV_COS_BIT),
    half_btf(-COSPI_INV[32], stg7[23], COSPI_INV[32], stg7[24], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg7[23], COSPI_INV[32], stg7[24], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg7[22], COSPI_INV[32], stg7[25], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg7[21], COSPI_INV[32], stg7[26], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg7[20], COSPI_INV[32], stg7[27], INV_COS_BIT),
    stg7[28],
    stg7[29],
    stg7[30],
    stg7[31],
  ];

  // stage 9
  output[0] = clamp_value(stg8[0] + stg8[31], range);
  output[1] = clamp_value(stg8[1] + stg8[30], range);
  output[2] = clamp_value(stg8[2] + stg8[29], range);
  output[3] = clamp_value(stg8[3] + stg8[28], range);
  output[4] = clamp_value(stg8[4] + stg8[27], range);
  output[5] = clamp_value(stg8[5] + stg8[26], range);
  output[6] = clamp_value(stg8[6] + stg8[25], range);
  output[7] = clamp_value(stg8[7] + stg8[24], range);
  output[8] = clamp_value(stg8[8] + stg8[23], range);
  output[9] = clamp_value(stg8[9] + stg8[22], range);
  output[10] = clamp_value(stg8[10] + stg8[21], range);
  output[11] = clamp_value(stg8[11] + stg8[20], range);
  output[12] = clamp_value(stg8[12] + stg8[19], range);
  output[13] = clamp_value(stg8[13] + stg8[18], range);
  output[14] = clamp_value(stg8[14] + stg8[17], range);
  output[15] = clamp_value(stg8[15] + stg8[16], range);
  output[16] = clamp_value(stg8[15] - stg8[16], range);
  output[17] = clamp_value(stg8[14] - stg8[17], range);
  output[18] = clamp_value(stg8[13] - stg8[18], range);
  output[19] = clamp_value(stg8[12] - stg8[19], range);
  output[20] = clamp_value(stg8[11] - stg8[20], range);
  output[21] = clamp_value(stg8[10] - stg8[21], range);
  output[22] = clamp_value(stg8[9] - stg8[22], range);
  output[23] = clamp_value(stg8[8] - stg8[23], range);
  output[24] = clamp_value(stg8[7] - stg8[24], range);
  output[25] = clamp_value(stg8[6] - stg8[25], range);
  output[26] = clamp_value(stg8[5] - stg8[26], range);
  output[27] = clamp_value(stg8[4] - stg8[27], range);
  output[28] = clamp_value(stg8[3] - stg8[28], range);
  output[29] = clamp_value(stg8[2] - stg8[29], range);
  output[30] = clamp_value(stg8[1] - stg8[30], range);
  output[31] = clamp_value(stg8[0] - stg8[31], range);
}

fn av1_iidentity32(input: &[i32], output: &mut [i32], _range: usize) {
  output[..32]
    .iter_mut()
    .zip(input[..32].iter())
    .for_each(|(outp, inp)| *outp = 4 * *inp);
}

fn av1_idct64(input: &[i32], output: &mut [i32], range: usize) {
  assert!(input.len() >= 64);
  assert!(output.len() >= 64);

  // stage 1;
  let stg1 = [
    input[0], input[32], input[16], input[48], input[8], input[40], input[24],
    input[56], input[4], input[36], input[20], input[52], input[12],
    input[44], input[28], input[60], input[2], input[34], input[18],
    input[50], input[10], input[42], input[26], input[58], input[6],
    input[38], input[22], input[54], input[14], input[46], input[30],
    input[62], input[1], input[33], input[17], input[49], input[9], input[41],
    input[25], input[57], input[5], input[37], input[21], input[53],
    input[13], input[45], input[29], input[61], input[3], input[35],
    input[19], input[51], input[11], input[43], input[27], input[59],
    input[7], input[39], input[23], input[55], input[15], input[47],
    input[31], input[63],
  ];

  // stage 2
  let stg2 = [
    stg1[0],
    stg1[1],
    stg1[2],
    stg1[3],
    stg1[4],
    stg1[5],
    stg1[6],
    stg1[7],
    stg1[8],
    stg1[9],
    stg1[10],
    stg1[11],
    stg1[12],
    stg1[13],
    stg1[14],
    stg1[15],
    stg1[16],
    stg1[17],
    stg1[18],
    stg1[19],
    stg1[20],
    stg1[21],
    stg1[22],
    stg1[23],
    stg1[24],
    stg1[25],
    stg1[26],
    stg1[27],
    stg1[28],
    stg1[29],
    stg1[30],
    stg1[31],
    half_btf(COSPI_INV[63], stg1[32], -COSPI_INV[1], stg1[63], INV_COS_BIT),
    half_btf(COSPI_INV[31], stg1[33], -COSPI_INV[33], stg1[62], INV_COS_BIT),
    half_btf(COSPI_INV[47], stg1[34], -COSPI_INV[17], stg1[61], INV_COS_BIT),
    half_btf(COSPI_INV[15], stg1[35], -COSPI_INV[49], stg1[60], INV_COS_BIT),
    half_btf(COSPI_INV[55], stg1[36], -COSPI_INV[9], stg1[59], INV_COS_BIT),
    half_btf(COSPI_INV[23], stg1[37], -COSPI_INV[41], stg1[58], INV_COS_BIT),
    half_btf(COSPI_INV[39], stg1[38], -COSPI_INV[25], stg1[57], INV_COS_BIT),
    half_btf(COSPI_INV[7], stg1[39], -COSPI_INV[57], stg1[56], INV_COS_BIT),
    half_btf(COSPI_INV[59], stg1[40], -COSPI_INV[5], stg1[55], INV_COS_BIT),
    half_btf(COSPI_INV[27], stg1[41], -COSPI_INV[37], stg1[54], INV_COS_BIT),
    half_btf(COSPI_INV[43], stg1[42], -COSPI_INV[21], stg1[53], INV_COS_BIT),
    half_btf(COSPI_INV[11], stg1[43], -COSPI_INV[53], stg1[52], INV_COS_BIT),
    half_btf(COSPI_INV[51], stg1[44], -COSPI_INV[13], stg1[51], INV_COS_BIT),
    half_btf(COSPI_INV[19], stg1[45], -COSPI_INV[45], stg1[50], INV_COS_BIT),
    half_btf(COSPI_INV[35], stg1[46], -COSPI_INV[29], stg1[49], INV_COS_BIT),
    half_btf(COSPI_INV[3], stg1[47], -COSPI_INV[61], stg1[48], INV_COS_BIT),
    half_btf(COSPI_INV[61], stg1[47], COSPI_INV[3], stg1[48], INV_COS_BIT),
    half_btf(COSPI_INV[29], stg1[46], COSPI_INV[35], stg1[49], INV_COS_BIT),
    half_btf(COSPI_INV[45], stg1[45], COSPI_INV[19], stg1[50], INV_COS_BIT),
    half_btf(COSPI_INV[13], stg1[44], COSPI_INV[51], stg1[51], INV_COS_BIT),
    half_btf(COSPI_INV[53], stg1[43], COSPI_INV[11], stg1[52], INV_COS_BIT),
    half_btf(COSPI_INV[21], stg1[42], COSPI_INV[43], stg1[53], INV_COS_BIT),
    half_btf(COSPI_INV[37], stg1[41], COSPI_INV[27], stg1[54], INV_COS_BIT),
    half_btf(COSPI_INV[5], stg1[40], COSPI_INV[59], stg1[55], INV_COS_BIT),
    half_btf(COSPI_INV[57], stg1[39], COSPI_INV[7], stg1[56], INV_COS_BIT),
    half_btf(COSPI_INV[25], stg1[38], COSPI_INV[39], stg1[57], INV_COS_BIT),
    half_btf(COSPI_INV[41], stg1[37], COSPI_INV[23], stg1[58], INV_COS_BIT),
    half_btf(COSPI_INV[9], stg1[36], COSPI_INV[55], stg1[59], INV_COS_BIT),
    half_btf(COSPI_INV[49], stg1[35], COSPI_INV[15], stg1[60], INV_COS_BIT),
    half_btf(COSPI_INV[17], stg1[34], COSPI_INV[47], stg1[61], INV_COS_BIT),
    half_btf(COSPI_INV[33], stg1[33], COSPI_INV[31], stg1[62], INV_COS_BIT),
    half_btf(COSPI_INV[1], stg1[32], COSPI_INV[63], stg1[63], INV_COS_BIT),
  ];

  // stage 3
  let stg3 = [
    stg2[0],
    stg2[1],
    stg2[2],
    stg2[3],
    stg2[4],
    stg2[5],
    stg2[6],
    stg2[7],
    stg2[8],
    stg2[9],
    stg2[10],
    stg2[11],
    stg2[12],
    stg2[13],
    stg2[14],
    stg2[15],
    half_btf(COSPI_INV[62], stg2[16], -COSPI_INV[2], stg2[31], INV_COS_BIT),
    half_btf(COSPI_INV[30], stg2[17], -COSPI_INV[34], stg2[30], INV_COS_BIT),
    half_btf(COSPI_INV[46], stg2[18], -COSPI_INV[18], stg2[29], INV_COS_BIT),
    half_btf(COSPI_INV[14], stg2[19], -COSPI_INV[50], stg2[28], INV_COS_BIT),
    half_btf(COSPI_INV[54], stg2[20], -COSPI_INV[10], stg2[27], INV_COS_BIT),
    half_btf(COSPI_INV[22], stg2[21], -COSPI_INV[42], stg2[26], INV_COS_BIT),
    half_btf(COSPI_INV[38], stg2[22], -COSPI_INV[26], stg2[25], INV_COS_BIT),
    half_btf(COSPI_INV[6], stg2[23], -COSPI_INV[58], stg2[24], INV_COS_BIT),
    half_btf(COSPI_INV[58], stg2[23], COSPI_INV[6], stg2[24], INV_COS_BIT),
    half_btf(COSPI_INV[26], stg2[22], COSPI_INV[38], stg2[25], INV_COS_BIT),
    half_btf(COSPI_INV[42], stg2[21], COSPI_INV[22], stg2[26], INV_COS_BIT),
    half_btf(COSPI_INV[10], stg2[20], COSPI_INV[54], stg2[27], INV_COS_BIT),
    half_btf(COSPI_INV[50], stg2[19], COSPI_INV[14], stg2[28], INV_COS_BIT),
    half_btf(COSPI_INV[18], stg2[18], COSPI_INV[46], stg2[29], INV_COS_BIT),
    half_btf(COSPI_INV[34], stg2[17], COSPI_INV[30], stg2[30], INV_COS_BIT),
    half_btf(COSPI_INV[2], stg2[16], COSPI_INV[62], stg2[31], INV_COS_BIT),
    clamp_value(stg2[32] + stg2[33], range),
    clamp_value(stg2[32] - stg2[33], range),
    clamp_value(-stg2[34] + stg2[35], range),
    clamp_value(stg2[34] + stg2[35], range),
    clamp_value(stg2[36] + stg2[37], range),
    clamp_value(stg2[36] - stg2[37], range),
    clamp_value(-stg2[38] + stg2[39], range),
    clamp_value(stg2[38] + stg2[39], range),
    clamp_value(stg2[40] + stg2[41], range),
    clamp_value(stg2[40] - stg2[41], range),
    clamp_value(-stg2[42] + stg2[43], range),
    clamp_value(stg2[42] + stg2[43], range),
    clamp_value(stg2[44] + stg2[45], range),
    clamp_value(stg2[44] - stg2[45], range),
    clamp_value(-stg2[46] + stg2[47], range),
    clamp_value(stg2[46] + stg2[47], range),
    clamp_value(stg2[48] + stg2[49], range),
    clamp_value(stg2[48] - stg2[49], range),
    clamp_value(-stg2[50] + stg2[51], range),
    clamp_value(stg2[50] + stg2[51], range),
    clamp_value(stg2[52] + stg2[53], range),
    clamp_value(stg2[52] - stg2[53], range),
    clamp_value(-stg2[54] + stg2[55], range),
    clamp_value(stg2[54] + stg2[55], range),
    clamp_value(stg2[56] + stg2[57], range),
    clamp_value(stg2[56] - stg2[57], range),
    clamp_value(-stg2[58] + stg2[59], range),
    clamp_value(stg2[58] + stg2[59], range),
    clamp_value(stg2[60] + stg2[61], range),
    clamp_value(stg2[60] - stg2[61], range),
    clamp_value(-stg2[62] + stg2[63], range),
    clamp_value(stg2[62] + stg2[63], range),
  ];

  // stage 4
  let stg4 = [
    stg3[0],
    stg3[1],
    stg3[2],
    stg3[3],
    stg3[4],
    stg3[5],
    stg3[6],
    stg3[7],
    half_btf(COSPI_INV[60], stg3[8], -COSPI_INV[4], stg3[15], INV_COS_BIT),
    half_btf(COSPI_INV[28], stg3[9], -COSPI_INV[36], stg3[14], INV_COS_BIT),
    half_btf(COSPI_INV[44], stg3[10], -COSPI_INV[20], stg3[13], INV_COS_BIT),
    half_btf(COSPI_INV[12], stg3[11], -COSPI_INV[52], stg3[12], INV_COS_BIT),
    half_btf(COSPI_INV[52], stg3[11], COSPI_INV[12], stg3[12], INV_COS_BIT),
    half_btf(COSPI_INV[20], stg3[10], COSPI_INV[44], stg3[13], INV_COS_BIT),
    half_btf(COSPI_INV[36], stg3[9], COSPI_INV[28], stg3[14], INV_COS_BIT),
    half_btf(COSPI_INV[4], stg3[8], COSPI_INV[60], stg3[15], INV_COS_BIT),
    clamp_value(stg3[16] + stg3[17], range),
    clamp_value(stg3[16] - stg3[17], range),
    clamp_value(-stg3[18] + stg3[19], range),
    clamp_value(stg3[18] + stg3[19], range),
    clamp_value(stg3[20] + stg3[21], range),
    clamp_value(stg3[20] - stg3[21], range),
    clamp_value(-stg3[22] + stg3[23], range),
    clamp_value(stg3[22] + stg3[23], range),
    clamp_value(stg3[24] + stg3[25], range),
    clamp_value(stg3[24] - stg3[25], range),
    clamp_value(-stg3[26] + stg3[27], range),
    clamp_value(stg3[26] + stg3[27], range),
    clamp_value(stg3[28] + stg3[29], range),
    clamp_value(stg3[28] - stg3[29], range),
    clamp_value(-stg3[30] + stg3[31], range),
    clamp_value(stg3[30] + stg3[31], range),
    stg3[32],
    half_btf(-COSPI_INV[4], stg3[33], COSPI_INV[60], stg3[62], INV_COS_BIT),
    half_btf(-COSPI_INV[60], stg3[34], -COSPI_INV[4], stg3[61], INV_COS_BIT),
    stg3[35],
    stg3[36],
    half_btf(-COSPI_INV[36], stg3[37], COSPI_INV[28], stg3[58], INV_COS_BIT),
    half_btf(-COSPI_INV[28], stg3[38], -COSPI_INV[36], stg3[57], INV_COS_BIT),
    stg3[39],
    stg3[40],
    half_btf(-COSPI_INV[20], stg3[41], COSPI_INV[44], stg3[54], INV_COS_BIT),
    half_btf(-COSPI_INV[44], stg3[42], -COSPI_INV[20], stg3[53], INV_COS_BIT),
    stg3[43],
    stg3[44],
    half_btf(-COSPI_INV[52], stg3[45], COSPI_INV[12], stg3[50], INV_COS_BIT),
    half_btf(-COSPI_INV[12], stg3[46], -COSPI_INV[52], stg3[49], INV_COS_BIT),
    stg3[47],
    stg3[48],
    half_btf(-COSPI_INV[52], stg3[46], COSPI_INV[12], stg3[49], INV_COS_BIT),
    half_btf(COSPI_INV[12], stg3[45], COSPI_INV[52], stg3[50], INV_COS_BIT),
    stg3[51],
    stg3[52],
    half_btf(-COSPI_INV[20], stg3[42], COSPI_INV[44], stg3[53], INV_COS_BIT),
    half_btf(COSPI_INV[44], stg3[41], COSPI_INV[20], stg3[54], INV_COS_BIT),
    stg3[55],
    stg3[56],
    half_btf(-COSPI_INV[36], stg3[38], COSPI_INV[28], stg3[57], INV_COS_BIT),
    half_btf(COSPI_INV[28], stg3[37], COSPI_INV[36], stg3[58], INV_COS_BIT),
    stg3[59],
    stg3[60],
    half_btf(-COSPI_INV[4], stg3[34], COSPI_INV[60], stg3[61], INV_COS_BIT),
    half_btf(COSPI_INV[60], stg3[33], COSPI_INV[4], stg3[62], INV_COS_BIT),
    stg3[63],
  ];

  // stage 5
  let stg5 = [
    stg4[0],
    stg4[1],
    stg4[2],
    stg4[3],
    half_btf(COSPI_INV[56], stg4[4], -COSPI_INV[8], stg4[7], INV_COS_BIT),
    half_btf(COSPI_INV[24], stg4[5], -COSPI_INV[40], stg4[6], INV_COS_BIT),
    half_btf(COSPI_INV[40], stg4[5], COSPI_INV[24], stg4[6], INV_COS_BIT),
    half_btf(COSPI_INV[8], stg4[4], COSPI_INV[56], stg4[7], INV_COS_BIT),
    clamp_value(stg4[8] + stg4[9], range),
    clamp_value(stg4[8] - stg4[9], range),
    clamp_value(-stg4[10] + stg4[11], range),
    clamp_value(stg4[10] + stg4[11], range),
    clamp_value(stg4[12] + stg4[13], range),
    clamp_value(stg4[12] - stg4[13], range),
    clamp_value(-stg4[14] + stg4[15], range),
    clamp_value(stg4[14] + stg4[15], range),
    stg4[16],
    half_btf(-COSPI_INV[8], stg4[17], COSPI_INV[56], stg4[30], INV_COS_BIT),
    half_btf(-COSPI_INV[56], stg4[18], -COSPI_INV[8], stg4[29], INV_COS_BIT),
    stg4[19],
    stg4[20],
    half_btf(-COSPI_INV[40], stg4[21], COSPI_INV[24], stg4[26], INV_COS_BIT),
    half_btf(-COSPI_INV[24], stg4[22], -COSPI_INV[40], stg4[25], INV_COS_BIT),
    stg4[23],
    stg4[24],
    half_btf(-COSPI_INV[40], stg4[22], COSPI_INV[24], stg4[25], INV_COS_BIT),
    half_btf(COSPI_INV[24], stg4[21], COSPI_INV[40], stg4[26], INV_COS_BIT),
    stg4[27],
    stg4[28],
    half_btf(-COSPI_INV[8], stg4[18], COSPI_INV[56], stg4[29], INV_COS_BIT),
    half_btf(COSPI_INV[56], stg4[17], COSPI_INV[8], stg4[30], INV_COS_BIT),
    stg4[31],
    clamp_value(stg4[32] + stg4[35], range),
    clamp_value(stg4[33] + stg4[34], range),
    clamp_value(stg4[33] - stg4[34], range),
    clamp_value(stg4[32] - stg4[35], range),
    clamp_value(-stg4[36] + stg4[39], range),
    clamp_value(-stg4[37] + stg4[38], range),
    clamp_value(stg4[37] + stg4[38], range),
    clamp_value(stg4[36] + stg4[39], range),
    clamp_value(stg4[40] + stg4[43], range),
    clamp_value(stg4[41] + stg4[42], range),
    clamp_value(stg4[41] - stg4[42], range),
    clamp_value(stg4[40] - stg4[43], range),
    clamp_value(-stg4[44] + stg4[47], range),
    clamp_value(-stg4[45] + stg4[46], range),
    clamp_value(stg4[45] + stg4[46], range),
    clamp_value(stg4[44] + stg4[47], range),
    clamp_value(stg4[48] + stg4[51], range),
    clamp_value(stg4[49] + stg4[50], range),
    clamp_value(stg4[49] - stg4[50], range),
    clamp_value(stg4[48] - stg4[51], range),
    clamp_value(-stg4[52] + stg4[55], range),
    clamp_value(-stg4[53] + stg4[54], range),
    clamp_value(stg4[53] + stg4[54], range),
    clamp_value(stg4[52] + stg4[55], range),
    clamp_value(stg4[56] + stg4[59], range),
    clamp_value(stg4[57] + stg4[58], range),
    clamp_value(stg4[57] - stg4[58], range),
    clamp_value(stg4[56] - stg4[59], range),
    clamp_value(-stg4[60] + stg4[63], range),
    clamp_value(-stg4[61] + stg4[62], range),
    clamp_value(stg4[61] + stg4[62], range),
    clamp_value(stg4[60] + stg4[63], range),
  ];

  // stage 6
  let stg6 = [
    half_btf(COSPI_INV[32], stg5[0], COSPI_INV[32], stg5[1], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg5[0], -COSPI_INV[32], stg5[1], INV_COS_BIT),
    half_btf(COSPI_INV[48], stg5[2], -COSPI_INV[16], stg5[3], INV_COS_BIT),
    half_btf(COSPI_INV[16], stg5[2], COSPI_INV[48], stg5[3], INV_COS_BIT),
    clamp_value(stg5[4] + stg5[5], range),
    clamp_value(stg5[4] - stg5[5], range),
    clamp_value(-stg5[6] + stg5[7], range),
    clamp_value(stg5[6] + stg5[7], range),
    stg5[8],
    half_btf(-COSPI_INV[16], stg5[9], COSPI_INV[48], stg5[14], INV_COS_BIT),
    half_btf(-COSPI_INV[48], stg5[10], -COSPI_INV[16], stg5[13], INV_COS_BIT),
    stg5[11],
    stg5[12],
    half_btf(-COSPI_INV[16], stg5[10], COSPI_INV[48], stg5[13], INV_COS_BIT),
    half_btf(COSPI_INV[48], stg5[9], COSPI_INV[16], stg5[14], INV_COS_BIT),
    stg5[15],
    clamp_value(stg5[16] + stg5[19], range),
    clamp_value(stg5[17] + stg5[18], range),
    clamp_value(stg5[17] - stg5[18], range),
    clamp_value(stg5[16] - stg5[19], range),
    clamp_value(-stg5[20] + stg5[23], range),
    clamp_value(-stg5[21] + stg5[22], range),
    clamp_value(stg5[21] + stg5[22], range),
    clamp_value(stg5[20] + stg5[23], range),
    clamp_value(stg5[24] + stg5[27], range),
    clamp_value(stg5[25] + stg5[26], range),
    clamp_value(stg5[25] - stg5[26], range),
    clamp_value(stg5[24] - stg5[27], range),
    clamp_value(-stg5[28] + stg5[31], range),
    clamp_value(-stg5[29] + stg5[30], range),
    clamp_value(stg5[29] + stg5[30], range),
    clamp_value(stg5[28] + stg5[31], range),
    stg5[32],
    stg5[33],
    half_btf(-COSPI_INV[8], stg5[34], COSPI_INV[56], stg5[61], INV_COS_BIT),
    half_btf(-COSPI_INV[8], stg5[35], COSPI_INV[56], stg5[60], INV_COS_BIT),
    half_btf(-COSPI_INV[56], stg5[36], -COSPI_INV[8], stg5[59], INV_COS_BIT),
    half_btf(-COSPI_INV[56], stg5[37], -COSPI_INV[8], stg5[58], INV_COS_BIT),
    stg5[38],
    stg5[39],
    stg5[40],
    stg5[41],
    half_btf(-COSPI_INV[40], stg5[42], COSPI_INV[24], stg5[53], INV_COS_BIT),
    half_btf(-COSPI_INV[40], stg5[43], COSPI_INV[24], stg5[52], INV_COS_BIT),
    half_btf(-COSPI_INV[24], stg5[44], -COSPI_INV[40], stg5[51], INV_COS_BIT),
    half_btf(-COSPI_INV[24], stg5[45], -COSPI_INV[40], stg5[50], INV_COS_BIT),
    stg5[46],
    stg5[47],
    stg5[48],
    stg5[49],
    half_btf(-COSPI_INV[40], stg5[45], COSPI_INV[24], stg5[50], INV_COS_BIT),
    half_btf(-COSPI_INV[40], stg5[44], COSPI_INV[24], stg5[51], INV_COS_BIT),
    half_btf(COSPI_INV[24], stg5[43], COSPI_INV[40], stg5[52], INV_COS_BIT),
    half_btf(COSPI_INV[24], stg5[42], COSPI_INV[40], stg5[53], INV_COS_BIT),
    stg5[54],
    stg5[55],
    stg5[56],
    stg5[57],
    half_btf(-COSPI_INV[8], stg5[37], COSPI_INV[56], stg5[58], INV_COS_BIT),
    half_btf(-COSPI_INV[8], stg5[36], COSPI_INV[56], stg5[59], INV_COS_BIT),
    half_btf(COSPI_INV[56], stg5[35], COSPI_INV[8], stg5[60], INV_COS_BIT),
    half_btf(COSPI_INV[56], stg5[34], COSPI_INV[8], stg5[61], INV_COS_BIT),
    stg5[62],
    stg5[63],
  ];

  // stage 7
  let stg7 = [
    clamp_value(stg6[0] + stg6[3], range),
    clamp_value(stg6[1] + stg6[2], range),
    clamp_value(stg6[1] - stg6[2], range),
    clamp_value(stg6[0] - stg6[3], range),
    stg6[4],
    half_btf(-COSPI_INV[32], stg6[5], COSPI_INV[32], stg6[6], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg6[5], COSPI_INV[32], stg6[6], INV_COS_BIT),
    stg6[7],
    clamp_value(stg6[8] + stg6[11], range),
    clamp_value(stg6[9] + stg6[10], range),
    clamp_value(stg6[9] - stg6[10], range),
    clamp_value(stg6[8] - stg6[11], range),
    clamp_value(-stg6[12] + stg6[15], range),
    clamp_value(-stg6[13] + stg6[14], range),
    clamp_value(stg6[13] + stg6[14], range),
    clamp_value(stg6[12] + stg6[15], range),
    stg6[16],
    stg6[17],
    half_btf(-COSPI_INV[16], stg6[18], COSPI_INV[48], stg6[29], INV_COS_BIT),
    half_btf(-COSPI_INV[16], stg6[19], COSPI_INV[48], stg6[28], INV_COS_BIT),
    half_btf(-COSPI_INV[48], stg6[20], -COSPI_INV[16], stg6[27], INV_COS_BIT),
    half_btf(-COSPI_INV[48], stg6[21], -COSPI_INV[16], stg6[26], INV_COS_BIT),
    stg6[22],
    stg6[23],
    stg6[24],
    stg6[25],
    half_btf(-COSPI_INV[16], stg6[21], COSPI_INV[48], stg6[26], INV_COS_BIT),
    half_btf(-COSPI_INV[16], stg6[20], COSPI_INV[48], stg6[27], INV_COS_BIT),
    half_btf(COSPI_INV[48], stg6[19], COSPI_INV[16], stg6[28], INV_COS_BIT),
    half_btf(COSPI_INV[48], stg6[18], COSPI_INV[16], stg6[29], INV_COS_BIT),
    stg6[30],
    stg6[31],
    clamp_value(stg6[32] + stg6[39], range),
    clamp_value(stg6[33] + stg6[38], range),
    clamp_value(stg6[34] + stg6[37], range),
    clamp_value(stg6[35] + stg6[36], range),
    clamp_value(stg6[35] - stg6[36], range),
    clamp_value(stg6[34] - stg6[37], range),
    clamp_value(stg6[33] - stg6[38], range),
    clamp_value(stg6[32] - stg6[39], range),
    clamp_value(-stg6[40] + stg6[47], range),
    clamp_value(-stg6[41] + stg6[46], range),
    clamp_value(-stg6[42] + stg6[45], range),
    clamp_value(-stg6[43] + stg6[44], range),
    clamp_value(stg6[43] + stg6[44], range),
    clamp_value(stg6[42] + stg6[45], range),
    clamp_value(stg6[41] + stg6[46], range),
    clamp_value(stg6[40] + stg6[47], range),
    clamp_value(stg6[48] + stg6[55], range),
    clamp_value(stg6[49] + stg6[54], range),
    clamp_value(stg6[50] + stg6[53], range),
    clamp_value(stg6[51] + stg6[52], range),
    clamp_value(stg6[51] - stg6[52], range),
    clamp_value(stg6[50] - stg6[53], range),
    clamp_value(stg6[49] - stg6[54], range),
    clamp_value(stg6[48] - stg6[55], range),
    clamp_value(-stg6[56] + stg6[63], range),
    clamp_value(-stg6[57] + stg6[62], range),
    clamp_value(-stg6[58] + stg6[61], range),
    clamp_value(-stg6[59] + stg6[60], range),
    clamp_value(stg6[59] + stg6[60], range),
    clamp_value(stg6[58] + stg6[61], range),
    clamp_value(stg6[57] + stg6[62], range),
    clamp_value(stg6[56] + stg6[63], range),
  ];

  // stage 8
  let stg8 = [
    clamp_value(stg7[0] + stg7[7], range),
    clamp_value(stg7[1] + stg7[6], range),
    clamp_value(stg7[2] + stg7[5], range),
    clamp_value(stg7[3] + stg7[4], range),
    clamp_value(stg7[3] - stg7[4], range),
    clamp_value(stg7[2] - stg7[5], range),
    clamp_value(stg7[1] - stg7[6], range),
    clamp_value(stg7[0] - stg7[7], range),
    stg7[8],
    stg7[9],
    half_btf(-COSPI_INV[32], stg7[10], COSPI_INV[32], stg7[13], INV_COS_BIT),
    half_btf(-COSPI_INV[32], stg7[11], COSPI_INV[32], stg7[12], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg7[11], COSPI_INV[32], stg7[12], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg7[10], COSPI_INV[32], stg7[13], INV_COS_BIT),
    stg7[14],
    stg7[15],
    clamp_value(stg7[16] + stg7[23], range),
    clamp_value(stg7[17] + stg7[22], range),
    clamp_value(stg7[18] + stg7[21], range),
    clamp_value(stg7[19] + stg7[20], range),
    clamp_value(stg7[19] - stg7[20], range),
    clamp_value(stg7[18] - stg7[21], range),
    clamp_value(stg7[17] - stg7[22], range),
    clamp_value(stg7[16] - stg7[23], range),
    clamp_value(-stg7[24] + stg7[31], range),
    clamp_value(-stg7[25] + stg7[30], range),
    clamp_value(-stg7[26] + stg7[29], range),
    clamp_value(-stg7[27] + stg7[28], range),
    clamp_value(stg7[27] + stg7[28], range),
    clamp_value(stg7[26] + stg7[29], range),
    clamp_value(stg7[25] + stg7[30], range),
    clamp_value(stg7[24] + stg7[31], range),
    stg7[32],
    stg7[33],
    stg7[34],
    stg7[35],
    half_btf(-COSPI_INV[16], stg7[36], COSPI_INV[48], stg7[59], INV_COS_BIT),
    half_btf(-COSPI_INV[16], stg7[37], COSPI_INV[48], stg7[58], INV_COS_BIT),
    half_btf(-COSPI_INV[16], stg7[38], COSPI_INV[48], stg7[57], INV_COS_BIT),
    half_btf(-COSPI_INV[16], stg7[39], COSPI_INV[48], stg7[56], INV_COS_BIT),
    half_btf(-COSPI_INV[48], stg7[40], -COSPI_INV[16], stg7[55], INV_COS_BIT),
    half_btf(-COSPI_INV[48], stg7[41], -COSPI_INV[16], stg7[54], INV_COS_BIT),
    half_btf(-COSPI_INV[48], stg7[42], -COSPI_INV[16], stg7[53], INV_COS_BIT),
    half_btf(-COSPI_INV[48], stg7[43], -COSPI_INV[16], stg7[52], INV_COS_BIT),
    stg7[44],
    stg7[45],
    stg7[46],
    stg7[47],
    stg7[48],
    stg7[49],
    stg7[50],
    stg7[51],
    half_btf(-COSPI_INV[16], stg7[43], COSPI_INV[48], stg7[52], INV_COS_BIT),
    half_btf(-COSPI_INV[16], stg7[42], COSPI_INV[48], stg7[53], INV_COS_BIT),
    half_btf(-COSPI_INV[16], stg7[41], COSPI_INV[48], stg7[54], INV_COS_BIT),
    half_btf(-COSPI_INV[16], stg7[40], COSPI_INV[48], stg7[55], INV_COS_BIT),
    half_btf(COSPI_INV[48], stg7[39], COSPI_INV[16], stg7[56], INV_COS_BIT),
    half_btf(COSPI_INV[48], stg7[38], COSPI_INV[16], stg7[57], INV_COS_BIT),
    half_btf(COSPI_INV[48], stg7[37], COSPI_INV[16], stg7[58], INV_COS_BIT),
    half_btf(COSPI_INV[48], stg7[36], COSPI_INV[16], stg7[59], INV_COS_BIT),
    stg7[60],
    stg7[61],
    stg7[62],
    stg7[63],
  ];

  // stage 9
  let stg9 = [
    clamp_value(stg8[0] + stg8[15], range),
    clamp_value(stg8[1] + stg8[14], range),
    clamp_value(stg8[2] + stg8[13], range),
    clamp_value(stg8[3] + stg8[12], range),
    clamp_value(stg8[4] + stg8[11], range),
    clamp_value(stg8[5] + stg8[10], range),
    clamp_value(stg8[6] + stg8[9], range),
    clamp_value(stg8[7] + stg8[8], range),
    clamp_value(stg8[7] - stg8[8], range),
    clamp_value(stg8[6] - stg8[9], range),
    clamp_value(stg8[5] - stg8[10], range),
    clamp_value(stg8[4] - stg8[11], range),
    clamp_value(stg8[3] - stg8[12], range),
    clamp_value(stg8[2] - stg8[13], range),
    clamp_value(stg8[1] - stg8[14], range),
    clamp_value(stg8[0] - stg8[15], range),
    stg8[16],
    stg8[17],
    stg8[18],
    stg8[19],
    half_btf(-COSPI_INV[32], stg8[20], COSPI_INV[32], stg8[27], INV_COS_BIT),
    half_btf(-COSPI_INV[32], stg8[21], COSPI_INV[32], stg8[26], INV_COS_BIT),
    half_btf(-COSPI_INV[32], stg8[22], COSPI_INV[32], stg8[25], INV_COS_BIT),
    half_btf(-COSPI_INV[32], stg8[23], COSPI_INV[32], stg8[24], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg8[23], COSPI_INV[32], stg8[24], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg8[22], COSPI_INV[32], stg8[25], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg8[21], COSPI_INV[32], stg8[26], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg8[20], COSPI_INV[32], stg8[27], INV_COS_BIT),
    stg8[28],
    stg8[29],
    stg8[30],
    stg8[31],
    clamp_value(stg8[32] + stg8[47], range),
    clamp_value(stg8[33] + stg8[46], range),
    clamp_value(stg8[34] + stg8[45], range),
    clamp_value(stg8[35] + stg8[44], range),
    clamp_value(stg8[36] + stg8[43], range),
    clamp_value(stg8[37] + stg8[42], range),
    clamp_value(stg8[38] + stg8[41], range),
    clamp_value(stg8[39] + stg8[40], range),
    clamp_value(stg8[39] - stg8[40], range),
    clamp_value(stg8[38] - stg8[41], range),
    clamp_value(stg8[37] - stg8[42], range),
    clamp_value(stg8[36] - stg8[43], range),
    clamp_value(stg8[35] - stg8[44], range),
    clamp_value(stg8[34] - stg8[45], range),
    clamp_value(stg8[33] - stg8[46], range),
    clamp_value(stg8[32] - stg8[47], range),
    clamp_value(-stg8[48] + stg8[63], range),
    clamp_value(-stg8[49] + stg8[62], range),
    clamp_value(-stg8[50] + stg8[61], range),
    clamp_value(-stg8[51] + stg8[60], range),
    clamp_value(-stg8[52] + stg8[59], range),
    clamp_value(-stg8[53] + stg8[58], range),
    clamp_value(-stg8[54] + stg8[57], range),
    clamp_value(-stg8[55] + stg8[56], range),
    clamp_value(stg8[55] + stg8[56], range),
    clamp_value(stg8[54] + stg8[57], range),
    clamp_value(stg8[53] + stg8[58], range),
    clamp_value(stg8[52] + stg8[59], range),
    clamp_value(stg8[51] + stg8[60], range),
    clamp_value(stg8[50] + stg8[61], range),
    clamp_value(stg8[49] + stg8[62], range),
    clamp_value(stg8[48] + stg8[63], range),
  ];

  // stage 10
  let stg10 = [
    clamp_value(stg9[0] + stg9[31], range),
    clamp_value(stg9[1] + stg9[30], range),
    clamp_value(stg9[2] + stg9[29], range),
    clamp_value(stg9[3] + stg9[28], range),
    clamp_value(stg9[4] + stg9[27], range),
    clamp_value(stg9[5] + stg9[26], range),
    clamp_value(stg9[6] + stg9[25], range),
    clamp_value(stg9[7] + stg9[24], range),
    clamp_value(stg9[8] + stg9[23], range),
    clamp_value(stg9[9] + stg9[22], range),
    clamp_value(stg9[10] + stg9[21], range),
    clamp_value(stg9[11] + stg9[20], range),
    clamp_value(stg9[12] + stg9[19], range),
    clamp_value(stg9[13] + stg9[18], range),
    clamp_value(stg9[14] + stg9[17], range),
    clamp_value(stg9[15] + stg9[16], range),
    clamp_value(stg9[15] - stg9[16], range),
    clamp_value(stg9[14] - stg9[17], range),
    clamp_value(stg9[13] - stg9[18], range),
    clamp_value(stg9[12] - stg9[19], range),
    clamp_value(stg9[11] - stg9[20], range),
    clamp_value(stg9[10] - stg9[21], range),
    clamp_value(stg9[9] - stg9[22], range),
    clamp_value(stg9[8] - stg9[23], range),
    clamp_value(stg9[7] - stg9[24], range),
    clamp_value(stg9[6] - stg9[25], range),
    clamp_value(stg9[5] - stg9[26], range),
    clamp_value(stg9[4] - stg9[27], range),
    clamp_value(stg9[3] - stg9[28], range),
    clamp_value(stg9[2] - stg9[29], range),
    clamp_value(stg9[1] - stg9[30], range),
    clamp_value(stg9[0] - stg9[31], range),
    stg9[32],
    stg9[33],
    stg9[34],
    stg9[35],
    stg9[36],
    stg9[37],
    stg9[38],
    stg9[39],
    half_btf(-COSPI_INV[32], stg9[40], COSPI_INV[32], stg9[55], INV_COS_BIT),
    half_btf(-COSPI_INV[32], stg9[41], COSPI_INV[32], stg9[54], INV_COS_BIT),
    half_btf(-COSPI_INV[32], stg9[42], COSPI_INV[32], stg9[53], INV_COS_BIT),
    half_btf(-COSPI_INV[32], stg9[43], COSPI_INV[32], stg9[52], INV_COS_BIT),
    half_btf(-COSPI_INV[32], stg9[44], COSPI_INV[32], stg9[51], INV_COS_BIT),
    half_btf(-COSPI_INV[32], stg9[45], COSPI_INV[32], stg9[50], INV_COS_BIT),
    half_btf(-COSPI_INV[32], stg9[46], COSPI_INV[32], stg9[49], INV_COS_BIT),
    half_btf(-COSPI_INV[32], stg9[47], COSPI_INV[32], stg9[48], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg9[47], COSPI_INV[32], stg9[48], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg9[46], COSPI_INV[32], stg9[49], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg9[45], COSPI_INV[32], stg9[50], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg9[44], COSPI_INV[32], stg9[51], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg9[43], COSPI_INV[32], stg9[52], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg9[42], COSPI_INV[32], stg9[53], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg9[41], COSPI_INV[32], stg9[54], INV_COS_BIT),
    half_btf(COSPI_INV[32], stg9[40], COSPI_INV[32], stg9[55], INV_COS_BIT),
    stg9[56],
    stg9[57],
    stg9[58],
    stg9[59],
    stg9[60],
    stg9[61],
    stg9[62],
    stg9[63],
  ];

  // stage 11
  output[0] = clamp_value(stg10[0] + stg10[63], range);
  output[1] = clamp_value(stg10[1] + stg10[62], range);
  output[2] = clamp_value(stg10[2] + stg10[61], range);
  output[3] = clamp_value(stg10[3] + stg10[60], range);
  output[4] = clamp_value(stg10[4] + stg10[59], range);
  output[5] = clamp_value(stg10[5] + stg10[58], range);
  output[6] = clamp_value(stg10[6] + stg10[57], range);
  output[7] = clamp_value(stg10[7] + stg10[56], range);
  output[8] = clamp_value(stg10[8] + stg10[55], range);
  output[9] = clamp_value(stg10[9] + stg10[54], range);
  output[10] = clamp_value(stg10[10] + stg10[53], range);
  output[11] = clamp_value(stg10[11] + stg10[52], range);
  output[12] = clamp_value(stg10[12] + stg10[51], range);
  output[13] = clamp_value(stg10[13] + stg10[50], range);
  output[14] = clamp_value(stg10[14] + stg10[49], range);
  output[15] = clamp_value(stg10[15] + stg10[48], range);
  output[16] = clamp_value(stg10[16] + stg10[47], range);
  output[17] = clamp_value(stg10[17] + stg10[46], range);
  output[18] = clamp_value(stg10[18] + stg10[45], range);
  output[19] = clamp_value(stg10[19] + stg10[44], range);
  output[20] = clamp_value(stg10[20] + stg10[43], range);
  output[21] = clamp_value(stg10[21] + stg10[42], range);
  output[22] = clamp_value(stg10[22] + stg10[41], range);
  output[23] = clamp_value(stg10[23] + stg10[40], range);
  output[24] = clamp_value(stg10[24] + stg10[39], range);
  output[25] = clamp_value(stg10[25] + stg10[38], range);
  output[26] = clamp_value(stg10[26] + stg10[37], range);
  output[27] = clamp_value(stg10[27] + stg10[36], range);
  output[28] = clamp_value(stg10[28] + stg10[35], range);
  output[29] = clamp_value(stg10[29] + stg10[34], range);
  output[30] = clamp_value(stg10[30] + stg10[33], range);
  output[31] = clamp_value(stg10[31] + stg10[32], range);
  output[32] = clamp_value(stg10[31] - stg10[32], range);
  output[33] = clamp_value(stg10[30] - stg10[33], range);
  output[34] = clamp_value(stg10[29] - stg10[34], range);
  output[35] = clamp_value(stg10[28] - stg10[35], range);
  output[36] = clamp_value(stg10[27] - stg10[36], range);
  output[37] = clamp_value(stg10[26] - stg10[37], range);
  output[38] = clamp_value(stg10[25] - stg10[38], range);
  output[39] = clamp_value(stg10[24] - stg10[39], range);
  output[40] = clamp_value(stg10[23] - stg10[40], range);
  output[41] = clamp_value(stg10[22] - stg10[41], range);
  output[42] = clamp_value(stg10[21] - stg10[42], range);
  output[43] = clamp_value(stg10[20] - stg10[43], range);
  output[44] = clamp_value(stg10[19] - stg10[44], range);
  output[45] = clamp_value(stg10[18] - stg10[45], range);
  output[46] = clamp_value(stg10[17] - stg10[46], range);
  output[47] = clamp_value(stg10[16] - stg10[47], range);
  output[48] = clamp_value(stg10[15] - stg10[48], range);
  output[49] = clamp_value(stg10[14] - stg10[49], range);
  output[50] = clamp_value(stg10[13] - stg10[50], range);
  output[51] = clamp_value(stg10[12] - stg10[51], range);
  output[52] = clamp_value(stg10[11] - stg10[52], range);
  output[53] = clamp_value(stg10[10] - stg10[53], range);
  output[54] = clamp_value(stg10[9] - stg10[54], range);
  output[55] = clamp_value(stg10[8] - stg10[55], range);
  output[56] = clamp_value(stg10[7] - stg10[56], range);
  output[57] = clamp_value(stg10[6] - stg10[57], range);
  output[58] = clamp_value(stg10[5] - stg10[58], range);
  output[59] = clamp_value(stg10[4] - stg10[59], range);
  output[60] = clamp_value(stg10[3] - stg10[60], range);
  output[61] = clamp_value(stg10[2] - stg10[61], range);
  output[62] = clamp_value(stg10[1] - stg10[62], range);
  output[63] = clamp_value(stg10[0] - stg10[63], range);
}

type InvTxfmFn = fn(input: &[i32], output: &mut [i32], range: usize);

static INV_TXFM_FNS: [[InvTxfmFn; 5]; 5] = [
  [av1_idct4, av1_idct8, av1_idct16, av1_idct32, av1_idct64],
  [
    av1_iadst4,
    av1_iadst8,
    av1_iadst16,
    |_, _, _| unimplemented!(),
    |_, _, _| unimplemented!(),
  ],
  [
    av1_iflipadst4,
    av1_iflipadst8,
    av1_iflipadst16,
    |_, _, _| unimplemented!(),
    |_, _, _| unimplemented!(),
  ],
  [
    av1_iidentity4,
    av1_iidentity8,
    av1_iidentity16,
    av1_iidentity32,
    |_, _, _| unimplemented!(),
  ],
  [
    av1_iwht4,
    |_, _, _| unimplemented!(),
    |_, _, _| unimplemented!(),
    |_, _, _| unimplemented!(),
    |_, _, _| unimplemented!(),
  ],
];

pub(crate) mod rust {
  use super::*;
  use crate::cpu_features::CpuFeatureLevel;

  use simd_helpers::cold_for_target_arch;
  use std::cmp;

  #[cold_for_target_arch("x86_64", "aarch64")]
  pub fn inverse_transform_add<T: Pixel>(
    input: &[T::Coeff], output: &mut PlaneRegionMut<'_, T>, _eob: u16,
    tx_size: TxSize, tx_type: TxType, bd: usize, _cpu: CpuFeatureLevel,
  ) {
    let width: usize = tx_size.width();
    let height: usize = tx_size.height();

    // Only use at most 32 columns and 32 rows of input coefficients.
    let input: &[T::Coeff] = &input[..width.min(32) * height.min(32)];

    // For 64 point transforms, rely on the last 32 columns being initialized
    //   to zero for filling out missing input coeffs.
    let mut buffer = vec![0i32; width * height].into_boxed_slice();
    let rect_type = tx_size.rect_ratio_log2();
    let tx_types_1d = get_1d_tx_types(tx_type);
    let lossless = tx_type == TxType::WHT_WHT;

    // perform inv txfm on every row
    let range = bd + 8;
    let txfm_fn =
      INV_TXFM_FNS[tx_types_1d.1 as usize][tx_size.width_log2() - 2];
    // 64 point transforms only signal 32 coeffs. We only take chunks of 32
    //   and skip over the last 32 transforms here.
    for (r, buffer_slice) in (0..height.min(32)).zip(buffer.chunks_mut(width))
    {
      // For 64 point transforms, rely on the last 32 elements being
      //   initialized to zero for filling out the missing coeffs.
      let mut temp_in: [i32; 64] = [0; 64];
      for (raw, clamped) in input[r..]
        .iter()
        .map(|a| i32::cast_from(*a))
        .step_by(height.min(32))
        .zip(temp_in.iter_mut())
      {
        let val = if rect_type.abs() == 1 {
          round_shift(raw * INV_SQRT2, SQRT2_BITS)
        } else if lossless {
          raw >> 2
        } else {
          raw
        };
        *clamped = clamp_value(val, range);
      }
      txfm_fn(&temp_in, buffer_slice, range);
    }

    // perform inv txfm on every col
    let range = cmp::max(bd + 6, 16);
    let txfm_fn =
      INV_TXFM_FNS[tx_types_1d.0 as usize][tx_size.height_log2() - 2];
    for c in 0..width {
      let mut temp_in: [i32; 64] = [0; 64];
      let mut temp_out: [i32; 64] = [0; 64];
      for (raw, clamped) in
        buffer[c..].iter().step_by(width).zip(temp_in.iter_mut())
      {
        *clamped = clamp_value(
          round_shift(*raw, INV_INTERMEDIATE_SHIFTS[tx_size as usize]),
          range,
        );
      }
      txfm_fn(&temp_in, &mut temp_out, range);
      for (temp, out) in temp_out
        .iter()
        .zip(output.rows_iter_mut().map(|row| &mut row[c]).take(height))
      {
        let v: i32 = (*out).as_();
        let r = if lossless { *temp } else { round_shift(*temp, 4) };
        let v = clamp(v + r, 0, (1 << bd) - 1);
        *out = T::cast_from(v);
      }
    }
  }

  /* From AV1 Spec.
  https://aomediacodec.github.io/av1-spec/#2d-inverse-transform-process
  */
  const INV_INTERMEDIATE_SHIFTS: [usize; TxSize::TX_SIZES_ALL] =
    [0, 1, 2, 2, 2, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2];
}
