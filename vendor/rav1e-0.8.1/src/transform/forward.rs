// Copyright (c) 2018-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::cpu_features::CpuFeatureLevel;
use crate::util::*;

use super::TxType;

cfg_if::cfg_if! {
  if #[cfg(nasm_x86_64)] {
    pub use crate::asm::x86::transform::forward::*;
  } else if #[cfg(asm_neon)] {
    pub use crate::asm::aarch64::transform::forward::*;
  } else {
    pub use self::rust::*;
  }
}

pub mod rust {
  use super::*;
  use std::mem::MaybeUninit;

  use crate::transform::forward_shared::*;
  use crate::transform::{av1_round_shift_array, valid_av1_transform, TxSize};
  use simd_helpers::cold_for_target_arch;

  type TxfmFunc = fn(&mut [i32]);

  impl_1d_tx!();

  impl TxOperations for i32 {
    fn zero() -> Self {
      0
    }

    fn tx_mul<const SHIFT: i32>(self, mul: i32) -> Self {
      ((self * mul) + (1 << SHIFT >> 1)) >> SHIFT
    }

    fn rshift1(self) -> Self {
      (self + i32::from(self < 0)) >> 1
    }

    fn add(self, b: Self) -> Self {
      self + b
    }

    fn sub(self, b: Self) -> Self {
      self - b
    }

    fn add_avg(self, b: Self) -> Self {
      (self + b) >> 1
    }

    fn sub_avg(self, b: Self) -> Self {
      (self - b) >> 1
    }
  }

  /// # Panics
  ///
  /// - If called with an invalid combination of `tx_size` and `tx_type`
  #[cold_for_target_arch("x86_64")]
  pub fn forward_transform<T: Coefficient>(
    input: &[i16], output: &mut [MaybeUninit<T>], stride: usize,
    tx_size: TxSize, tx_type: TxType, bd: usize, _cpu: CpuFeatureLevel,
  ) {
    assert!(valid_av1_transform(tx_size, tx_type));

    // Note when assigning txfm_size_col, we use the txfm_size from the
    // row configuration and vice versa. This is intentionally done to
    // accurately perform rectangular transforms. When the transform is
    // rectangular, the number of columns will be the same as the
    // txfm_size stored in the row cfg struct. It will make no difference
    // for square transforms.
    let txfm_size_col = tx_size.width();
    let txfm_size_row = tx_size.height();

    let mut buf = Aligned::<[MaybeUninit<i32>; 64 * 64]>::uninit_array();
    let buf = &mut buf.data[..txfm_size_col * txfm_size_row];

    let cfg = Txfm2DFlipCfg::fwd(tx_type, tx_size, bd);

    let txfm_func_col = get_func(cfg.txfm_type_col);
    let txfm_func_row = get_func(cfg.txfm_type_row);

    // Columns
    for c in 0..txfm_size_col {
      let mut col_coeffs = Aligned::<[MaybeUninit<i32>; 64]>::uninit_array();
      let col_coeffs = &mut col_coeffs.data[..txfm_size_row];
      if cfg.ud_flip {
        // flip upside down
        for r in 0..txfm_size_row {
          col_coeffs[r]
            .write((input[(txfm_size_row - r - 1) * stride + c]).into());
        }
      } else {
        for r in 0..txfm_size_row {
          col_coeffs[r].write((input[r * stride + c]).into());
        }
      }
      // SAFETY: The loops above have initialized all txfm_size_row elements
      let col_coeffs = unsafe { slice_assume_init_mut(col_coeffs) };

      av1_round_shift_array(col_coeffs, txfm_size_row, -cfg.shift[0]);
      txfm_func_col(col_coeffs);
      av1_round_shift_array(col_coeffs, txfm_size_row, -cfg.shift[1]);
      if cfg.lr_flip {
        for r in 0..txfm_size_row {
          // flip from left to right
          buf[r * txfm_size_col + (txfm_size_col - c - 1)]
            .write(col_coeffs[r]);
        }
      } else {
        for r in 0..txfm_size_row {
          buf[r * txfm_size_col + c].write(col_coeffs[r]);
        }
      }
    }
    // SAFETY: The loops above have initialized the entire buf
    let buf = unsafe { slice_assume_init_mut(buf) };

    // Rows
    for (r, row_coeffs) in buf.chunks_exact_mut(txfm_size_col).enumerate() {
      txfm_func_row(row_coeffs);
      av1_round_shift_array(row_coeffs, txfm_size_col, -cfg.shift[2]);

      // Store output in at most 32x32 chunks so that the first 32x32
      // coefficients are stored first. When we don't have 64 rows, there is no
      // change in order. With 64 rows, the chunks are in this order
      //  - First 32 rows and first 32 cols
      //  - Last 32 rows and first 32 cols
      //  - First 32 rows and last 32 cols
      //  - Last 32 rows and last 32 cols

      // Output is grouped into 32x32 chunks so a stride of at most 32 is
      // used for each chunk.
      let output_stride = txfm_size_row.min(32);

      // Split the first 32 rows from the last 32 rows
      let output = &mut output
        [(r >= 32) as usize * output_stride * txfm_size_col.min(32)..];

      for cg in (0..txfm_size_col).step_by(32) {
        // Split the first 32 cols from the last 32 cols
        let output = &mut output[txfm_size_row * cg..];

        for c in 0..txfm_size_col.min(32) {
          output[c * output_stride + (r & 31)]
            .write(T::cast_from(row_coeffs[c + cg]));
        }
      }
    }
  }
}
