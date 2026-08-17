// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::tiling::PlaneRegionMut;
use crate::util::*;
use std::mem::MaybeUninit;

// Note: Input coeffs are mutable since the assembly uses them as a scratchpad
pub type InvTxfmFunc =
  unsafe extern fn(*mut u8, libc::ptrdiff_t, *mut i16, i32);

pub type InvTxfmHBDFunc =
  unsafe extern fn(*mut u16, libc::ptrdiff_t, *mut i16, i32, i32);

pub fn call_inverse_func<T: Pixel>(
  func: InvTxfmFunc, input: &[T::Coeff], output: &mut PlaneRegionMut<'_, T>,
  eob: u16, width: usize, height: usize, bd: usize,
) {
  debug_assert!(bd == 8);

  // Only use at most 32 columns and 32 rows of input coefficients.
  let input: &[T::Coeff] = &input[..width.min(32) * height.min(32)];

  let mut copied = Aligned::<[MaybeUninit<T::Coeff>; 32 * 32]>::uninit_array();

  // Convert input to 16-bits.
  // TODO: Remove by changing inverse assembly to not overwrite its input
  for (a, b) in copied.data.iter_mut().zip(input) {
    a.write(*b);
  }

  // perform the inverse transform
  // SAFETY: Calls Assembly code.
  unsafe {
    func(
      output.data_ptr_mut() as *mut _,
      output.plane_cfg.stride as isize,
      copied.data.as_mut_ptr() as *mut _,
      eob as i32 - 1,
    );
  }
}

pub fn call_inverse_hbd_func<T: Pixel>(
  func: InvTxfmHBDFunc, input: &[T::Coeff],
  output: &mut PlaneRegionMut<'_, T>, eob: u16, width: usize, height: usize,
  bd: usize,
) {
  // Only use at most 32 columns and 32 rows of input coefficients.
  let input: &[T::Coeff] = &input[..width.min(32) * height.min(32)];

  let mut copied = Aligned::<[MaybeUninit<T::Coeff>; 32 * 32]>::uninit_array();

  // Convert input to 16-bits.
  // TODO: Remove by changing inverse assembly to not overwrite its input
  for (a, b) in copied.data.iter_mut().zip(input) {
    a.write(*b);
  }

  // perform the inverse transform
  // SAFETY: Calls Assembly code.
  unsafe {
    func(
      output.data_ptr_mut() as *mut _,
      T::to_asm_stride(output.plane_cfg.stride),
      copied.data.as_mut_ptr() as *mut _,
      eob as i32 - 1,
      (1 << bd) - 1,
    );
  }
}

#[cfg(test)]
pub mod test {
  use super::*;
  use crate::context::av1_get_coded_tx_size;
  use crate::cpu_features::CpuFeatureLevel;
  use crate::frame::{AsRegion, Plane};
  use crate::scan_order::av1_scan_orders;
  use crate::transform::TxSize::*;
  use crate::transform::*;
  use rand::{random, rng, Rng};

  pub fn pick_eob<T: Coefficient>(
    coeffs: &mut [T], tx_size: TxSize, tx_type: TxType, sub_h: usize,
  ) -> u16 {
    /* From dav1d
     * copy the topleft coefficients such that the return value (being the
     * coefficient scantable index for the eob token) guarantees that only
     * the topleft $sub out of $sz (where $sz >= $sub) coefficients in both
     * dimensions are non-zero. This leads to braching to specific optimized
     * simd versions (e.g. dc-only) so that we get full asm coverage in this
     * test */
    let coeff_h = av1_get_coded_tx_size(tx_size).height();
    let sub_high: usize = if sub_h > 0 { sub_h * 8 - 1 } else { 0 };
    let sub_low: usize = if sub_h > 1 { sub_high - 8 } else { 0 };
    let mut eob = 0u16;
    let mut exit = 0;

    let scan = av1_scan_orders[tx_size][tx_type].scan;

    for (i, &pos) in scan.iter().enumerate() {
      exit = i as u16;

      let rc = pos as usize;
      let rcx = rc % coeff_h;
      let rcy = rc / coeff_h;

      if rcx > sub_high || rcy > sub_high {
        break;
      } else if eob == 0 && (rcx > sub_low || rcy > sub_low) {
        eob = i as u16;
      }
    }

    if eob != 0 {
      eob += rng().random_range(0..(exit - eob).min(1));
    }
    for &pos in scan.iter().skip(usize::from(eob)) {
      coeffs[pos as usize] = T::cast_from(0);
    }

    eob + 1
  }

