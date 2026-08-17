/*
 * Copyright (c) 2025, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_PORTS_RISCV_H_
#define AOM_AOM_PORTS_RISCV_H_
#include <stdlib.h>

#include "config/aom_config.h"

#ifdef __cplusplus
extern "C" {
#endif

#define HAS_RVV 0x01

int riscv_simd_caps(void);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AOM_PORTS_RISCV_H_
