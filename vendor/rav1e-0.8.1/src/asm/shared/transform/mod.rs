// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

pub mod forward;
pub mod inverse;

use crate::transform::*;
use std::ops::Index;

impl<T> Index<TxSize> for [T; TxSize::TX_SIZES_ALL] {
  type Output = T;
  #[inline]
  fn index(&self, tx_size: TxSize) -> &Self::Output {
    // SAFETY: values of TxSize are < TX_SIZES_ALL
    unsafe { self.get_unchecked(tx_size as usize) }
  }
}

impl<T> Index<TxType> for [T; TX_TYPES] {
  type Output = T;
  #[inline]
  fn index(&self, tx_type: TxType) -> &Self::Output {
    // SAFETY: Wraps WHT_WHT to DCT_DCT
    unsafe { self.get_unchecked((tx_type as usize) & 15) }
  }
}

impl<T> Index<TxType> for [T; TX_TYPES_PLUS_LL] {
  type Output = T;
  #[inline]
  fn index(&self, tx_type: TxType) -> &Self::Output {
    // SAFETY: values of TxType are < TX_TYPES_PLUS_LL
    unsafe { self.get_unchecked(tx_type as usize) }
  }
}
