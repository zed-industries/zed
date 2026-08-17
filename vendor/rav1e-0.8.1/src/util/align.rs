// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use std::mem::MaybeUninit;

#[repr(align(64))]
pub struct Align64;

// A 64 byte aligned piece of data.
// # Examples
// ```
// let mut x: Aligned<[i16; 64 * 64]> = Aligned::new([0; 64 * 64]);
// assert!(x.data.as_ptr() as usize % 16 == 0);
//
// let mut x: Aligned<[i16; 64 * 64]> = Aligned::uninitialized();
// assert!(x.data.as_ptr() as usize % 16 == 0);
// ```
pub struct Aligned<T> {
  _alignment: [Align64; 0],
  pub data: T,
}

#[cfg(any(test, feature = "bench"))]
impl<const N: usize, T> Aligned<[T; N]> {
  #[inline(always)]
  pub fn from_fn<F>(cb: F) -> Self
  where
    F: FnMut(usize) -> T,
  {
    Aligned { _alignment: [], data: std::array::from_fn(cb) }
  }
}

impl<const N: usize, T> Aligned<[MaybeUninit<T>; N]> {
  #[inline(always)]
  pub const fn uninit_array() -> Self {
    Aligned {
      _alignment: [],
      // SAFETY: Uninitialized [MaybeUninit<T>; N] is valid.
      data: unsafe { MaybeUninit::uninit().assume_init() },
    }
  }
}

impl<T> Aligned<T> {
  pub const fn new(data: T) -> Self {
    Aligned { _alignment: [], data }
  }
  #[allow(clippy::uninit_assumed_init)]
  /// # Safety
  ///
  /// The resulting `Aligned<T>` *must* be written to before it is read from.
  pub const unsafe fn uninitialized() -> Self {
    Self::new(MaybeUninit::uninit().assume_init())
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn is_aligned<T>(ptr: *const T, n: usize) -> bool {
    ((ptr as usize) & ((1 << n) - 1)) == 0
  }

  #[test]
  fn sanity_stack() {
    let a: Aligned<_> = Aligned::new([0u8; 3]);
    assert!(is_aligned(a.data.as_ptr(), 4));
  }
}
