// Copyright (c) 2019-2023, The rav1e contributors. All rights reserved
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

use core::arch::aarch64::*;

#[derive(Clone, Copy)]
#[repr(transparent)]
struct I32X8(int32x4x2_t);

impl I32X8 {
  #[inline]
  const unsafe fn vec(self) -> int32x4x2_t {
    self.0
  }

  #[inline]
  const unsafe fn raw(a: int32x4x2_t) -> I32X8 {
    Self(a)
  }

  #[inline]
  const unsafe fn new(a: int32x4_t, b: int32x4_t) -> I32X8 {
    Self(int32x4x2_t(a, b))
  }
}

type TxfmFunc = unsafe fn(&mut [I32X8]);

impl_1d_tx!(allow(unused_attributes), unsafe);

impl TxOperations for I32X8 {
  #[inline]
  unsafe fn zero() -> Self {
    let zero = vdupq_n_s32(0);
    I32X8::new(zero, zero)
  }

  #[inline]
  unsafe fn tx_mul<const SHIFT: i32>(self, mul: i32) -> Self {
    I32X8::new(
      vrshrq_n_s32(vmulq_n_s32(self.vec().0, mul), SHIFT),
      vrshrq_n_s32(vmulq_n_s32(self.vec().1, mul), SHIFT),
    )
  }

  #[inline]
  unsafe fn rshift1(self) -> Self {
    I32X8::new(
      vhsubq_s32(
        self.vec().0,
        vreinterpretq_s32_u32(vcltzq_s32(self.vec().0)),
      ),
      vhsubq_s32(
        self.vec().1,
        vreinterpretq_s32_u32(vcltzq_s32(self.vec().1)),
      ),
    )
  }

  #[inline]
  unsafe fn add(self, b: Self) -> Self {
    I32X8::new(
      vaddq_s32(self.vec().0, b.vec().0),
      vaddq_s32(self.vec().1, b.vec().1),
    )
  }

  #[inline]
  unsafe fn sub(self, b: Self) -> Self {
    I32X8::new(
      vsubq_s32(self.vec().0, b.vec().0),
      vsubq_s32(self.vec().1, b.vec().1),
    )
  }

  #[inline]
  unsafe fn add_avg(self, b: Self) -> Self {
    I32X8::new(
      vhaddq_s32(self.vec().0, b.vec().0),
      vhaddq_s32(self.vec().1, b.vec().1),
    )
  }

  #[inline]
  unsafe fn sub_avg(self, b: Self) -> Self {
    I32X8::new(
      vhsubq_s32(self.vec().0, b.vec().0),
      vhsubq_s32(self.vec().1, b.vec().1),
    )
  }
}

#[inline]
unsafe fn vreinterpretq_u32_s32_x2(a: int32x4x2_t) -> uint32x4x2_t {
  uint32x4x2_t(vreinterpretq_u32_s32(a.0), vreinterpretq_u32_s32(a.1))
}

#[inline]
unsafe fn vtrnq_s64_to_s32(a: int32x4_t, b: int32x4_t) -> int32x4x2_t {
  let a = vreinterpretq_s64_s32(a);
  let b = vreinterpretq_s64_s32(b);
  int32x4x2_t(
    vreinterpretq_s32_s64(vtrn1q_s64(a, b)),
    vreinterpretq_s32_s64(vtrn2q_s64(a, b)),
  )
}

