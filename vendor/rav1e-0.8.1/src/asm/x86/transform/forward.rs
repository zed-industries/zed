// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::asm::shared::transform::forward::*;
use crate::cpu_features::CpuFeatureLevel;
use crate::transform::forward::rust;
use crate::transform::forward_shared::*;
use crate::transform::*;
use crate::util::*;
use std::mem::MaybeUninit;

use debug_unreachable::debug_unreachable;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[derive(Copy, Clone)]
struct I32X8 {
  data: __m256i,
}

impl I32X8 {
  #[target_feature(enable = "avx2")]
  #[inline]
  const unsafe fn vec(self) -> __m256i {
    self.data
  }

  #[target_feature(enable = "avx2")]
  #[inline]
  const unsafe fn new(a: __m256i) -> I32X8 {
    I32X8 { data: a }
  }
}

type TxfmFunc = unsafe fn(&mut [I32X8]);

impl_1d_tx!(target_feature(enable = "avx2"), unsafe);

impl TxOperations for I32X8 {
  #[target_feature(enable = "avx2")]
  #[inline]
  unsafe fn zero() -> Self {
    I32X8::new(_mm256_setzero_si256())
  }

  #[target_feature(enable = "avx2")]
  #[inline]
  unsafe fn tx_mul<const SHIFT: i32>(self, mul: i32) -> Self {
    I32X8::new(_mm256_srav_epi32(
      _mm256_add_epi32(
        _mm256_mullo_epi32(self.vec(), _mm256_set1_epi32(mul)),
        _mm256_set1_epi32(1 << SHIFT >> 1),
      ),
      _mm256_set1_epi32(SHIFT),
    ))
  }

  #[target_feature(enable = "avx2")]
  #[inline]
  unsafe fn rshift1(self) -> Self {
    I32X8::new(_mm256_srai_epi32(
      _mm256_sub_epi32(
        self.vec(),
        _mm256_cmpgt_epi32(_mm256_setzero_si256(), self.vec()),
      ),
      1,
    ))
  }

  #[target_feature(enable = "avx2")]
  #[inline]
  unsafe fn add(self, b: Self) -> Self {
    I32X8::new(_mm256_add_epi32(self.vec(), b.vec()))
  }

  #[target_feature(enable = "avx2")]
  #[inline]
  unsafe fn sub(self, b: Self) -> Self {
    I32X8::new(_mm256_sub_epi32(self.vec(), b.vec()))
  }

  #[target_feature(enable = "avx2")]
  #[inline]
  unsafe fn add_avg(self, b: Self) -> Self {
    I32X8::new(_mm256_srai_epi32(_mm256_add_epi32(self.vec(), b.vec()), 1))
  }

  #[target_feature(enable = "avx2")]
  #[inline]
  unsafe fn sub_avg(self, b: Self) -> Self {
    I32X8::new(_mm256_srai_epi32(_mm256_sub_epi32(self.vec(), b.vec()), 1))
  }
}

#[target_feature(enable = "avx2")]
unsafe fn transpose_8x8_avx2(input: &[I32X8; 8], into: &mut [I32X8; 8]) {
  let stage1 = (
    _mm256_unpacklo_epi32(input[0].vec(), input[1].vec()),
    _mm256_unpackhi_epi32(input[0].vec(), input[1].vec()),
    _mm256_unpacklo_epi32(input[2].vec(), input[3].vec()),
    _mm256_unpackhi_epi32(input[2].vec(), input[3].vec()),
    _mm256_unpacklo_epi32(input[4].vec(), input[5].vec()),
    _mm256_unpackhi_epi32(input[4].vec(), input[5].vec()),
    _mm256_unpacklo_epi32(input[6].vec(), input[7].vec()),
    _mm256_unpackhi_epi32(input[6].vec(), input[7].vec()),
  );

  let stage2 = (
    _mm256_unpacklo_epi64(stage1.0, stage1.2),
    _mm256_unpackhi_epi64(stage1.0, stage1.2),
    _mm256_unpacklo_epi64(stage1.1, stage1.3),
    _mm256_unpackhi_epi64(stage1.1, stage1.3),
    _mm256_unpacklo_epi64(stage1.4, stage1.6),
    _mm256_unpackhi_epi64(stage1.4, stage1.6),
    _mm256_unpacklo_epi64(stage1.5, stage1.7),
    _mm256_unpackhi_epi64(stage1.5, stage1.7),
  );

  #[allow(clippy::identity_op)]
  const LO: i32 = (2 << 4) | 0;
  const HI: i32 = (3 << 4) | 1;
  into[0] = I32X8::new(_mm256_permute2x128_si256(stage2.0, stage2.4, LO));
  into[1] = I32X8::new(_mm256_permute2x128_si256(stage2.1, stage2.5, LO));
  into[2] = I32X8::new(_mm256_permute2x128_si256(stage2.2, stage2.6, LO));
  into[3] = I32X8::new(_mm256_permute2x128_si256(stage2.3, stage2.7, LO));
  into[4] = I32X8::new(_mm256_permute2x128_si256(stage2.0, stage2.4, HI));
  into[5] = I32X8::new(_mm256_permute2x128_si256(stage2.1, stage2.5, HI));
  into[6] = I32X8::new(_mm256_permute2x128_si256(stage2.2, stage2.6, HI));
  into[7] = I32X8::new(_mm256_permute2x128_si256(stage2.3, stage2.7, HI));
}

