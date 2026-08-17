// Copyright (c) 2019-2020, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

cfg_if::cfg_if! {
  if #[cfg(nasm_x86_64)] {
    #[macro_use]
    mod x86;
    pub use x86::*;
  } else if #[cfg(asm_neon)] {
    #[macro_use]
    mod aarch64;
    pub use aarch64::*;
  } else {
    mod rust;
    pub use rust::*;
  }
}