#[inline]
unsafe fn transpose_8x8_neon(
  input: &[I32X8; 8], into: &mut [MaybeUninit<I32X8>; 8],
) {
  let stage1 = (
    vtrnq_s32(input[0].vec().0, input[1].vec().0),
    vtrnq_s32(input[2].vec().0, input[3].vec().0),
    vtrnq_s32(input[4].vec().0, input[5].vec().0),
    vtrnq_s32(input[6].vec().0, input[7].vec().0),
    vtrnq_s32(input[0].vec().1, input[1].vec().1),
    vtrnq_s32(input[2].vec().1, input[3].vec().1),
    vtrnq_s32(input[4].vec().1, input[5].vec().1),
    vtrnq_s32(input[6].vec().1, input[7].vec().1),
  );
  let stage2 = (
    vtrnq_s64_to_s32(stage1.0 .0, stage1.1 .0),
    vtrnq_s64_to_s32(stage1.0 .1, stage1.1 .1),
    vtrnq_s64_to_s32(stage1.2 .0, stage1.3 .0),
    vtrnq_s64_to_s32(stage1.2 .1, stage1.3 .1),
    vtrnq_s64_to_s32(stage1.4 .0, stage1.5 .0),
    vtrnq_s64_to_s32(stage1.4 .1, stage1.5 .1),
    vtrnq_s64_to_s32(stage1.6 .0, stage1.7 .0),
    vtrnq_s64_to_s32(stage1.6 .1, stage1.7 .1),
  );
  into[0].write(I32X8::new(stage2.0 .0, stage2.2 .0));
  into[1].write(I32X8::new(stage2.1 .0, stage2.3 .0));
  into[2].write(I32X8::new(stage2.0 .1, stage2.2 .1));
  into[3].write(I32X8::new(stage2.1 .1, stage2.3 .1));
  into[4].write(I32X8::new(stage2.4 .0, stage2.6 .0));
  into[5].write(I32X8::new(stage2.5 .0, stage2.7 .0));
  into[6].write(I32X8::new(stage2.4 .1, stage2.6 .1));
  into[7].write(I32X8::new(stage2.5 .1, stage2.7 .1));
}

#[inline]
unsafe fn transpose_8x4_neon(
  input: &[I32X8; 8], into: &mut [MaybeUninit<I32X8>; 4],
) {
  let stage1 = (
    vtrnq_s32(input[0].vec().0, input[1].vec().0),
    vtrnq_s32(input[2].vec().0, input[3].vec().0),
    vtrnq_s32(input[4].vec().0, input[5].vec().0),
    vtrnq_s32(input[6].vec().0, input[7].vec().0),
  );
  let stage2 = (
    vtrnq_s64_to_s32(stage1.0 .0, stage1.1 .0),
    vtrnq_s64_to_s32(stage1.0 .1, stage1.1 .1),
    vtrnq_s64_to_s32(stage1.2 .0, stage1.3 .0),
    vtrnq_s64_to_s32(stage1.2 .1, stage1.3 .1),
  );
  into[0].write(I32X8::new(stage2.0 .0, stage2.2 .0));
  into[1].write(I32X8::new(stage2.1 .0, stage2.3 .0));
  into[2].write(I32X8::new(stage2.0 .1, stage2.2 .1));
  into[3].write(I32X8::new(stage2.1 .1, stage2.3 .1));
}

#[inline]
unsafe fn transpose_4x8_neon(
  input: &[I32X8; 4], into: &mut [MaybeUninit<I32X8>; 8],
) {
  let stage1 = (
    vtrnq_s32(input[0].vec().0, input[1].vec().0),
    vtrnq_s32(input[2].vec().0, input[3].vec().0),
    vtrnq_s32(input[0].vec().1, input[1].vec().1),
    vtrnq_s32(input[2].vec().1, input[3].vec().1),
  );
  let stage2 = (
    vtrnq_s64_to_s32(stage1.0 .0, stage1.1 .0),
    vtrnq_s64_to_s32(stage1.0 .1, stage1.1 .1),
    vtrnq_s64_to_s32(stage1.2 .0, stage1.3 .0),
    vtrnq_s64_to_s32(stage1.2 .1, stage1.3 .1),
  );
  into[0].write(I32X8::raw(stage2.0));
  into[1].write(I32X8::raw(stage2.1));
  into[2].write(I32X8::new(stage2.0 .1, stage2.0 .0));
  into[3].write(I32X8::new(stage2.1 .1, stage2.1 .0));
  into[4].write(I32X8::raw(stage2.2));
  into[5].write(I32X8::raw(stage2.3));
  into[6].write(I32X8::new(stage2.2 .1, stage2.2 .0));
  into[7].write(I32X8::new(stage2.3 .1, stage2.3 .0));
}

