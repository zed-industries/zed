// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::ec::rust;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[inline(always)]
pub fn update_cdf<const N: usize>(cdf: &mut [u16; N], val: u32) {
  if cdf.len() == 4 {
    // SAFETY: Calls Assembly code, which is only valid when the length of
    // `cdf` is 4.
    return unsafe {
      update_cdf_4_sse2(cdf, val);
    };
  }

  rust::update_cdf(cdf, val);
}

#[target_feature(enable = "sse2")]
#[inline]
unsafe fn update_cdf_4_sse2(cdf: &mut [u16], val: u32) {
  let nsymbs = 4;
  let rate = 5 + (cdf[nsymbs - 1] >> 4) as usize;
  let count = cdf[nsymbs - 1] + (cdf[nsymbs - 1] < 32) as u16;

  // A bit of explanation of what is happening down here. First of all, let's look at the simple
  // implementation:
  //
  // ```
  // if i as u32 >= val {
  //   *v -= *v >> rate;
  // } else {
  //   *v += (32768 - *v) >> rate;
  // }
  // ```
  //
  // We want to perform the same arithmetic operation in the two branches, therefore we can
  // transform in something like:
  //
  // ```
  // if i as u32 >= val {
  //   *v -= *v >> rate;
  // } else {
  //   *v -= -((32768 - *v) >> rate);
  // }
  // ```
  //
  // It is possible to bring the "minus" for the second branch logically before the right shift
  // using the following rule:
  // -(x >> y) = (-1 - x) >> y + 1
  // So we obtain
  //
  // ```
  // if i as u32 >= val {
  //   *v -= *v >> rate;
  // } else {
  //   *v -= (-1 - (32768 - *v)) >> rate + 1;
  // }
  // ```
  //
  // Good. A range `0..4` can be compared against `val` in order to have a starting point to work
  // in different ways on the two branches. `cmplt` returns `-1` if `lhs < rhs`, 0 otherwise.
  // It is possible to use the `avg` SIMD operator, which performs `(lhs + rhs + 1) >> 1`. This is
  // useful because `avg` treats numbers as unsigned, `-1 = 0xFFFF`, therefore `(0xFFFF + 0 + 1) >>
  // 1 = 0x8000 = 32768`. Obviously `(0 + 0 + 1) >> 1 = 0`.
  //
  // Now the result of `cmplt` can be used along with the result from `avg` and the data in `cdf`
  // in order to obtain the right hand side of the subtraction from `cdf`.

  let val_splat = _mm_set1_epi16(val as i16);
  let indices = _mm_set_epi16(0, 0, 0, 0, 3, 2, 1, 0);
  let index_lt_val = _mm_cmplt_epi16(indices, val_splat);
  let k = _mm_avg_epu16(index_lt_val, _mm_setzero_si128());
  let cdf_simd = _mm_loadl_epi64(cdf.as_mut_ptr() as *const __m128i);
  let k_minus_v = _mm_sub_epi16(k, cdf_simd);
  let negated_if_lt_val = _mm_sub_epi16(index_lt_val, k_minus_v);
  let shifted =
    _mm_sra_epi16(negated_if_lt_val, _mm_set_epi32(0, 0, 0, rate as i32));
  let fixed_if_lt_val = _mm_sub_epi16(shifted, index_lt_val);
  let result = _mm_sub_epi16(cdf_simd, fixed_if_lt_val);

  _mm_storel_epi64(cdf.as_mut_ptr() as *mut __m128i, result);
  cdf[nsymbs - 1] = count;
}

#[cfg(test)]
mod test {
  use crate::ec::rust;

  #[test]
  fn update_cdf_4_sse2() {
    let mut cdf = [7296, 3819, 1616, 0];
    let mut cdf2 = [7296, 3819, 1616, 0];
    for i in 0..4 {
      rust::update_cdf(&mut cdf, i);
      // SAFETY: We are only testing on cdfs of size 4
      unsafe {
        super::update_cdf_4_sse2(&mut cdf2, i);
      }
      assert_eq!(cdf, cdf2);
    }

    let mut cdf = [7297, 3820, 1617, 0];
    let mut cdf2 = cdf;
    for i in 0..4 {
      rust::update_cdf(&mut cdf, i);
      // SAFETY: We are only testing on cdfs of size 4
      unsafe {
        super::update_cdf_4_sse2(&mut cdf2, i);
      }
      assert_eq!(cdf, cdf2);
    }
  }
}
