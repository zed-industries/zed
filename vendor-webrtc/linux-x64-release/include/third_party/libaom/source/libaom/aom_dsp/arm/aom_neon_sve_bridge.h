/*
 * Copyright (c) 2023, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_DSP_ARM_AOM_NEON_SVE_BRIDGE_H_
#define AOM_AOM_DSP_ARM_AOM_NEON_SVE_BRIDGE_H_

#include <arm_neon_sve_bridge.h>

#include "config/aom_dsp_rtcd.h"
#include "config/aom_config.h"

// We can access instructions exclusive to the SVE instruction set from a
// predominantly Neon context by making use of the Neon-SVE bridge intrinsics
// to reinterpret Neon vectors as SVE vectors - with the high part of the SVE
// vector (if it's longer than 128 bits) being "don't care".

// While sub-optimal on machines that have SVE vector length > 128-bit - as the
// remainder of the vector is unused - this approach is still beneficial when
// compared to a Neon-only solution.

static inline uint64x2_t aom_udotq_u16(uint64x2_t acc, uint16x8_t x,
                                       uint16x8_t y) {
  return svget_neonq_u64(svdot_u64(svset_neonq_u64(svundef_u64(), acc),
                                   svset_neonq_u16(svundef_u16(), x),
                                   svset_neonq_u16(svundef_u16(), y)));
}

static inline int64x2_t aom_sdotq_s16(int64x2_t acc, int16x8_t x, int16x8_t y) {
  return svget_neonq_s64(svdot_s64(svset_neonq_s64(svundef_s64(), acc),
                                   svset_neonq_s16(svundef_s16(), x),
                                   svset_neonq_s16(svundef_s16(), y)));
}

#define aom_svdot_lane_s16(sum, s0, f, lane)                          \
  svget_neonq_s64(svdot_lane_s64(svset_neonq_s64(svundef_s64(), sum), \
                                 svset_neonq_s16(svundef_s16(), s0),  \
                                 svset_neonq_s16(svundef_s16(), f), lane))

static inline uint16x8_t aom_tbl_u16(uint16x8_t s, uint16x8_t tbl) {
  return svget_neonq_u16(svtbl_u16(svset_neonq_u16(svundef_u16(), s),
                                   svset_neonq_u16(svundef_u16(), tbl)));
}

static inline int16x8_t aom_tbl_s16(int16x8_t s, uint16x8_t tbl) {
  return svget_neonq_s16(svtbl_s16(svset_neonq_s16(svundef_s16(), s),
                                   svset_neonq_u16(svundef_u16(), tbl)));
}

#endif  // AOM_AOM_DSP_ARM_AOM_NEON_SVE_BRIDGE_H_