#[target_feature(enable = "avx2")]
unsafe fn transpose_8x4_avx2(input: &[I32X8; 8], into: &mut [I32X8; 4]) {
  // Last 8 are empty
  let stage1 = (
    //0101
    _mm256_unpacklo_epi32(input[0].vec(), input[1].vec()),
    _mm256_unpackhi_epi32(input[0].vec(), input[1].vec()),
    _mm256_unpacklo_epi32(input[2].vec(), input[3].vec()),
    _mm256_unpackhi_epi32(input[2].vec(), input[3].vec()),
    _mm256_unpacklo_epi32(input[4].vec(), input[5].vec()),
    _mm256_unpackhi_epi32(input[4].vec(), input[5].vec()),
    _mm256_unpacklo_epi32(input[6].vec(), input[7].vec()),
    _mm256_unpackhi_epi32(input[6].vec(), input[7].vec()),
  );

  let stage2 = (
    _mm256_unpacklo_epi64(stage1.0, stage1.2),
    _mm256_unpackhi_epi64(stage1.0, stage1.2),
    _mm256_unpacklo_epi64(stage1.1, stage1.3),
    _mm256_unpackhi_epi64(stage1.1, stage1.3),
    _mm256_unpacklo_epi64(stage1.4, stage1.6),
    _mm256_unpackhi_epi64(stage1.4, stage1.6),
    _mm256_unpacklo_epi64(stage1.5, stage1.7),
    _mm256_unpackhi_epi64(stage1.5, stage1.7),
  );

  #[allow(clippy::identity_op)]
  const LO: i32 = (2 << 4) | 0;
  into[0] = I32X8::new(_mm256_permute2x128_si256(stage2.0, stage2.4, LO));
  into[1] = I32X8::new(_mm256_permute2x128_si256(stage2.1, stage2.5, LO));
  into[2] = I32X8::new(_mm256_permute2x128_si256(stage2.2, stage2.6, LO));
  into[3] = I32X8::new(_mm256_permute2x128_si256(stage2.3, stage2.7, LO));
}

#[target_feature(enable = "avx2")]
unsafe fn transpose_4x8_avx2(input: &[I32X8; 4], into: &mut [I32X8; 8]) {
  let stage1 = (
    // 0101
    _mm256_unpacklo_epi32(input[0].vec(), input[1].vec()),
    _mm256_unpackhi_epi32(input[0].vec(), input[1].vec()),
    // 2323
    _mm256_unpacklo_epi32(input[2].vec(), input[3].vec()),
    _mm256_unpackhi_epi32(input[2].vec(), input[3].vec()),
  );

  let stage2 = (
    // 01234567
    _mm256_unpacklo_epi64(stage1.0, stage1.2),
    _mm256_unpackhi_epi64(stage1.0, stage1.2),
    _mm256_unpacklo_epi64(stage1.1, stage1.3),
    _mm256_unpackhi_epi64(stage1.1, stage1.3),
  );

  into[0] = I32X8::new(stage2.0);
  into[1] = I32X8::new(stage2.1);
  into[2] = I32X8::new(stage2.2);
  into[3] = I32X8::new(stage2.3);
  into[4] =
    I32X8::new(_mm256_castsi128_si256(_mm256_extractf128_si256(stage2.0, 1)));
  into[5] =
    I32X8::new(_mm256_castsi128_si256(_mm256_extractf128_si256(stage2.1, 1)));
  into[6] =
    I32X8::new(_mm256_castsi128_si256(_mm256_extractf128_si256(stage2.2, 1)));
  into[7] =
    I32X8::new(_mm256_castsi128_si256(_mm256_extractf128_si256(stage2.3, 1)));
}