  pub fn test_transform<T: Pixel>(
    tx_size: TxSize, tx_type: TxType, bit_depth: usize, cpu: CpuFeatureLevel,
  ) {
    let sub_h_iterations: usize = match tx_size.height().max(tx_size.width()) {
      4 => 2,
      8 => 2,
      16 => 3,
      32 | 64 => 4,
      _ => unreachable!(),
    };

    for sub_h in 0..sub_h_iterations {
      let mut src_storage = [T::zero(); 64 * 64];
      let src = &mut src_storage[..tx_size.area()];
      let mut dst = Plane::from_slice(
        &[T::zero(); 64 * 64][..tx_size.area()],
        tx_size.width(),
      );
      let mut res = Aligned::<[MaybeUninit<i16>; 64 * 64]>::uninit_array();
      let res = &mut res.data[..tx_size.area()];
      let mut freq =
        Aligned::<[MaybeUninit<T::Coeff>; 64 * 64]>::uninit_array();
      let freq = &mut freq.data[..tx_size.area()];
      for ((r, s), d) in
        res.iter_mut().zip(src.iter_mut()).zip(dst.data.iter_mut())
      {
        *s = T::cast_from(random::<u16>() >> (16 - bit_depth));
        *d = T::cast_from(random::<u16>() >> (16 - bit_depth));
        r.write(i16::cast_from(*s) - i16::cast_from(*d));
      }
      // SAFETY: The loop just initialized res, and all three slices have the same length
      let res = unsafe { slice_assume_init_mut(res) };
      forward_transform(
        res,
        freq,
        tx_size.width(),
        tx_size,
        tx_type,
        bit_depth,
        CpuFeatureLevel::RUST,
      );
      // SAFETY: forward_transform initialized freq
      let freq = unsafe { slice_assume_init_mut(freq) };

      let eob: u16 = pick_eob(freq, tx_size, tx_type, sub_h);
      let mut rust_dst = dst.clone();

      inverse_transform_add(
        freq,
        &mut dst.as_region_mut(),
        eob,
        tx_size,
        tx_type,
        bit_depth,
        cpu,
      );
      inverse_transform_add(
        freq,
        &mut rust_dst.as_region_mut(),
        eob,
        tx_size,
        tx_type,
        bit_depth,
        CpuFeatureLevel::RUST,
      );
      assert_eq!(rust_dst.data_origin(), dst.data_origin());
    }
  }

  macro_rules! test_itx_fns {
    ([$([$(($ENUM:expr, $TYPE1:ident, $TYPE2:ident)),*]),*], $W:expr, $H:expr) => {
      paste::item! {
        $(
          $(
            #[test]
            fn [<inv_txfm2d_add_ $TYPE2 _$TYPE1 _$W x $H>]() {
              for &cpu in
                &CpuFeatureLevel::all()[..=CpuFeatureLevel::default().as_index()]
              {
                test_transform::<u8>([<TX_ $W X $H>], $ENUM, 8, cpu);
                test_transform::<u16>([<TX_ $W X $H>], $ENUM, 10, cpu);
                test_transform::<u16>([<TX_ $W X $H>], $ENUM, 12, cpu);
              }
            }
          )*
        )*
      }
    };

    ($TYPES_VALID:tt, [$(($W:expr, $H:expr)),*]) => {
      $(
        test_itx_fns!($TYPES_VALID, $W, $H);
      )*
    };

    ($TYPES64:tt, $DIMS64:tt, $TYPES32:tt, $DIMS32:tt, $TYPES16:tt, $DIMS16:tt,
     $TYPES84:tt, $DIMS84:tt, $TYPES4:tt, $DIMS4:tt) => {
      test_itx_fns!([$TYPES64], $DIMS64);
      test_itx_fns!([$TYPES64, $TYPES32], $DIMS32);
      test_itx_fns!([$TYPES64, $TYPES32, $TYPES16], $DIMS16);
      test_itx_fns!(
        [$TYPES64, $TYPES32, $TYPES16, $TYPES84], $DIMS84
      );
      test_itx_fns!(
        [$TYPES64, $TYPES32, $TYPES16, $TYPES84, $TYPES4], $DIMS4
      );
    };
  }

  test_itx_fns!(
    // 64x
    [(TxType::DCT_DCT, dct, dct)],
    [(64, 64), (64, 32), (32, 64), (16, 64), (64, 16)],
    // 32x
    [(TxType::IDTX, identity, identity)],
    [(32, 32), (32, 16), (16, 32), (32, 8), (8, 32)],
    // 16x16
    [
      (TxType::DCT_ADST, dct, adst),
      (TxType::ADST_DCT, adst, dct),
      (TxType::DCT_FLIPADST, dct, flipadst),
      (TxType::FLIPADST_DCT, flipadst, dct),
      (TxType::V_DCT, dct, identity),
      (TxType::H_DCT, identity, dct),
      (TxType::ADST_ADST, adst, adst),
      (TxType::ADST_FLIPADST, adst, flipadst),
      (TxType::FLIPADST_ADST, flipadst, adst),
      (TxType::FLIPADST_FLIPADST, flipadst, flipadst)
    ],
    [(16, 16)],
    // 8x, 4x and 16x (minus 16x16 and 4x4)
    [
      (TxType::V_ADST, adst, identity),
      (TxType::H_ADST, identity, adst),
      (TxType::V_FLIPADST, flipadst, identity),
      (TxType::H_FLIPADST, identity, flipadst)
    ],
    [(16, 8), (8, 16), (16, 4), (4, 16), (8, 8), (8, 4), (4, 8)],
    // 4x4
    [(TxType::WHT_WHT, wht, wht)],
    [(4, 4)]
  );
}
