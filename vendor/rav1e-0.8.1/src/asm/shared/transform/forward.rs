// Copyright (c) 2019-2023, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

/// For classifying the number of rows and columns in a transform. Used to
/// select the operations to perform for different vector lengths.
#[derive(Debug, Clone, Copy)]
pub enum SizeClass1D {
  X4,
  X8UP,
}

impl SizeClass1D {
  #[inline]
  pub fn from_length(len: usize) -> Self {
    assert!(len.is_power_of_two());
    use SizeClass1D::*;
    match len {
      4 => X4,
      _ => X8UP,
    }
  }
}

pub fn cast<const N: usize, T>(x: &[T]) -> &[T; N] {
  // SAFETY: we perform a bounds check with [..N],
  // so casting to *const [T; N] is valid because the bounds
  // check guarantees that x has N elements
  unsafe { &*(&x[..N] as *const [T] as *const [T; N]) }
}

pub fn cast_mut<const N: usize, T>(x: &mut [T]) -> &mut [T; N] {
  // SAFETY: we perform a bounds check with [..N],
  // so casting to *mut [T; N] is valid because the bounds
  // check guarantees that x has N elements
  unsafe { &mut *(&mut x[..N] as *mut [T] as *mut [T; N]) }
}

#[cfg(test)]
mod test {
  use crate::cpu_features::*;
  use crate::transform::{forward_transform, get_valid_txfm_types, TxSize};
  use crate::util::slice_assume_init_mut;
  use rand::Rng;
  use std::mem::MaybeUninit;

  // Ensure that the simd results match the rust code
  #[test]
  fn test_forward_transform() {
    for &cpu in
      &CpuFeatureLevel::all()[1..=CpuFeatureLevel::default().as_index()]
    {
      println!("Testing {:?}", cpu);
      test_forward_transform_simd(cpu);
    }
  }

  fn test_forward_transform_simd(cpu: CpuFeatureLevel) {
    let mut rng = rand::rng();

    let tx_sizes = {
      use TxSize::*;
      [
        TX_4X4, TX_8X8, TX_16X16, TX_32X32, TX_64X64, TX_4X8, TX_8X4, TX_8X16,
        TX_16X8, TX_16X32, TX_32X16, TX_32X64, TX_64X32, TX_4X16, TX_16X4,
        TX_8X32, TX_32X8, TX_16X64, TX_64X16,
      ]
    };

    for &tx_size in &tx_sizes {
      let area = tx_size.area();

      let input: Vec<i16> =
        (0..area).map(|_| rng.random_range(-255..256)).collect();

      for &tx_type in get_valid_txfm_types(tx_size) {
        let mut output_ref = vec![MaybeUninit::new(0i16); area];
        let mut output_simd = vec![MaybeUninit::new(0i16); area];

        println!("Testing combination {:?}, {:?}", tx_size, tx_type);
        forward_transform(
          &input[..],
          &mut output_ref[..],
          tx_size.width(),
          tx_size,
          tx_type,
          8,
          CpuFeatureLevel::RUST,
        );
        let output_ref = unsafe { slice_assume_init_mut(&mut output_ref[..]) };
        forward_transform(
          &input[..],
          &mut output_simd[..],
          tx_size.width(),
          tx_size,
          tx_type,
          8,
          cpu,
        );
        let output_simd =
          unsafe { slice_assume_init_mut(&mut output_simd[..]) };
        assert_eq!(output_ref, output_simd)
      }
    }
  }
}
