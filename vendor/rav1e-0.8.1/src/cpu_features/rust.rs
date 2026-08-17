// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use arg_enum_proc_macro::ArgEnum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, ArgEnum, Default)]
pub enum CpuFeatureLevel {
  #[default]
  RUST,
}

impl CpuFeatureLevel {
  #[cfg(test)]
  #[allow(unused)]
  pub(crate) const fn all() -> &'static [Self] {
    use CpuFeatureLevel::*;
    &[RUST]
  }
}