#[inline]
unsafe fn transpose_4x4_neon(
  input: &[I32X8; 4], into: &mut [MaybeUninit<I32X8>; 4],
) {
  let stage1 = (
    vtrnq_s32(input[0].vec().0, input[1].vec().0),
    vtrnq_s32(input[2].vec().0, input[3].vec().0),
  );
  let stage2 = (
    vtrnq_s64_to_s32(stage1.0 .0, stage1.1 .0),
    vtrnq_s64_to_s32(stage1.0 .1, stage1.1 .1),
  );
  into[0].write(I32X8::raw(stage2.0));
  into[1].write(I32X8::raw(stage2.1));
  into[2].write(I32X8::new(stage2.0 .1, stage2.0 .0));
  into[3].write(I32X8::new(stage2.1 .1, stage2.1 .0));
}

#[inline]
unsafe fn shift_left_neon(a: I32X8, shift: u8) -> I32X8 {
  let shift = vdupq_n_s32(shift.into());
  I32X8::new(vrshlq_s32(a.vec().0, shift), vrshlq_s32(a.vec().1, shift))
}

#[inline]
unsafe fn shift_right_neon<const SHIFT: i32>(a: I32X8) -> I32X8 {
  I32X8::new(vrshrq_n_s32(a.vec().0, SHIFT), vrshrq_n_s32(a.vec().1, SHIFT))
}

#[inline]
unsafe fn round_shift_array_neon(arr: &mut [I32X8], bit: i8) {
  if arr.len() % 4 != 0 {
    debug_unreachable!();
  }

  if bit == 0 {
    return;
  }
  if bit > 0 {
    if bit == 1 {
      for s in arr.chunks_exact_mut(4) {
        for chunk in s {
          *chunk = shift_right_neon::<1>(*chunk)
        }
      }
    } else if bit == 2 {
      for s in arr.chunks_exact_mut(4) {
        for chunk in s {
          *chunk = shift_right_neon::<2>(*chunk)
        }
      }
    } else {
      debug_unreachable!();
    }
  } else {
    let shift = (-bit) as u8;
    for s in arr.chunks_exact_mut(4) {
      for chunk in s {
        *chunk = shift_left_neon(*chunk, shift);
      }
    }
  }
}

