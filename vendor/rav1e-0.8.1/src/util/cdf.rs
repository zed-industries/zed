// Copyright (c) 2017-2021, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

pub const fn cdf<const VARS: usize, const CDF_LEN: usize>(
  vars: [u16; VARS],
) -> [u16; CDF_LEN] {
  // Ensure that at least one zero is kept at the end
  assert!(CDF_LEN > VARS);

  let mut out = [0; CDF_LEN];
  let mut i = 0;
  while i < vars.len() {
    assert!(vars[i] <= 32768);
    out[i] = 32768 - vars[i];
    i += 1;
  }

  out
}

pub const fn cdf_2d<
  const VARS: usize,
  const CDF_LEN: usize,
  const N_2D: usize,
>(
  vars: [[u16; VARS]; N_2D],
) -> [[u16; CDF_LEN]; N_2D] {
  let mut out = [[0u16; CDF_LEN]; N_2D];
  let mut c = 0;
  while c < vars.len() {
    out[c] = cdf(vars[c]);
    c += 1;
  }

  out
}

pub const fn cdf_3d<
  const VARS: usize,
  const CDF_LEN: usize,
  const N_2D: usize,
  const N_3D: usize,
>(
  vars: [[[u16; VARS]; N_2D]; N_3D],
) -> [[[u16; CDF_LEN]; N_2D]; N_3D] {
  let mut out = [[[0u16; CDF_LEN]; N_2D]; N_3D];
  let mut c = 0;
  while c < vars.len() {
    out[c] = cdf_2d(vars[c]);
    c += 1;
  }

  out
}

pub const fn cdf_4d<
  const VARS: usize,
  const CDF_LEN: usize,
  const N_2D: usize,
  const N_3D: usize,
  const N_4D: usize,
>(
  vars: [[[[u16; VARS]; N_2D]; N_3D]; N_4D],
) -> [[[[u16; CDF_LEN]; N_2D]; N_3D]; N_4D] {
  let mut out = [[[[0u16; CDF_LEN]; N_2D]; N_3D]; N_4D];
  let mut c = 0;
  while c < vars.len() {
    out[c] = cdf_3d(vars[c]);
    c += 1;
  }

  out
}

pub const fn cdf_5d<
  const VARS: usize,
  const CDF_LEN: usize,
  const N_2D: usize,
  const N_3D: usize,
  const N_4D: usize,
  const N_5D: usize,
>(
  vars: [[[[[u16; VARS]; N_2D]; N_3D]; N_4D]; N_5D],
) -> [[[[[u16; CDF_LEN]; N_2D]; N_3D]; N_4D]; N_5D] {
  let mut out = [[[[[0u16; CDF_LEN]; N_2D]; N_3D]; N_4D]; N_5D];
  let mut c = 0;
  while c < vars.len() {
    out[c] = cdf_4d(vars[c]);
    c += 1;
  }

  out
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn cdf_len_ok() {
    let _: [u16; 5] = cdf([]);
    let _: [u16; 5] = cdf([1]);
    let _: [u16; 5] = cdf([1, 2, 3, 4]);
  }

  #[test]
  #[should_panic]
  fn cdf_len_panics() {
    let _: [u16; 5] = cdf([1, 2, 3, 4, 5]);
  }

  #[test]
  #[should_panic]
  fn cdf_val_panics() {
    let _: [u16; 5] = cdf([40000]);
  }

  #[test]
  fn cdf_vals_ok() {
    let cdf: [u16; 5] = cdf([2000, 10000, 32768, 0]);
    assert_eq!(cdf, [30768, 22768, 0, 32768, 0]);
  }

  #[test]
  fn cdf_5d_ok() {
    let cdf: [[[[[u16; 4]; 2]; 1]; 1]; 1] =
      cdf_5d([[[[[1000, 2000], [3000, 4000]]]]]);
    assert_eq!(cdf, [[[[[31768, 30768, 0, 0], [29768, 28768, 0, 0],]]]])
  }
}