#[target_feature(enable = "avx2")]
unsafe fn transpose_4x4_avx2(input: &[I32X8; 4], into: &mut [I32X8; 4]) {
  let stage1 = (
    _mm256_unpacklo_epi32(input[0].vec(), input[1].vec()),
    _mm256_unpackhi_epi32(input[0].vec(), input[1].vec()),
    _mm256_unpacklo_epi32(input[2].vec(), input[3].vec()),
    _mm256_unpackhi_epi32(input[2].vec(), input[3].vec()),
  );

  into[0] = I32X8::new(_mm256_unpacklo_epi64(stage1.0, stage1.2));
  into[1] = I32X8::new(_mm256_unpackhi_epi64(stage1.0, stage1.2));
  into[2] = I32X8::new(_mm256_unpacklo_epi64(stage1.1, stage1.3));
  into[3] = I32X8::new(_mm256_unpackhi_epi64(stage1.1, stage1.3));
}

#[target_feature(enable = "avx2")]
#[inline]
unsafe fn shift_left(a: I32X8, shift: u8) -> I32X8 {
  I32X8::new(_mm256_sllv_epi32(a.vec(), _mm256_set1_epi32(shift as i32)))
}

#[target_feature(enable = "avx2")]
#[inline]
unsafe fn shift_right(a: I32X8, shift: u8) -> I32X8 {
  I32X8::new(_mm256_srav_epi32(
    _mm256_add_epi32(a.vec(), _mm256_set1_epi32(1 << (shift as i32) >> 1)),
    _mm256_set1_epi32(shift as i32),
  ))
}

#[target_feature(enable = "avx2")]
#[inline]
unsafe fn round_shift_array_avx2(arr: &mut [I32X8], bit: i8) {
  if arr.len() % 4 != 0 {
    debug_unreachable!();
  }

  if bit == 0 {
    return;
  }
  if bit > 0 {
    let shift = bit as u8;
    for s in arr.chunks_exact_mut(4) {
      for chunk in s {
        *chunk = shift_right(*chunk, shift);
      }
    }
  } else {
    let shift = (-bit) as u8;
    for s in arr.chunks_exact_mut(4) {
      for chunk in s {
        *chunk = shift_left(*chunk, shift);
      }
    }
  }
}