#[allow(clippy::identity_op, clippy::erasing_op)]
unsafe fn forward_transform_neon<T: Coefficient>(
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

  let mut buf = Aligned::new([MaybeUninit::<I32X8>::uninit(); 64 * 64 / 8]);
  let buf = &mut buf.data[..txfm_size_col * (txfm_size_row / 8).max(1)];
  let cfg = Txfm2DFlipCfg::fwd(tx_type, tx_size, bd);

  let txfm_func_col = get_func(cfg.txfm_type_col);
  let txfm_func_row = get_func(cfg.txfm_type_row);

  // Columns
  for cg in (0..txfm_size_col).step_by(8) {
    let shift = cfg.shift[0] as u8;
    #[inline]
    unsafe fn load_columns(input_ptr: *const i16, shift: u8) -> I32X8 {
      // TODO: load 64-bits for x4 wide columns
      let a = vld1q_s16(input_ptr);
      shift_left_neon(
        I32X8::new(vmovl_s16(vget_low_s16(a)), vmovl_high_s16(a)),
        shift,
      )
    }

    // Avoid zero initialization
    let col_coeffs =
      &mut [MaybeUninit::<I32X8>::uninit(); 64][..txfm_size_row];

    if cfg.ud_flip {
      // flip upside down
      for (in_slice, out_reg) in
        input[cg..].chunks(stride).zip(col_coeffs.iter_mut().rev())
      {
        *out_reg = MaybeUninit::new(load_columns(in_slice.as_ptr(), shift));
      }
    } else {
      for (in_slice, out_reg) in
        input[cg..].chunks(stride).zip(col_coeffs.iter_mut())
      {
        *out_reg = MaybeUninit::new(load_columns(in_slice.as_ptr(), shift));
      }
    }

    let col_coeffs = slice_assume_init_mut(col_coeffs);

    txfm_func_col(col_coeffs);
    round_shift_array_neon(col_coeffs, -cfg.shift[1]);

    // Transpose the array. Select the appropriate method to do so.
    match (row_class, col_class) {
      (SizeClass1D::X8UP, SizeClass1D::X8UP) => {
        for rg in (0..txfm_size_row).step_by(8) {
          let buf = &mut buf[(rg / 8 * txfm_size_col) + cg..];
          let buf = cast_mut::<8, _>(buf);
          let input = &col_coeffs[rg..];
          let input = cast::<8, _>(input);
          transpose_8x8_neon(input, buf);
        }
      }
      (SizeClass1D::X8UP, SizeClass1D::X4) => {
        for rg in (0..txfm_size_row).step_by(8) {
          let buf = &mut buf[(rg / 8 * txfm_size_col) + cg..];
          let buf = cast_mut::<4, _>(buf);
          let input = &col_coeffs[rg..];
          let input = cast::<8, _>(input);
          transpose_8x4_neon(input, buf);
        }
      }
      (SizeClass1D::X4, SizeClass1D::X8UP) => {
        // Don't need to loop over rows
        let buf = &mut buf[cg..];
        let buf = cast_mut::<8, _>(buf);
        let input = cast::<4, _>(col_coeffs);
        transpose_4x8_neon(input, buf);
      }
      (SizeClass1D::X4, SizeClass1D::X4) => {
        // Don't need to loop over rows
        let buf = cast_mut::<4, _>(buf);
        let input = cast::<4, _>(col_coeffs);
        transpose_4x4_neon(input, buf);
      }
    }
  }

  let buf = slice_assume_init_mut(buf);

  // Rows
  for rg in (0..txfm_size_row).step_by(8) {
    let row_coeffs = &mut buf[rg / 8 * txfm_size_col..][..txfm_size_col];

    if cfg.lr_flip {
      row_coeffs.reverse();
    }

    txfm_func_row(row_coeffs);
    round_shift_array_neon(row_coeffs, -cfg.shift[2]);

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
                vst1_u16_x2(
                  output[c * output_stride..].as_mut_ptr() as *mut _,
                  uint16x4x2_t(
                    vreinterpret_u16_s16(vmovn_s32(vec.0)),
                    vreinterpret_u16_s16(vmovn_s32(vec.1)),
                  ),
                );
              }
              PixelType::U16 => {
                vst1q_u32_x2(
                  output[c * output_stride..].as_mut_ptr() as *mut _,
                  vreinterpretq_u32_s32_x2(row_coeffs[c + cg].vec()),
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
              vst1_s16(
                output[c * txfm_size_row + rg..].as_mut_ptr() as *mut _,
                vmovn_s32(row_coeffs[c].vec().0),
              );
            }
            PixelType::U16 => {
              vst1q_s32(
                output[c * txfm_size_row + rg..].as_mut_ptr() as *mut _,
                row_coeffs[c].vec().0,
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
  if cpu >= CpuFeatureLevel::NEON {
    // SAFETY: Calls Assembly code.
    unsafe {
      forward_transform_neon(input, output, stride, tx_size, tx_type, bd);
    }
  } else {
    rust::forward_transform(input, output, stride, tx_size, tx_type, bd, cpu);
  }
}
