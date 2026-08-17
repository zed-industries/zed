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

#ifndef AOM_AOM_DSP_AOM_SIMD_H_
#define AOM_AOM_DSP_AOM_SIMD_H_

#include <stdint.h>

#if defined(_WIN32)
#include <intrin.h>
#endif

#include "config/aom_config.h"

#include "aom_dsp/aom_simd_inline.h"

#define SIMD_CHECK 1  // Sanity checks in C equivalents

// VS compiling for 32 bit targets does not support vector types in
// structs as arguments, which makes the v256 type of the intrinsics
// hard to support, so optimizations for this target are disabled.
#if HAVE_SSE2 && (defined(_WIN64) || !defined(_MSC_VER) || defined(__clang__))
#include "simd/v256_intrinsics_x86.h"
#else
#include "simd/v256_intrinsics.h"
#endif

#endif  // AOM_AOM_DSP_AOM_SIMD_H_
