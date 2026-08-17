// Copyright (c) 2018-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

pub static AV1_LEVEL_DEFINED: [bool; 32] = [
  true, // 2.0
  true, // 2.1
  false, false, true, // 3.0
  true, // 3.1
  false, false, true, // 4.0
  true, // 4.1
  false, false, true, // 5.0
  true, // 5.1
  true, // 5.2
  true, // 5.3
  true, // 6.0
  true, // 6.1
  true, // 6.2
  true, // 6.3
  false, false, false, false, false, false, false, false, false, false, false,
  false,
];

pub static AV1_LEVEL_MAX_PIC_SIZE: [usize; 32] = [
  147456, // 2.0
  278784, // 2.1
  0, 0, 665856,  // 3.0
  1065024, // 3.1
  0, 0, 2359296,  // 4.0
  23592960, // 4.1
  0, 0, 8912896,  // 5.0
  8912896,  // 5.1
  8912896,  // 5.2
  8912896,  // 5.3
  35651584, // 6.0
  35651584, // 6.1
  35651584, // 6.2
  35651584, // 6.3
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub static AV1_LEVEL_MAX_H_SIZE: [usize; 32] = [
  2048, // 2.0
  2816, // 2.1
  0, 0, 4352, // 3.0
  5504, // 3.1
  0, 0, 6144, // 4.0
  6144, // 4.1
  0, 0, 8192,  // 5.0
  8192,  // 5.1
  8192,  // 5.2
  8192,  // 5.3
  16384, // 6.0
  16384, // 6.1
  16384, // 6.2
  16384, // 6.3
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub static AV1_LEVEL_MAX_V_SIZE: [usize; 32] = [
  1152, // 2.0
  1584, // 2.1
  0, 0, 2448, // 3.0
  3096, // 3.1
  0, 0, 3456, // 4.0
  3456, // 4.1
  0, 0, 4352, // 5.0
  4352, // 5.1
  4352, // 5.2
  4352, // 5.3
  8704, // 6.0
  8704, // 6.1
  8704, // 6.2
  8704, // 6.3
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub static AV1_LEVEL_MAX_DISPLAY_RATE: [usize; 32] = [
  4_423_680, // 2.0
  8_363_520, // 2.1
  0,
  0,
  19_975_680, // 3.0
  31_950_720, // 3.1
  0,
  0,
  70_778_880,  // 4.0
  141_557_760, // 4.1
  0,
  0,
  267_386_880,   // 5.0
  534_773_760,   // 5.1
  1_069_547_520, // 5.2
  1_069_547_520, // 5.3
  1_069_547_520, // 6.0
  2_139_095_040, // 6.1
  4_278_190_080, // 6.2
  4_278_190_080, // 6.3
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
];
