// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use std::mem::MaybeUninit;

pub fn init_slice_repeat_mut<T: Copy>(
  slice: &'_ mut [MaybeUninit<T>], value: T,
) -> &'_ mut [T] {
  // Fill all of slice
  for a in slice.iter_mut() {
    *a = MaybeUninit::new(value);
  }

  // SAFETY: Defined behavior, since all elements of slice are initialized
  unsafe { slice_assume_init_mut(slice) }
}

/// Assume all the elements are initialized.
#[inline(always)]
pub unsafe fn slice_assume_init_mut<T: Copy>(
  slice: &'_ mut [MaybeUninit<T>],
) -> &'_ mut [T] {
  &mut *(slice as *mut [MaybeUninit<T>] as *mut [T])
}
