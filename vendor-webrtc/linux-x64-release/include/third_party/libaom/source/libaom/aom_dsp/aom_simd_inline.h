/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_DSP_AOM_SIMD_INLINE_H_
#define AOM_AOM_DSP_AOM_SIMD_INLINE_H_

#include "aom_dsp/aom_dsp_common.h"

#ifndef SIMD_INLINE
#define SIMD_INLINE static AOM_FORCE_INLINE
#endif

#define SIMD_CLAMP(value, min, max) \
  ((value) > (max) ? (max) : (value) < (min) ? (min) : (value))

#endif  // AOM_AOM_DSP_AOM_SIMD_INLINE_H_
