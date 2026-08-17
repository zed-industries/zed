// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use crate::context::av1_get_coded_tx_size;
use crate::cpu_features::CpuFeatureLevel;
use crate::quantize::*;
use crate::transform::TxSize;
use crate::util::*;
use std::mem::MaybeUninit;

type DequantizeFn = unsafe fn(
  qindex: u8,
  coeffs_ptr: *const i16,
  _eob: u16,
  rcoeffs_ptr: *mut i16,
  tx_size: TxSize,
  bit_depth: usize,
  dc_delta_q: i8,
  ac_delta_q: i8,
);

cpu_function_lookup_table!(
  DEQUANTIZE_FNS: [Option<DequantizeFn>],
  default: None,
  [(AVX2, Some(dequantize_avx2))]
);

#[inline(always)]
pub fn dequantize<T: Coefficient>(
  qindex: u8, coeffs: &[T], eob: u16, rcoeffs: &mut [MaybeUninit<T>],
  tx_size: TxSize, bit_depth: usize, dc_delta_q: i8, ac_delta_q: i8,
  cpu: CpuFeatureLevel,
) {
  let call_rust = |rcoeffs: &mut [MaybeUninit<T>]| {
    crate::quantize::rust::dequantize(
      qindex, coeffs, eob, rcoeffs, tx_size, bit_depth, dc_delta_q,
      ac_delta_q, cpu,
    );
  };

  #[cfg(any(feature = "check_asm", test))]
  let mut ref_rcoeffs = {
    let area = av1_get_coded_tx_size(tx_size).area();
    let mut copy = vec![MaybeUninit::new(T::cast_from(0)); area];
    call_rust(&mut copy);
    copy
  };

  match T::Pixel::type_enum() {
    PixelType::U8 => {
      if let Some(func) = DEQUANTIZE_FNS[cpu.as_index()] {
        // SAFETY: Calls Assembly code.
        unsafe {
          (func)(
            qindex,
            coeffs.as_ptr() as *const _,
            eob,
            rcoeffs.as_mut_ptr() as *mut _,
            tx_size,
            bit_depth,
            dc_delta_q,
            ac_delta_q,
          )
        }
      } else {
        call_rust(rcoeffs)
      }
    }
    PixelType::U16 => call_rust(rcoeffs),
  }

  #[cfg(any(feature = "check_asm", test))]
  {
    let area = av1_get_coded_tx_size(tx_size).area();
    let rcoeffs = unsafe { slice_assume_init_mut(&mut rcoeffs[..area]) };
    let ref_rcoeffs = unsafe { slice_assume_init_mut(&mut ref_rcoeffs[..]) };
    assert_eq!(rcoeffs, ref_rcoeffs);
  }
}

#[target_feature(enable = "avx2")]
unsafe fn dequantize_avx2(
  qindex: u8, coeffs_ptr: *const i16, _eob: u16, rcoeffs_ptr: *mut i16,
  tx_size: TxSize, bit_depth: usize, dc_delta_q: i8, ac_delta_q: i8,
) {
  let log_tx_scale = _mm256_set1_epi32(get_log_tx_scale(tx_size) as i32);

  let quants_ac =
    _mm256_set1_epi32(ac_q(qindex, ac_delta_q, bit_depth).get() as i32);
  // Use the dc quantize as first vector element for the first iteration
  let mut quants = _mm256_insert_epi32(
    quants_ac,
    dc_q(qindex, dc_delta_q, bit_depth).get() as i32,
    0,
  );

  let area: usize = av1_get_coded_tx_size(tx_size).area();
  // Step by 16 (256/16) coefficients for each iteration
  let step: usize = 16;
  assert!(area >= step);

  // Increase the pointers as we iterate
  let mut coeffs_ptr: *const i16 = coeffs_ptr;
  let mut rcoeffs_ptr: *mut i16 = rcoeffs_ptr;
  for _i in (0..area).step_by(step) {
    let coeffs = _mm256_load_si256(coeffs_ptr as *const _);
    let coeffs_abs = _mm256_abs_epi16(coeffs);

    // TODO: Since log_tx_scale is at most 2 and we gain an extra bit by taking
    // the abs value (unless the range is [-(2^15), 2^15 + 1]), it might be
    // possible to perform a 16-bit multiply and get the highest bit by
    // comparing coeffs to (1<<16) / quant. The would cost 1 compare, 1 blend,
    // and 1 add, but would get rid of 1 pack, 2 unpacks, 1 shift, and 1
    // multiply.

    let rcoeffs = _mm256_sign_epi16(
      _mm256_packs_epi32(
        // (abs_coeff * quant) >> log_tx_scale
        _mm256_srlv_epi32(
          _mm256_madd_epi16(
            quants,
            // Convert the first half of each lane to 32-bits
            _mm256_unpacklo_epi16(coeffs_abs, _mm256_setzero_si256()),
          ),
          log_tx_scale,
        ),
        // Second half
        _mm256_srlv_epi32(
          _mm256_madd_epi16(
            quants_ac,
            _mm256_unpackhi_epi16(coeffs_abs, _mm256_setzero_si256()),
          ),
          log_tx_scale,
        ),
      ),
      coeffs,
    );
    _mm256_store_si256(rcoeffs_ptr as *mut _, rcoeffs);

    // Only use a dc quantizer for the first iteration
    quants = quants_ac;
    coeffs_ptr = coeffs_ptr.add(step);
    rcoeffs_ptr = rcoeffs_ptr.add(step);
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use rand::distr::{Distribution, Uniform};
  use rand::{rng, Rng};

  #[test]
  fn dequantize_test() {
    let mut rng = rng();

    use TxSize::*;
    let tx_sizes = [
      TX_4X4, TX_8X8, TX_16X16, TX_32X32, TX_64X64, TX_4X8, TX_8X4, TX_8X16,
      TX_16X8, TX_16X32, TX_32X16, TX_32X64, TX_64X32, TX_4X16, TX_16X4,
      TX_8X32, TX_32X8, TX_16X64, TX_64X16,
    ];

    let bd: usize = 8;

    for &tx_size in &tx_sizes {
      let qindex: u8 = rng.random_range((MINQ as u8)..(MAXQ as u8));
      let dc_quant = dc_q(qindex, 0, bd).get() as i16;
      let ac_quant = ac_q(qindex, 0, bd).get() as i16;

      // Test the min, max, and random eobs
      let eobs = {
        let mut out = [0u16; 16];
        let area: usize = av1_get_coded_tx_size(tx_size).area();
        out[0] = 0;
        out[1] = area as u16;
        for eob in out.iter_mut().skip(2) {
          *eob = rng.random_range(0..area as u16);
        }
        out
      };

      for &eob in &eobs {
        let mut qcoeffs = Aligned::new([0i16; 32 * 32]);
        let mut rcoeffs = Aligned::new([MaybeUninit::new(0i16); 32 * 32]);

        // Generate quantized coefficients up to the eob
        // SAFETY: always correct
        let between = Uniform::new_inclusive(-i16::MAX, i16::MAX).unwrap();
        for (i, qcoeff) in
          qcoeffs.data.iter_mut().enumerate().take(eob as usize)
        {
          *qcoeff = between.sample(&mut rng)
            / if i == 0 { dc_quant } else { ac_quant };
        }

        // Rely on quantize's internal tests
        dequantize(
          qindex,
          &qcoeffs.data,
          eob,
          &mut rcoeffs.data,
          tx_size,
          bd,
          0,
          0,
          CpuFeatureLevel::default(),
        );
      }
    }
  }
}