#[allow(clippy::identity_op, clippy::erasing_op)]
#[target_feature(enable = "avx2")]
unsafe fn forward_transform_avx2<T: Coefficient>(
  input: &[i16], output: &mut [MaybeUninit<T>], stride: usize,
  tx_size: TxSize, tx_type: TxType, bd: usize,
) {
  // Note when assigning txfm_size_col, we use the txfm_size from the
  // row configuration and vice versa. This is intentionally done to
  // accurately perform rectangular transforms. When the transform is
  // rectangular, the number of columns will be the same as the
  // txfm_size stored in the row cfg struct. It will make no difference
  // for square transforms.
  let txfm_size_col = tx_size.width();
  let txfm_size_row = tx_size.height();

  let col_class = SizeClass1D::from_length(txfm_size_col);
  let row_class = SizeClass1D::from_length(txfm_size_row);

  let mut tmp: Aligned<[I32X8; 64 * 64 / 8]> = Aligned::uninitialized();
  let buf = &mut tmp.data[..txfm_size_col * (txfm_size_row / 8).max(1)];
  let cfg = Txfm2DFlipCfg::fwd(tx_type, tx_size, bd);

  let txfm_func_col = get_func(cfg.txfm_type_col);
  let txfm_func_row = get_func(cfg.txfm_type_row);

  // Columns
  for cg in (0..txfm_size_col).step_by(8) {
    let shift = cfg.shift[0] as u8;
    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn load_columns(input_ptr: *const i16, shift: u8) -> I32X8 {
      // TODO: load 64-bits for x4 wide columns
      shift_left(
        I32X8::new(_mm256_cvtepi16_epi32(_mm_loadu_si128(
          input_ptr as *const _,
        ))),
        shift,
      )
    }

    // Avoid zero initialization
    let tx_in = &mut [MaybeUninit::<I32X8>::uninit(); 64][..txfm_size_row];

    if cfg.ud_flip {
      // flip upside down
      for (in_slice, out_reg) in
        input[cg..].chunks(stride).zip(tx_in.iter_mut().rev())
      {
        *out_reg = MaybeUninit::new(load_columns(in_slice.as_ptr(), shift));
      }
    } else {
      for (in_slice, out_reg) in
        input[cg..].chunks(stride).zip(tx_in.iter_mut())
      {
        *out_reg = MaybeUninit::new(load_columns(in_slice.as_ptr(), shift));
      }
    }

    let col_coeffs = slice_assume_init_mut(tx_in);

    txfm_func_col(col_coeffs);
    round_shift_array_avx2(col_coeffs, -cfg.shift[1]);

    // Transpose the array. Select the appropriate method to do so.
    match (row_class, col_class) {
      (SizeClass1D::X8UP, SizeClass1D::X8UP) => {
        for rg in (0..txfm_size_row).step_by(8) {
          let buf = &mut buf[(rg / 8 * txfm_size_col) + cg..];
          let buf = cast_mut::<8, _>(buf);
          let input = &col_coeffs[rg..];
          let input = cast::<8, _>(input);
          transpose_8x8_avx2(input, buf);
        }
      }
      (SizeClass1D::X8UP, SizeClass1D::X4) => {
        for rg in (0..txfm_size_row).step_by(8) {
          let buf = &mut buf[(rg / 8 * txfm_size_col) + cg..];
          let buf = cast_mut::<4, _>(buf);
          let input = &col_coeffs[rg..];
          let input = cast::<8, _>(input);
          transpose_8x4_avx2(input, buf);
        }
      }
      (SizeClass1D::X4, SizeClass1D::X8UP) => {
        // Don't need to loop over rows
        let buf = &mut buf[cg..];
        let buf = cast_mut::<8, _>(buf);
        let input = cast::<4, _>(col_coeffs);
        transpose_4x8_avx2(input, buf);
      }
      (SizeClass1D::X4, SizeClass1D::X4) => {
        // Don't need to loop over rows
        let buf = cast_mut::<4, _>(buf);
        let input = cast::<4, _>(col_coeffs);
        transpose_4x4_avx2(input, buf);
      }
    }
  }

  // Rows
  for rg in (0..txfm_size_row).step_by(8) {
    let row_coeffs = &mut buf[rg / 8 * txfm_size_col..][..txfm_size_col];

    if cfg.lr_flip {
      row_coeffs.reverse();
    }

    txfm_func_row(row_coeffs);
    round_shift_array_avx2(row_coeffs, -cfg.shift[2]);

    // Write out the coefficients using the correct method for transforms of
    // this size.
    match row_class {
      SizeClass1D::X8UP => {
        // Store output in at most 32x32 chunks. See rust code for details.

        // Output is grouped into 32x32 chunks so a stride of at most 32 is
        // used for each chunk
        let output_stride = txfm_size_row.min(32);

        // Split the first 32 rows from the last 32 rows and offset by rg % 32
        let output = &mut output[(rg & 31)
          + (rg >= 32) as usize * output_stride * txfm_size_col.min(32)..];

        for cg in (0..txfm_size_col).step_by(32) {
          // Offset by zero or half of output
          let output = &mut output[txfm_size_row * cg..];

          for c in 0..txfm_size_col.min(32) {
            match T::Pixel::type_enum() {
              PixelType::U8 => {
                let vec = row_coeffs[c + cg].vec();
                let lo = _mm256_castsi256_si128(vec);
                let hi = _mm256_extracti128_si256(vec, 1);
                _mm_storeu_si128(
                  output[c * output_stride..].as_mut_ptr() as *mut _,
                  _mm_packs_epi32(lo, hi),
                );
              }
              PixelType::U16 => {
                _mm256_storeu_si256(
                  output[c * output_stride..].as_mut_ptr() as *mut _,
                  row_coeffs[c + cg].vec(),
                );
              }
            }
          }
        }
      }
      SizeClass1D::X4 => {
        // Write out coefficients in normal order - it isn't possible to have
        // more than 32 rows.
        for c in 0..txfm_size_col {
          match T::Pixel::type_enum() {
            PixelType::U8 => {
              let lo = _mm256_castsi256_si128(row_coeffs[c].vec());
              _mm_storel_epi64(
                output[c * txfm_size_row + rg..].as_mut_ptr() as *mut _,
                _mm_packs_epi32(lo, lo),
              );
            }
            PixelType::U16 => {
              _mm256_storeu_si256(
                output[c * txfm_size_row + rg..].as_mut_ptr() as *mut _,
                row_coeffs[c].vec(),
              );
            }
          }
        }
      }
    }
  }
}

/// # Panics
///
/// - If called with an invalid combination of `tx_size` and `tx_type`
#[inline]
pub fn forward_transform<T: Coefficient>(
  input: &[i16], output: &mut [MaybeUninit<T>], stride: usize,
  tx_size: TxSize, tx_type: TxType, bd: usize, cpu: CpuFeatureLevel,
) {
  assert!(valid_av1_transform(tx_size, tx_type));
  if cpu >= CpuFeatureLevel::AVX2 {
    // SAFETY: Calls Assembly code.
    unsafe {
      forward_transform_avx2(input, output, stride, tx_size, tx_type, bd);
    }
  } else {
    rust::forward_transform(input, output, stride, tx_size, tx_type, bd, cpu);
  }
}
